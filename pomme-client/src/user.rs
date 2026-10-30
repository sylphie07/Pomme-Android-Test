use std::path::{Path, PathBuf};

use md5::Digest as _;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalProfile {
    /// Stable identifier for this profile record and its selection in the UI.
    pub id: uuid::Uuid,
    pub username: String,
    /// Stable player identity used for world data. It is deliberately
    /// independent from both the profile-record ID and the visible name.
    pub player_uuid: uuid::Uuid,
}

impl LocalProfile {
    pub fn new(username: impl Into<String>) -> Self {
        let username = username.into();
        let id = uuid::Uuid::new_v4();
        // Preserve the identity used by the current offline-mode server
        // until the persistent UUID is wired through in Phase 2.2-D.
        let player_uuid = UserData::offline_uuid(&username);
        Self {
            id,
            username,
            player_uuid,
        }
    }
}

const LOCAL_PROFILE_FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct LocalProfileDocument {
    #[serde(default)]
    format_version: Option<u32>,
    profiles: Vec<LocalProfileRecord>,
    selected_profile: uuid::Uuid,
}

#[derive(Serialize, Deserialize)]
struct LocalProfileRecord {
    id: uuid::Uuid,
    username: String,
    #[serde(default)]
    player_uuid: Option<uuid::Uuid>,
}

/// Persistent, non-secret player identities. This file never contains
/// Microsoft access or refresh tokens; those remain in pomme-auth's store.
pub struct LocalProfileStore {
    path: PathBuf,
    profiles: Vec<LocalProfile>,
    selected_profile: uuid::Uuid,
}

impl LocalProfileStore {
    /// Load a profile document without ever replacing unreadable or invalid
    /// user data. Legacy documents are migrated only after full validation.
    pub fn load_or_create(path: &Path) -> std::io::Result<Self> {
        if path.exists() {
            let json = std::fs::read_to_string(path)?;
            let (profiles, selected_profile, migrated) = Self::decode_document(&json)?;
            let store = Self {
                path: path.to_owned(),
                profiles,
                selected_profile,
            };
            if migrated {
                store.persist()?;
            }
            return Ok(store);
        }

        // A missing primary file with a backup indicates an interrupted
        // publication. Recover only a backup that passes the same validation.
        let backup = Self::backup_path(path);
        if backup.exists() {
            let json = std::fs::read_to_string(&backup)?;
            let (profiles, selected_profile, _) = Self::decode_document(&json)?;
            let store = Self {
                path: path.to_owned(),
                profiles,
                selected_profile,
            };
            store.persist()?;
            return Ok(store);
        }

        let profile = LocalProfile::new("Player");
        let store = Self {
            path: path.to_owned(),
            profiles: vec![profile.clone()],
            selected_profile: profile.id,
        };
        store.persist()?;
        Ok(store)
    }

    fn decode_document(json: &str) -> std::io::Result<(Vec<LocalProfile>, uuid::Uuid, bool)> {
        let raw: serde_json::Value =
            serde_json::from_str(json).map_err(std::io::Error::other)?;
        let has_format_version = raw
            .as_object()
            .is_some_and(|object| object.contains_key("format_version"));
        let has_player_uuid_field = raw
            .get("profiles")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|profiles| {
                profiles.iter().any(|profile| {
                    profile
                        .as_object()
                        .is_some_and(|object| object.contains_key("player_uuid"))
                })
            });
        let document: LocalProfileDocument =
            serde_json::from_value(raw).map_err(std::io::Error::other)?;

        let migrated = match document.format_version {
            None => {
                if has_format_version {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "local profile format_version must be a supported integer",
                    ));
                }
                if has_player_uuid_field {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "local profile document mixes legacy and versioned identity fields",
                    ));
                }
                true
            }
            Some(LOCAL_PROFILE_FORMAT_VERSION) => {
                if document.profiles.iter().any(|profile| profile.player_uuid.is_none()) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "versioned local profile is missing player_uuid",
                    ));
                }
                false
            }
            Some(version) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("unsupported local profile format version: {version}"),
                ));
            }
        };

        if document.profiles.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "local profile document contains no profiles",
            ));
        }

        let mut profiles = Vec::with_capacity(document.profiles.len());
        let mut ids = std::collections::HashSet::new();
        let mut player_uuids = std::collections::HashSet::new();
        for record in document.profiles {
            if !ids.insert(record.id) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("duplicate local profile ID: {}", record.id),
                ));
            }
            let player_uuid = match record.player_uuid {
                Some(id) => id,
                None => UserData::offline_uuid(&record.username),
            };
            if !player_uuids.insert(player_uuid) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("duplicate local player UUID: {player_uuid}"),
                ));
            }
            profiles.push(LocalProfile {
                id: record.id,
                username: record.username,
                player_uuid,
            });
        }

        if !ids.contains(&document.selected_profile) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "selected local profile does not exist",
            ));
        }

        Ok((profiles, document.selected_profile, migrated))
    }

    fn backup_path(path: &Path) -> PathBuf {
        let mut backup = path.as_os_str().to_owned();
        backup.push(".bak");
        PathBuf::from(backup)
    }

    #[allow(dead_code)] // Used by the local-profile picker API and its persistence tests.
    pub fn profiles(&self) -> &[LocalProfile] {
        &self.profiles
    }

    pub fn selected(&self) -> &LocalProfile {
        self.profiles
            .iter()
            .find(|profile| profile.id == self.selected_profile)
            .expect("validated local profile selection must exist")
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

        if self
            .profiles
            .iter()
            .any(|profile| profile.username.eq_ignore_ascii_case(username))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "a local profile with this name already exists",
            ));
        }

        let profile = LocalProfile::new(username);
        if self
            .profiles
            .iter()
            .any(|existing| existing.player_uuid == profile.player_uuid)
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "the new profile would reuse an existing player identity",
            ));
        }
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
        use std::io::Write as _;

        let document = LocalProfileDocument {
            format_version: Some(LOCAL_PROFILE_FORMAT_VERSION),
            profiles: self
                .profiles
                .iter()
                .map(|profile| LocalProfileRecord {
                    id: profile.id,
                    username: profile.username.clone(),
                    player_uuid: Some(profile.player_uuid),
                })
                .collect(),
            selected_profile: self.selected_profile,
        };
        let json = serde_json::to_vec_pretty(&document).map_err(std::io::Error::other)?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Preserve the previous valid document before publishing a replacement.
        if self.path.exists() {
            let backup = Self::backup_path(&self.path);
            let backup_temp = Self::unique_temp_path(&backup);
            let backup_result = (|| {
                let mut source = std::fs::File::open(&self.path)?;
                let mut destination = std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&backup_temp)?;
                std::io::copy(&mut source, &mut destination)?;
                destination.sync_all()?;
                std::fs::rename(&backup_temp, &backup)?;
                if let Err(error) = Self::sync_parent(&backup) {
                    tracing::warn!("Could not sync local profile backup directory: {error}");
                }
                Ok(())
            })();
            if backup_result.is_err() {
                let _ = std::fs::remove_file(&backup_temp);
                return backup_result;
            }
        }

        let temporary = Self::unique_temp_path(&self.path);
        let write_result = (|| {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            file.write_all(&json)?;
            file.sync_all()?;
            std::fs::rename(&temporary, &self.path)?;
            if let Err(error) = Self::sync_parent(&self.path) {
                // Publication already succeeded. Report reduced durability,
                // but do not make callers roll back in-memory state.
                tracing::warn!("Could not sync local profile directory: {error}");
            }
            Ok(())
        })();
        if write_result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        write_result
    }

    fn unique_temp_path(path: &Path) -> PathBuf {
        let mut temporary = path.as_os_str().to_owned();
        temporary.push(format!(".part-{}", uuid::Uuid::new_v4()));
        PathBuf::from(temporary)
    }

    fn sync_parent(path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
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

/// Microsoft credentials and their authorization state are kept separately
/// from the selected local player profile.
#[derive(Clone, Debug, PartialEq)]
pub struct MicrosoftSession {
    pub account: pomme_auth::AuthAccount,
    pub authorization: JavaAuthorization,
}

pub struct UserData {
    /// Active local player identity. Microsoft sign-in never changes these.
    pub username: String,
    pub uuid: uuid::Uuid,
    pub local_profile: LocalProfile,
    /// Independent Microsoft session used only for remote authorization.
    pub microsoft_session: Option<MicrosoftSession>,
    /// Compatibility mirror for the account menu; never authorizes by itself.
    pub access_token: Option<String>,
    /// Whether a parseable profile UUID was supplied; this alone is not
    /// authorization.
    pub has_profile: bool,
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
        _access_token: Option<String>,
        local_profile: LocalProfile,
    ) -> Self {
        let uuid = uuid.and_then(|value| uuid::Uuid::parse_str(&value).ok());
        let has_profile = uuid.is_some();
        let uuid = uuid.unwrap_or_else(|| Self::offline_uuid(&username));
        Self {
            username,
            uuid,
            access_token: None,
            has_profile,
            local_profile,
            microsoft_session: None,
            identity_source: IdentitySource::LauncherArguments,
            // Launcher arguments are identity inputs, not proof that the
            // account owns Java Edition. They never create a Microsoft session.
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
            microsoft_session: None,
            identity_source: IdentitySource::LocalProfile,
            java_authorization: JavaAuthorization::Unverified,
        }
    }

    pub fn from_account(account: &pomme_auth::AuthAccount, local_profile: LocalProfile) -> Self {
        let mut user = Self::from_local_profile(local_profile);
        user.set_microsoft_account(Some(account.clone()));
        user
    }

    /// Update only the Microsoft session; local profile, username and offline
    /// UUID remain stable across sign-in, sign-out and account switching.
    pub fn set_microsoft_account(&mut self, account: Option<pomme_auth::AuthAccount>) {
        match account {
            Some(account) => {
                let has_profile = uuid::Uuid::parse_str(&account.uuid).is_ok();
                let authorization = if has_profile {
                    JavaAuthorization::Verified {
                        expires_at: account.expires_at,
                    }
                } else {
                    JavaAuthorization::Unverified
                };
                self.access_token = Some(account.access_token.clone());
                self.has_profile = has_profile;
                self.identity_source = IdentitySource::MicrosoftAccount;
                self.java_authorization = authorization;
                self.microsoft_session = Some(MicrosoftSession {
                    account,
                    authorization,
                });
            }
            None => {
                self.microsoft_session = None;
                self.access_token = None;
                self.has_profile = false;
                self.identity_source = IdentitySource::LocalProfile;
                self.java_authorization = JavaAuthorization::Unverified;
            }
        }
    }

    /// Change the selected local player without discarding the Microsoft
    /// session. UUID policy is unchanged in this phase.
    #[allow(dead_code)] // Used by the local-profile selection flow in the next phase.
    pub fn set_local_profile(&mut self, profile: LocalProfile) {
        self.username = profile.username.clone();
        self.uuid = Self::offline_uuid(&self.username);
        self.local_profile = profile;
    }

    pub fn microsoft_profile_uuid(&self) -> Option<uuid::Uuid> {
        self.microsoft_session
            .as_ref()
            .and_then(|session| uuid::Uuid::parse_str(&session.account.uuid).ok())
    }

    /// The account identity used for multiplayer; never use local profile
    /// identity or launcher arguments as a substitute.
    pub fn multiplayer_identity(&self) -> Option<(&str, uuid::Uuid, &str)> {
        if !self.is_java_authorized() {
            return None;
        }
        let session = self.microsoft_session.as_ref()?;
        Some((
            session.account.username.as_str(),
            uuid::Uuid::parse_str(&session.account.uuid).ok()?,
            session.account.access_token.as_str(),
        ))
    }

    pub fn is_java_authorized(&self) -> bool {
        let Some(session) = self.microsoft_session.as_ref() else {
            return false;
        };
        let JavaAuthorization::Verified { expires_at } = session.authorization else {
            return false;
        };
        expires_at > unix_now()
            && !session.account.access_token.is_empty()
            && uuid::Uuid::parse_str(&session.account.uuid).is_ok()
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
    #[allow(dead_code)] // Retained as a guarded compatibility accessor.
    pub fn authenticated_access_token(&self) -> Option<&str> {
        self.multiplayer_identity().map(|(_, _, token)| token)
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
    use super::{JavaAuthorization, LocalProfile, LocalProfileStore, UserData};

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
    fn legacy_local_profiles_migrate_with_the_historical_offline_uuid() {
        let dir = std::env::temp_dir().join(format!("pomme-legacy-profile-{}", uuid::Uuid::new_v4()));
        let path = dir.join("local-profiles.json");
        std::fs::create_dir_all(&dir).unwrap();
        let profile_id = uuid::Uuid::new_v4();
        let legacy = serde_json::json!({
            "profiles": [{"id": profile_id, "username": "Alex"}],
            "selected_profile": profile_id
        });
        std::fs::write(&path, serde_json::to_vec_pretty(&legacy).unwrap()).unwrap();

        let store = LocalProfileStore::load_or_create(&path).unwrap();
        assert_eq!(store.selected().id, profile_id);
        assert_eq!(store.selected().username, "Alex");
        assert_eq!(
            store.selected().player_uuid.to_string(),
            "36532b5e-c442-3dbb-a24c-c7e55d0f979a"
        );
        let persisted: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(persisted["format_version"], LOCAL_PROFILE_FORMAT_VERSION);
        assert_eq!(persisted["selected_profile"], profile_id.to_string());
        assert!(path.with_file_name("local-profiles.json.bak").exists());

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn legacy_null_player_uuid_is_not_treated_as_an_absent_field() {
        let profile_id = uuid::Uuid::new_v4();
        let document = serde_json::json!({
            "profiles": [{
                "id": profile_id,
                "username": "Alex",
                "player_uuid": null
            }],
            "selected_profile": profile_id
        });
        assert!(LocalProfileStore::decode_document(&document.to_string()).is_err());
    }

    #[test]
    fn legacy_profiles_with_colliding_historical_uuids_are_rejected() {
        let first_id = uuid::Uuid::new_v4();
        let second_id = uuid::Uuid::new_v4();
        let document = serde_json::json!({
            "profiles": [
                {"id": first_id, "username": "Alex"},
                {"id": second_id, "username": "Alex"}
            ],
            "selected_profile": first_id
        });
        assert!(LocalProfileStore::decode_document(&document.to_string()).is_err());
    }

    #[test]
    fn null_format_version_is_not_treated_as_a_legacy_document() {
        let document = serde_json::json!({
            "format_version": null,
            "profiles": [],
            "selected_profile": uuid::Uuid::new_v4()
        });
        assert!(LocalProfileStore::decode_document(&document.to_string()).is_err());
    }

    #[test]
    fn corrupt_local_profiles_are_preserved_and_never_replaced() {
        let dir = std::env::temp_dir().join(format!("pomme-corrupt-profile-{}", uuid::Uuid::new_v4()));
        let path = dir.join("local-profiles.json");
        std::fs::create_dir_all(&dir).unwrap();
        let original = b"{ this is not valid json";
        std::fs::write(&path, original).unwrap();

        assert!(LocalProfileStore::load_or_create(&path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), original);

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn new_profile_starts_with_the_existing_offline_identity() {
        let profile = LocalProfile::new("NewPlayer");
        assert_eq!(
            profile.player_uuid,
            UserData::offline_uuid("NewPlayer")
        );
        assert_ne!(profile.id, profile.player_uuid);
    }

    #[test]
    fn versioned_local_profile_uuid_survives_reload() {
        let dir = std::env::temp_dir().join(format!("pomme-versioned-profile-{}", uuid::Uuid::new_v4()));
        let path = dir.join("local-profiles.json");
        let store = LocalProfileStore::load_or_create(&path).unwrap();
        let original = store.selected().clone();
        let reloaded = LocalProfileStore::load_or_create(&path).unwrap();
        assert_eq!(reloaded.selected(), &original);
        assert_ne!(original.id, original.player_uuid);

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn microsoft_sign_in_sign_out_and_account_switch_preserve_local_profile() {
        let profile = LocalProfile::new("PersistentLocal");
        let mut user = UserData::from_local_profile(profile.clone());
        let local_username = user.username.clone();
        let local_uuid = user.uuid;

        let account = |username: &str, uuid: &str, token: &str| pomme_auth::AuthAccount {
            username: username.into(),
            uuid: uuid.into(),
            access_token: token.into(),
            expires_at: super::unix_now() + 3600,
        };

        user.set_microsoft_account(Some(account(
            "MicrosoftOne",
            "b50ad385-829d-3141-a216-7e7d7539ba7f",
            "token-one",
        )));
        assert_eq!(user.local_profile, profile);
        assert_eq!(user.username, local_username);
        assert_eq!(user.uuid, local_uuid);
        assert_eq!(
            user.multiplayer_identity().map(|(name, _, _)| name),
            Some("MicrosoftOne")
        );

        user.set_microsoft_account(Some(account(
            "MicrosoftTwo",
            "36532b5e-c442-3dbb-a24c-c7e55d0f979a",
            "token-two",
        )));
        assert_eq!(user.username, local_username);
        assert_eq!(user.uuid, local_uuid);
        assert_eq!(
            user.multiplayer_identity().map(|(name, _, _)| name),
            Some("MicrosoftTwo")
        );

        user.set_microsoft_account(None);
        assert_eq!(user.local_profile, profile);
        assert_eq!(user.username, local_username);
        assert_eq!(user.uuid, local_uuid);
        assert_eq!(user.multiplayer_identity(), None);
        assert!(user.allows_singleplayer(true));
        assert!(!user.allows_multiplayer());
    }

    #[test]
    fn changing_local_profile_keeps_microsoft_session() {
        let first = LocalProfile::new("FirstLocal");
        let second = LocalProfile::new("SecondLocal");
        let account = pomme_auth::AuthAccount {
            username: "OnlinePlayer".into(),
            uuid: "b50ad385-829d-3141-a216-7e7d7539ba7f".into(),
            access_token: "valid-session".into(),
            expires_at: super::unix_now() + 3600,
        };
        let mut user = UserData::from_account(&account, first);
        user.set_local_profile(second.clone());

        assert_eq!(user.local_profile, second);
        assert_eq!(user.username, "SecondLocal");
        assert_eq!(
            user.multiplayer_identity().map(|(name, _, _)| name),
            Some("OnlinePlayer")
        );
        assert!(user.allows_multiplayer());
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
