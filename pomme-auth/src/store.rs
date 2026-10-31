//! Where accounts and refresh tokens are kept: string values by key.

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub trait SecretStore: Send + Sync {
    fn read(&self, key: &str) -> Option<String>;
    fn write(&self, key: &str, value: &str);
}

/// The OS credential store, under the launcher's service name so the
/// launcher and the desktop client see the same accounts.
#[cfg(feature = "keyring")]
pub struct KeyringStore;

#[cfg(feature = "keyring")]
const KEYRING_SERVICE: &str = "pomme-launcher";

#[cfg(feature = "keyring")]
impl SecretStore for KeyringStore {
    fn read(&self, key: &str) -> Option<String> {
        keyring::Entry::new(KEYRING_SERVICE, key)
            .ok()?
            .get_password()
            .ok()
    }

    fn write(&self, key: &str, value: &str) {
        if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, key) {
            let _ = entry.set_password(value);
        }
    }
}

/// A JSON file of every key, for platforms without an OS credential store
/// (Android, inside the app's private storage).
pub struct FileStore {
    pub path: PathBuf,
}

impl FileStore {
    /// A missing file is a new store. An unreadable or malformed existing
    /// file is an error: treating it as empty would let the next write replace
    /// the only copy of the user's saved accounts.
    fn entries(&self) -> std::io::Result<HashMap<String, String>> {
        let bytes = match std::fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return Ok(HashMap::new());
            }
            Err(error) => return Err(error),
        };

        serde_json::from_slice(&bytes)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))
    }
}

impl SecretStore for FileStore {
    fn read(&self, key: &str) -> Option<String> {
        self.entries().ok()?.remove(key)
    }

    fn write(&self, key: &str, value: &str) {
        // SecretStore's existing interface cannot return an error. Fail
        // closed here: most importantly, never replace an existing file that
        // could not be decoded. A later API revision can surface this error
        // to the UI instead of silently declining the write.
        let Ok(mut entries) = self.entries() else {
            return;
        };

        entries.insert(key.to_owned(), value.to_owned());
        let Ok(json) = serde_json::to_vec(&entries) else {
            return;
        };

        if let Some(dir) = self.path.parent() {
            if std::fs::create_dir_all(dir).is_err() {
                return;
            }
        }

        // Publish through a sibling temporary file so an interrupted write
        // cannot truncate the current account store.
        let mut tmp = self.path.as_os_str().to_owned();
        tmp.push(".part");
        let tmp = PathBuf::from(tmp);
        if std::fs::write(&tmp, json).is_err() {
            return;
        }
        if std::fs::rename(&tmp, &self.path).is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FileStore, SecretStore};

    #[test]
    fn file_store_keeps_each_key() {
        let dir = std::env::temp_dir().join(format!("pomme-auth-{}", std::process::id()));
        let store = FileStore {
            path: dir.join("accounts.json"),
        };
        assert_eq!(store.read("a"), None);
        store.write("a", "1");
        store.write("b", "2");
        store.write("a", "3");
        assert_eq!(store.read("a").as_deref(), Some("3"));
        assert_eq!(store.read("b").as_deref(), Some("2"));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn malformed_existing_store_is_never_overwritten_by_a_write() {
        let dir = std::env::temp_dir().join(format!(
            "pomme-auth-corrupt-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("accounts.json");
        let original = br#"{"minecraft-accounts":[{"username":"Existing"}"#;
        std::fs::write(&path, original).unwrap();

        let store = FileStore { path: path.clone() };
        assert_eq!(store.read("minecraft-accounts"), None);
        store.write("minecraft-accounts", "replacement");

        assert_eq!(std::fs::read(&path).unwrap(), original);
        assert!(!dir.join("accounts.json.part").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn missing_store_is_created_on_first_write() {
        let dir = std::env::temp_dir().join(format!(
            "pomme-auth-new-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let store = FileStore {
            path: dir.join("accounts.json"),
        };
        store.write("key", "value");
        assert_eq!(store.read("key").as_deref(), Some("value"));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
