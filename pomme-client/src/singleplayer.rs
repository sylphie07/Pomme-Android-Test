//! The integrated server, as the rest of the client sees it.
//!
//! This is the only module that knows whether SteelMC is compiled in, so the
//! app phases can carry a world without a `cfg` at every construction site.

/// Whether this build can open a world.
pub const AVAILABLE: bool = cfg!(feature = "singleplayer");

/// The protocol the integrated server speaks.
#[cfg(feature = "singleplayer")]
pub const PROTOCOL: i32 = pomme_singleplayer::PROTOCOL;
#[cfg(not(feature = "singleplayer"))]
pub const PROTOCOL: i32 = pomme_protocol::version::NATIVE.protocol;

/// Shown wherever the feature is compiled out.
pub const UNAVAILABLE_MESSAGE: &str = "This build was compiled without singleplayer.";

#[cfg(feature = "singleplayer")]
mod steel {
    use std::fs::{File, OpenOptions, TryLockError};
    use std::path::Path;

    use pomme_singleplayer::{
        Difficulty, GameType, LaunchOptions, PendingWorld, Progress, ServerTransport, WorldHandle,
        launch,
    };

    use crate::net::conn::{MemoryEnd, memory_pipes};
    use crate::ui::world_list::{self, WorldSummary};

    /// Holds the world folder for the session. Two clients writing the same
    /// region files corrupts the save. Released when the file handle drops.
    struct SessionLock {
        _file: File,
    }

    impl SessionLock {
        fn take(dir: &Path) -> Result<Self, String> {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false)
                .open(dir.join("session.lock"))
                .map_err(|error| format!("Could not open the world's session lock: {error}"))?;
            file.try_lock().map_err(|error| match error {
                // Vanilla `selectWorld.locked`.
                TryLockError::WouldBlock => {
                    "Locked by another running instance of Minecraft.".to_owned()
                }
                TryLockError::Error(error) => {
                    format!("Could not lock the world's session lock: {error}")
                }
            })?;
            Ok(Self { _file: file })
        }
    }

    enum Server {
        Starting(PendingWorld),
        /// Dropping the handle stops the server and saves.
        Running(WorldHandle),
    }

    /// The integrated server for this session.
    pub struct World {
        // Declared first so the server saves and stops before the lock is
        // released, not after.
        server: Server,
        _lock: SessionLock,
    }

    impl World {
        /// Advances a starting world and watches a running one, returning the
        /// reason the server is gone.
        pub fn poll(&mut self) -> Result<(), String> {
            match &mut self.server {
                Server::Starting(pending) => match pending.poll() {
                    Progress::Starting => Ok(()),
                    Progress::Ready(handle) => {
                        self.server = Server::Running(handle);
                        Ok(())
                    }
                    Progress::Failed(error) => Err(error),
                },
                Server::Running(handle) if handle.is_finished() => {
                    Err("The integrated server stopped unexpectedly.".to_owned())
                }
                Server::Running(_) => Ok(()),
            }
        }

        fn handle(&self) -> Option<&WorldHandle> {
            match &self.server {
                Server::Starting(pending) => pending.handle(),
                Server::Running(handle) => Some(handle),
            }
        }

        /// Asks the server to save and stop, whether or not it finished
        /// starting. Returns without waiting.
        pub fn begin_close(&self) {
            if let Some(handle) = self.handle() {
                handle.begin_shutdown();
            }
        }

        /// Whether the server has finished saving and stopped.
        pub fn is_closed(&self) -> bool {
            self.handle().is_none_or(WorldHandle::is_finished)
        }
    }

    /// Starts `world` and hands back the client's end of the pipe.
    pub fn open(
        world: &WorldSummary,
        dir: &Path,
        view_distance: u8,
    ) -> Result<(World, MemoryEnd), String> {
        let lock = SessionLock::take(dir)?;
        let (client, server) = memory_pipes();

        // An empty seed means the server picks one on first launch and keeps it
        // in its own level data from then on.
        let seed = world_list::parse_seed(&world.seed).unwrap_or_else(|| fastrand::i64(..));

        let pending = launch(LaunchOptions {
            save_path: dir.to_path_buf(),
            seed,
            view_distance,
            simulation_distance: view_distance,
            game_mode: match world.game_mode {
                world_list::GameMode::Survival => GameType::Survival,
                world_list::GameMode::Creative => GameType::Creative,
            },
            difficulty: match world.difficulty {
                world_list::Difficulty::Peaceful => Difficulty::Peaceful,
                world_list::Difficulty::Easy => Difficulty::Easy,
                world_list::Difficulty::Normal => Difficulty::Normal,
                world_list::Difficulty::Hard => Difficulty::Hard,
            },
            allow_commands: world.allow_commands,
            transport: ServerTransport {
                read: Box::new(server.rx),
                write: Box::new(server.tx),
            },
        });

        Ok((
            World {
                server: Server::Starting(pending),
                _lock: lock,
            },
            client,
        ))
    }
}

#[cfg(not(feature = "singleplayer"))]
mod steel {
    use std::path::Path;

    use crate::net::conn::MemoryEnd;
    use crate::ui::world_list::WorldSummary;

    /// Uninhabited without the feature, so `Option<World>` costs nothing and
    /// the compiler proves every singleplayer arm dead.
    pub enum World {}

    impl World {
        pub const fn poll(&mut self) -> Result<(), String> {
            match *self {}
        }

        pub const fn begin_close(&self) {
            match *self {}
        }

        pub const fn is_closed(&self) -> bool {
            match *self {}
        }
    }

    pub fn open(
        _world: &WorldSummary,
        _dir: &Path,
        _view_distance: u8,
    ) -> Result<(World, MemoryEnd), String> {
        Err(super::UNAVAILABLE_MESSAGE.to_owned())
    }
}

pub use steel::{World, open};

#[cfg(all(test, feature = "singleplayer"))]
mod integration_tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, Instant};

    use crate::net::NetworkEvent;
    use crate::net::connection::{ConnectArgs, Transport, spawn_connection};
    use crate::ui::chat::ChatOptions;
    use crate::ui::menu::SkinOptions;
    use crate::ui::world_list::{Difficulty, GameMode, WorldSummary};

    struct TempWorld(PathBuf);

    impl Drop for TempWorld {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn summary() -> WorldSummary {
        WorldSummary {
            name: "Steel integration test".to_owned(),
            folder: "steel-integration-test".to_owned(),
            last_played: 0,
            game_mode: GameMode::Creative,
            hardcore: false,
            allow_commands: true,
            difficulty: Difficulty::Peaceful,
            seed: "123456789".to_owned(),
            version: "integration-test".to_owned(),
            extra: serde_json::Map::new(),
        }
    }

    fn enter_and_leave(rt: &tokio::runtime::Runtime, dir: &Path) {
        let (world, end) = super::open(&summary(), dir, 2)
            .expect("SteelMC should accept the integrated world launch");

        // SteelMC's embedded listener is deliberately offline-mode. The
        // test identity exercises only the local Java protocol handshake;
        // it is not used by, or accepted as, a Microsoft ownership proof.
        let connection = spawn_connection(
            rt,
            ConnectArgs {
                transport: Transport::Memory {
                    end,
                    protocol: super::PROTOCOL,
                },
                username: "PommeTest".to_owned(),
                uuid: uuid::Uuid::nil(),
                access_token: None,
                view_distance: 2,
                chat_options: ChatOptions::default(),
                skin_options: SkinOptions::default(),
            },
        );

        let deadline = Instant::now() + Duration::from_secs(180);
        let mut connected = false;
        let mut player_login = false;
        let mut received_chunk = false;

        while Instant::now() < deadline {
            if let Err(error) = world.poll() {
                panic!("SteelMC stopped while the client was joining: {error}");
            }

            match connection.event_rx.recv_timeout(Duration::from_millis(50)) {
                Ok(NetworkEvent::Connected) => connected = true,
                Ok(NetworkEvent::PlayerLogin { .. }) => player_login = true,
                Ok(NetworkEvent::ChunkLoaded { .. }) => received_chunk = true,
                Ok(NetworkEvent::Disconnected { reason }) => {
                    panic!("the integrated client disconnected: {reason}");
                }
                Ok(_) => {}
                Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                    panic!("the client event channel closed before the world loaded");
                }
            }

            if connected && player_login && received_chunk {
                break;
            }
        }

        assert!(connected, "the client should complete the login/configuration handshake");
        assert!(player_login, "the client should receive the integrated world's player login");
        assert!(received_chunk, "the client should receive real world chunk data");

        drop(connection);
        world.begin_close();

        let close_deadline = Instant::now() + Duration::from_secs(60);
        while !world.is_closed() && Instant::now() < close_deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(world.is_closed(), "SteelMC should save and stop after the client leaves");
        drop(world);
    }

    /// Exercises the real Pomme client connection against the embedded
    /// SteelMC server, then leaves and re-enters the same save in one process.
    /// This is intentionally ignored in ordinary unit runs; ci-singleplayer
    /// runs ignored integration tests.
    #[test]
    #[ignore = "starts SteelMC, generates terrain, and joins through the real in-memory Java protocol"]
    fn integrated_client_can_enter_and_reenter_a_steel_world() {
        let root = std::env::temp_dir().join(format!(
            "pomme-client-steel-e2e-{}",
            std::process::id()
        ));
        let save = TempWorld(root.join("saves").join("steel-integration-test"));
        fs::create_dir_all(&save.0).expect("the test save directory should be creatable");

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .expect("the integration test runtime should start");

        enter_and_leave(&rt, &save.0);
        enter_and_leave(&rt, &save.0);
    }
}
