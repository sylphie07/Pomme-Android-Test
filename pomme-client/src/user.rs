use std::path::{Path, PathBuf};

use md5::Digest as _;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalProfile {
    pub id: uuid::Uuid,
    pub username: String,
}

impl LocalProfile {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            username: username.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LocalProfileDocument {
    profiles: Vec<LocalProfile>,
    selected_profile: uuid::Uuid,
}

/// Persistent, non-secret player identities. This file never contains
/// Microsoft access or refresh tokens; those remain in pomme-auth's store.
pub struct LocalProfileStore {
    path: PathBuf,
    profiles: Vec<LocalProfile>,
    selected_profile: uuid::Uuid,
}

impl LocalProfileStore {
    pub fn load_or_create(path: &Path) -> Self {
        if let Ok(json) = std::fs::read_to_string(path)
            && let Ok(document) = serde_json::from_str::<LocalProfileDocument>(&json)
            && !document.profiles.is_empty()
            && document
                .profiles
                .iter()
                .any(|profile| profile.id == document.selected_profile)
        {
            return Self {
                path: path.to_owned(),
                profiles: document.profiles,
                selected_profile: document.selected_profile,
            };
        }

        let profile = LocalProfile::new("Player");
        let store = Self {
            path: path.to_owned(),
            profiles: vec![profile.clone()],
            selected_profile: profile.id,
        };
        if let Err(error) = store.persist() {
            tracing::warn!("Could not persist the default local profile: {error}");
        }
        store
    }

    #[allow(dead_code)] // Used by the local-profile picker API and its persistence tests.
    pub fn profiles(&self) -> &[LocalProfile] {
        &self.profiles
    }

    pub fn selected(&self) -> &LocalProfile {
        self.profiles
            .iter()
            .find(|profile| profile.id == self.selected_profile)
            .unwrap_or(&self.profiles[0])
    }

    #[allow(dead_code)] // Used by the local-profile picker API and its persistence tests.
    pub fn create_profile(&mut self, username: &str) -> std::io::Result<LocalProfile> {
        let valid = !username.is_empty()
            && username.len() <= 16
            && username
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
        if !valid {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Minecraft profile names must be 1-16 ASCII letters, digits, or underscores",
            ));
        }

        let profile = LocalProfile::new(username);
        self.profiles.push(profile.clone());
        if let Err(error) = self.persist() {
            self.profiles.pop();
            return Err(error);
        }
        Ok(profile)
    }

    #[allow(dead_code)] // Used by the local-profile picker API and its persistence tests.
    pub fn select_profile(&mut self, id: uuid::Uuid) -> std::io::Result<()> {
        if !self.profiles.iter().any(|profile| profile.id == id) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "local profile does not exist",
            ));
        }
        let previous = self.selected_profile;
        self.selected_profile = id;
        if let Err(error) = self.persist() {
            self.selected_profile = previous;
            return Err(error);
        }
        Ok(())
    }

    fn persist(&self) -> std::io::Result<()> {
        let document = LocalProfileDocument {
            profiles: self.profiles.clone(),
            selected_profile: self.selected_profile,
        };
        let json = serde_json::to_vec_pretty(&document).map_err(std::io::Error::other)?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut temporary = self.path.as_os_str().to_owned();
        temporary.push(".part");
        let temporary = PathBuf::from(temporary);
        std::fs::write(&temporary, json)?;
        std::fs::rename(temporary, &self.path)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentitySource {
    LocalProfile,
    LauncherArguments,
    MicrosoftAccount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JavaAuthorization {
    Unverified,
    Verified { expires_at: u64 },
}

pub struct UserData {
    pub username: String,
    pub uuid: uuid::Uuid,
    pub access_token: Option<String>,
    /// True only when a real profile UUID was supplied or restored.
    pub has_profile: bool,
    pub local_profile: LocalProfile,
    pub identity_source: IdentitySource,
    pub java_authorization: JavaAuthorization,
}

impl UserData {
    pub fn from_args(
        username: Option<String>,
        uuid: Option<String>,
        access_token: Option<String>,
    ) -> Self {
        let name = username.unwrap_or_else(|| "Steve".to_owned());
        let local_profile = LocalProfile::new(name.clone());
        Self::from_launch_args(name, uuid, access_token, local_profile)
    }

    fn from_launch_args(
        username: String,
        uuid: Option<String>,
        access_token: Option<String>,
        local_profile: LocalProfile,
    ) -> Self {
        let uuid = uuid.and_then(|value| uuid::Uuid::parse_str(&value).ok());
        let has_profile = uuid.is_some();
        let uuid = uuid.unwrap_or_else(|| Self::offline_uuid(&username));
        Self {
            username,
            uuid,
            access_token,
            has_profile,
            local_profile,
            identity_source: IdentitySource::LauncherArguments,
            // Launcher arguments are identity inputs, not proof that the
            // account owns Java Edition. Release builds never trust them.
            java_authorization: JavaAuthorization::Unverified,
        }
    }

    /// Release builds start from the persisted local identity and restore any
    /// saved Microsoft account in the background. The integrated server is
    /// offline-mode, so local singleplayer never requires Java authorization;
    /// remote multiplayer always requires a verified session.
    pub fn for_launch(
        username: Option<String>,
        uuid: Option<String>,
        access_token: Option<String>,
        release: bool,
        mobile: bool,
        local_profile: LocalProfile,
    ) -> (Self, bool) {
        if release {
            return (Self::from_local_profile(local_profile), true);
        }
        let flagged = username.is_some() || uuid.is_some() || access_token.is_some();
        let user = if flagged {
            Self::from_launch_args(
                username.unwrap_or_else(|| local_profile.username.clone()),
                uuid,
                access_token,
                local_profile,
            )
        } else {
            Self::from_local_profile(local_profile)
        };
        (user, mobile && !flagged)
    }

    pub fn from_local_profile(profile: LocalProfile) -> Self {
        let username = profile.username.clone();
        Self {
            uuid: Self::offline_uuid(&username),
            username,
            access_token: None,
            has_profile: false,
            local_profile: profile,
            identity_source: IdentitySource::LocalProfile,
            java_authorization: JavaAuthorization::Unverified,
        }
    }

    pub fn from_account(account: &pomme_auth::AuthAccount, local_profile: LocalProfile) -> Self {
        let profile_uuid = uuid::Uuid::parse_str(&account.uuid).ok();
        let has_profile = profile_uuid.is_some();
        Self {
            username: account.username.clone(),
            uuid: profile_uuid.unwrap_or_else(|| Self::offline_uuid(&account.username)),
            access_token: Some(account.access_token.clone()),
            has_profile,
            local_profile,
            identity_source: IdentitySource::MicrosoftAccount,
            java_authorization: if has_profile {
                JavaAuthorization::Verified {
                    expires_at: account.expires_at,
                }
            } else {
                JavaAuthorization::Unverified
            },
        }
    }

    pub fn is_java_authorized(&self) -> bool {
        if self.identity_source != IdentitySource::MicrosoftAccount || !self.has_profile {
            return false;
        }
        let JavaAuthorization::Verified { expires_at } = self.java_authorization else {
            return false;
        };
        expires_at > unix_now() && self.access_token.is_some()
    }

    pub fn allows_singleplayer(&self, _release: bool) -> bool {
        // The embedded SteelMC server is intentionally offline-mode. A local
        // identity is sufficient here in every build configuration.
        true
    }

    pub fn allows_multiplayer(&self) -> bool {
        self.is_java_authorized()
    }

    /// Only a verified, unexpired Minecraft session may be sent to a remote
    /// server. A local/offline profile can never supply multiplayer auth.
    pub fn authenticated_access_token(&self) -> Option<&str> {
        self.is_java_authorized()
            .then(|| self.access_token.as_deref())
            .flatten()
    }

    fn offline_uuid(username: &str) -> uuid::Uuid {
        let digest = md5::Md5::digest(format!("OfflinePlayer:{username}").as_bytes());
        uuid::Builder::from_md5_bytes(digest.into()).into_uuid()
    }
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::{JavaAuthorization, LocalProfileStore, UserData};

    #[test]
    fn local_profiles_create_persist_reload_and_select() {
        let dir = std::env::temp_dir().join(format!("pomme-local-profiles-{}", std::process::id()));
        let path = dir.join("profiles.json");
        let mut store = LocalProfileStore::load_or_create(&path);
        let first = store.selected().clone();
        let second = store.create_profile("Alex_2").unwrap();
        assert_eq!(store.profiles().len(), 2);

        store.select_profile(second.id).unwrap();
        let reloaded = LocalProfileStore::load_or_create(&path);
        assert_eq!(reloaded.profiles().len(), 2);
        assert_eq!(reloaded.selected(), &second);
        assert_ne!(first.id, second.id);

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn local_profile_is_not_a_java_or_multiplayer_authorization() {
        let profile = super::LocalProfile::new("OfflineTester");
        let user = UserData::from_local_profile(profile);
        assert!(!user.has_profile);
        assert_eq!(user.authenticated_access_token(), None);
        assert!(user.allows_singleplayer(false));
        assert!(user.allows_singleplayer(true));
        assert!(!user.allows_multiplayer());
    }

    #[test]
    fn restored_unexpired_microsoft_session_grants_policy_permissions() {
        let account = pomme_auth::AuthAccount {
            username: "VerifiedPlayer".into(),
            uuid: "b50ad385-829d-3141-a216-7e7d7539ba7f".into(),
            access_token: "real-session-token".into(),
            expires_at: super::unix_now() + 3600,
        };
        let user = UserData::from_account(&account, super::LocalProfile::new("LocalPlayer"));
        assert_eq!(
            user.java_authorization,
            JavaAuthorization::Verified {
                expires_at: account.expires_at
            }
        );
        assert!(user.allows_singleplayer(true));
        assert!(user.allows_multiplayer());
        assert_eq!(
            user.authenticated_access_token(),
            Some("real-session-token")
        );
    }

    #[test]
    fn restored_account_from_store_authorizes_only_while_valid() {
        use pomme_auth::SecretStore;

        let dir =
            std::env::temp_dir().join(format!("pomme-restored-profile-{}", std::process::id()));
        let store = pomme_auth::FileStore {
            path: dir.join("accounts.json"),
        };
        let account = pomme_auth::AuthAccount {
            username: "RestoredPlayer".into(),
            uuid: "b50ad385-829d-3141-a216-7e7d7539ba7f".into(),
            access_token: "stored-valid-token".into(),
            expires_at: super::unix_now() + 3600,
        };
        store.write(
            "minecraft-accounts",
            &serde_json::to_string(&vec![account.clone()]).unwrap(),
        );

        let restored = pomme_auth::try_restore(&store, &account.uuid).unwrap();
        let user = UserData::from_account(&restored, super::LocalProfile::new("LocalPlayer"));
        assert!(user.allows_singleplayer(true));
        assert!(user.allows_multiplayer());
        assert_eq!(
            user.authenticated_access_token(),
            Some("stored-valid-token")
        );

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn launch_arguments_and_offline_uuid_are_not_authorization() {
        let user = UserData::from_args(
            Some("InjectedName".into()),
            Some("b50ad385-829d-3141-a216-7e7d7539ba7f".into()),
            Some("unverified-token".into()),
        );
        assert!(user.has_profile);
        assert!(!user.is_java_authorized());
        assert_eq!(user.authenticated_access_token(), None);
        assert!(!user.allows_multiplayer());
    }

    #[test]
    fn expired_session_blocks_multiplayer_but_keeps_local_singleplayer() {
        let account = pomme_auth::AuthAccount {
            username: "ExpiredPlayer".into(),
            uuid: "b50ad385-829d-3141-a216-7e7d7539ba7f".into(),
            access_token: "expired-token".into(),
            expires_at: 1,
        };
        let user = UserData::from_account(&account, super::LocalProfile::new("LocalPlayer"));
        assert!(user.allows_singleplayer(true));
        assert!(!user.allows_multiplayer());
        assert_eq!(user.authenticated_access_token(), None);
    }

    #[test]
    fn malformed_microsoft_profile_id_fails_closed() {
        let account = pomme_auth::AuthAccount {
            username: "MalformedProfile".into(),
            uuid: "not-a-uuid".into(),
            access_token: "token".into(),
            expires_at: super::unix_now() + 3600,
        };
        let user = UserData::from_account(&account, super::LocalProfile::new("LocalPlayer"));
        assert!(!user.has_profile);
        assert!(!user.is_java_authorized());
        assert_eq!(user.authenticated_access_token(), None);
    }

    #[test]
    fn offline_uuid_matches_vanilla() {
        for (name, expected) in [
            ("Notch", "b50ad385-829d-3141-a216-7e7d7539ba7f"),
            ("Steve", "5627dd98-e6be-3c21-b8a8-e92344183641"),
            ("Alex", "36532b5e-c442-3dbb-a24c-c7e55d0f979a"),
        ] {
            assert_eq!(UserData::offline_uuid(name).to_string(), expected);
        }
    }

    #[test]
    fn release_uses_local_identity_and_keeps_remote_account_gate() {
        let profile = super::LocalProfile::new("PersistentPlayer");
        let (user, required) = UserData::for_launch(None, None, None, true, true, profile.clone());
        assert!(required);
        assert_eq!(user.username, profile.username);
        assert!(user.allows_singleplayer(true));
        assert!(!user.allows_multiplayer());
    }

    #[test]
    fn local_profile_singleplayer_does_not_change_multiplayer_policy() {
        let profile = super::LocalProfile::new("LocalOnly");
        let (user, _) = UserData::for_launch(None, None, None, false, false, profile);
        assert!(user.allows_singleplayer(false));
        assert!(!user.allows_multiplayer());
    }
}
