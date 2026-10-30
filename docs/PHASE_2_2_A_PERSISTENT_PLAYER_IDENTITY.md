# Phase 2.2-A — Persistent Player Identity per Local Profile

Last updated: 2026-10-30
Status: **Phase 2.2-A investigation/design completed. A later user authorization for Phase 2.2-B1–B4 is recorded in Section 15; C/D/E remain outside that authorization.**

## 1. Scope and safety boundary

Design a stable player identity for each local Pomme profile so changing the visible username does not change the identity SteelMC uses to locate persistent player data.

Hard requirements:
- Preserve all existing worlds and player data.
- Keep `LocalProfile.id` and `LocalProfile.player_uuid` independent.
- Keep local identity separate from Microsoft/Java authentication.
- Never trust arbitrary UUIDs from generic offline `Hello` packets.
- Do not alter generic offline login.
- No profile rename implementation in this phase.
- No source-code changes, submodule changes, player-data migration, merge, PR, master changes, or APK generation.
- Stop after this report and wait for explicit user authorization.

## 2. Exact repository state

Repository: `sylphie07/Pomme-Android-Test`
Working branch: `unofficial-android-menu-access`

The user's requested historical baseline was `c730ae9e1334646f507f9f068e8774e8950a18a8`. It was checked during the investigation and matched that requested SHA at that point.

When preparing this chat record, the live branch endpoint reported a newer branch HEAD:
- Current branch HEAD: `4c1073a5af9f77b0b8f8c6723598496c1bde7a71`
- Commit message: `docs: fix project context documentation typo`
- The commit diff only corrects a typo in `docs/PROJECT_CONTEXT.md`.
- SteelMC gitlink at the current branch tree: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- `.gitmodules` points `third_party/SteelMC` to `https://github.com/Steel-Foundation/SteelMC.git`.

The Phase 2.2-A source investigation was performed against Pomme HEAD `c730ae9e1334646f507f9f068e8774e8950a18a8` and SteelMC `e655880a7b1487b2444c70013c14ee86c69d53b3`. Before any future source edits, verify the live branch HEAD and submodule pin again.

## 3. Proposed data model

```rust
struct LocalProfile {
    id: Uuid,          // stable profile-record identifier
    username: String,  // visible Minecraft name
    player_uuid: Uuid, // stable identity used inside local worlds
}
```

Do not repurpose `id`. It already identifies a profile record and participates in selected-profile lookup. The player identity has a different lifecycle and purpose.

## 4. Pomme source inspection and current identity flow

Inspected at Pomme commit `c730ae9e1334646f507f9f068e8774e8950a18a8`:

- `pomme-client/src/user.rs`
  - `LocalProfile`, `LocalProfile::new()`
  - `LocalProfileDocument`
  - `LocalProfileStore`, `load_or_create()`, `persist()`
  - `UserData::from_local_profile()`, `UserData::offline_uuid()`, `UserData::set_local_profile()`
- `pomme-client/src/app/core.rs`
  - construction and ownership of `UserData`
- `pomme-client/src/app/phases/in_menu.rs`
  - `open_world()`, `connect_args()`, `apply_account()`
- `pomme-client/src/net/connection.rs`
  - `ConnectArgs`, `Transport`, `connect_to_server()`, `ServerboundHello`
- `pomme-singleplayer/src/lib.rs`
  - `LaunchOptions`, `launch()`, `serve()`, `JavaTcpClient::from_transport()`, world startup

Source links:
- [user.rs at inspected commit](https://github.com/sylphie07/Pomme-Android-Test/blob/c730ae9e1334646f507f9f068e8774e8950a18a8/pomme-client/src/user.rs)
- [core.rs](https://github.com/sylphie07/Pomme-Android-Test/blob/c730ae9e1334646f507f9f068e8774e8950a18a8/pomme-client/src/app/core.rs)
- [in_menu.rs](https://github.com/sylphie07/Pomme-Android-Test/blob/c730ae9e1334646f507f9f068e8774e8950a18a8/pomme-client/src/app/phases/in_menu.rs)
- [connection.rs](https://github.com/sylphie07/Pomme-Android-Test/blob/c730ae9e1334646f507f9f068e8774e8950a18a8/pomme-client/src/net/connection.rs)
- [pomme-singleplayer/lib.rs](https://github.com/sylphie07/Pomme-Android-Test/blob/c730ae9e1334646f507f9f068e8774e8950a18a8/pomme-singleplayer/src/lib.rs)

Verified design-relevant behavior:
1. `LocalProfile::new()` generates a random UUID v4 for `LocalProfile.id`.
2. The current local player UUID is derived from the username using the vanilla offline UUID algorithm (MD5 of `OfflinePlayer:{username}`).
3. `LocalProfile.id` is not the player UUID currently used by SteelMC.
4. Pomme sends username and UUID in the protocol Hello packet.
5. The embedded server is launched in-process and uses an in-memory transport.
6. The existing Phase 2.1 account flow keeps Microsoft session state separate from local profile state. `connect_args()` distinguishes local in-memory transport from remote transport.

## 5. Critical SteelMC finding

Inspected at SteelMC commit `e655880a7b1487b2444c70013c14ee86c69d53b3`:

- `steel-login/src/handlers/login.rs`: `handle_hello()`
- `steel-login/src/tcp_client.rs`: `ServerConnectionSession`, `JavaTcpClient`, `from_transport()`
- `steel-core/src/player/profile/mod.rs`: `offline_uuid()`
- `steel-core/src/server/player_lifecycle.rs`
- `steel-core/src/player/player_data_storage/mod.rs`
- `steel-core/src/player/player_data_storage/file_storage.rs`
- `steel-core/src/player/player_data_storage/permissions.rs`
- `steel-core/src/player/profile/known_players.rs`
- `steel-core/src/player/player_data.rs`
- `steel-core/src/player/advancement/mod.rs`

Source links:
- [login handler](https://github.com/Steel-Foundation/SteelMC/blob/e655880a7b1487b2444c70013c14ee86c69d53b/steel-login/src/handlers/login.rs)
- [TCP client/session](https://github.com/Steel-Foundation/SteelMC/blob/e655880a7b1487b2444c70013c14ee86c69d53b/steel-login/src/tcp_client.rs)
- [offline UUID](https://github.com/Steel-Foundation/SteelMC/blob/e655880a7b1487b2444c70013c14ee86c69d53b/steel-core/src/player/profile/mod.rs)
- [player lifecycle](https://github.com/Steel-Foundation/SteelMC/blob/e655880a7b1487b2444c70013c14ee86c69d53b/steel-core/src/server/player_lifecycle.rs)
- [file storage](https://github.com/Steel-Foundation/SteelMC/blob/e655880a7b1487b2444c70013c14ee86c69d53b/steel-core/src/player/player_data_storage/file_storage.rs)

**Critical:** SteelMC explicitly treats the UUID supplied in Hello as client-provided/untrusted. In offline login, `handle_hello()` calculates the effective UUID from the requested username. Therefore, merely changing Pomme's outgoing UUID will not establish a persistent identity. Do not change generic offline login to trust Hello UUIDs.

## 6. Storage keys and compatibility

SteelMC uses the logged-in `gameprofile.id()` as the player-data key. Inspected persistence locations include:

| Data | Storage key/location |
|---|---|
| Global player data | `global/players/data/{UUID}` |
| Per-domain player data | `{domain}/players/data/{UUID}` |
| Stats | `{domain}/players/stats/{UUID}.toml` |
| Individual permissions | `global/player_permissions.toml`, keyed by UUID |
| Known players | `global/known_players.dat`, UUID/name association |

Persistent player data includes inventory/equipment, Ender Chest, position/rotation, health, game mode, experience/score, abilities and other player state.

World region/build data is not keyed by player UUID and should not need migration for this identity-field introduction.

### Legacy migration rule

For a valid legacy profile without `player_uuid`, initialize:

```text
player_uuid = offlineUUID(username_currently_stored)
```

Preserve `LocalProfile.id` and selected-profile ID exactly.

If this historical UUID is retained and the embedded SteelMC uses it, the existing storage paths remain identical. **No player-world file migration is required just to introduce the field.** This is the safest path for existing worlds.

Exceptions:
- If the username had already changed before migration, the currently stored name may not reproduce the UUID that owns old files. Do not guess; use verified historical mapping or manual recovery.
- Exact duplicate usernames already share the same name-derived UUID and therefore the same player identity/data today. Do not split or clone data silently.
- Case-only name differences may produce distinct offline UUIDs but collide in SteelMC's case-insensitive known-player name index.
- Old client versions that do not understand `player_uuid` may return to name-derived identity after a rename. Future rollout must define downgrade behavior and warnings.

## 7. JSON, corruption, duplicates, and write safety

Current `LocalProfileDocument` has no explicit format version. `LocalProfileStore::load_or_create()` has a destructive failure mode: read/parse/validation failures (including corrupt JSON or invalid selection/empty profiles) can lead to creating and persisting a default profile, potentially replacing the prior file.

This must be corrected before migration is implemented.

Recommended format:
- Add `format_version`.
- New profiles receive a generated persistent `player_uuid`.
- Legacy valid documents without the field get the current-name historical offline UUID.
- Preserve profile record IDs and selection.
- Reject or quarantine malformed/unknown future versions; never overwrite the only original.
- Validate duplicate profile-record IDs and duplicate player UUIDs.
- Duplicate names require explicit conflict handling; do not silently assign new identities.
- Never regenerate a persisted `player_uuid` on reload.

Current `persist()` uses a temporary `.part` file and rename, which is a useful foundation, but no full recovery protocol was verified. Risks include fixed temp name/concurrent writes, no verified recovery of stale temp on startup, missing explicit file/directory sync, and no complete backup/recovery transaction.

Recommended future persistence: unique temp in same directory, write and sync, retain validated backup, atomic publish, sync directory where supported, startup recovery rules, and idempotent migration. On every error preserve the last known-good source file. Do not claim filesystem atomicity beyond platform/filesystem guarantees.

## 8. Trusted identity handoff alternatives

| Option | Assessment |
|---|---|
| Trust UUID in generic Hello | Reject. Client-controlled and unsafe; permits spoofing on offline servers. |
| Put player identity in shared `ServerConnectionSession` | Not recommended. That object is listener/session scoped and can be shared; player identity belongs to a concrete connection. |
| Embedded-server-wide configuration | Possible, but identity is per player/connection and config can become awkward if multiple local sessions/connections exist. |
| Internal context on a dedicated integrated connection constructor | **Recommended.** Explicit trust boundary; only Pomme's in-process server construction path can supply it; generic remote/offline constructors remain unchanged. |

Recommended conceptual design:
- Add a dedicated integrated identity value carrying visible username and persistent UUID.
- Pass it through a narrowly scoped internal/integrated transport construction path to the concrete connection object.
- The integrated login handler uses this identity only when that connection was created by the trusted embedded-server path.
- Generic remote/offline login continues deriving UUID from username and ignores Hello UUID for identity.
- Keep `ServerConnectionSession`'s existing listener/session responsibility rather than turning it into a shared player identity container.

This is an architectural recommendation, not a verified implementation. Exact API placement still needs review during 2.2-C.

## 9. Microsoft/Java authentication separation

The local `player_uuid` must only represent local embedded-world identity.

- Microsoft session/token and authenticated Java profile remain separate.
- Local singleplayer uses the local profile's `username` + `player_uuid`.
- Remote multiplayer continues through the existing official Microsoft authentication/authorization path.
- Sign-in, sign-out, or switching Microsoft accounts must not mutate `LocalProfile.id`, `username`, or `player_uuid`.
- Do not weaken ownership checks or online-mode verification.

## 10. Automated tests to add later

No tests were run for this design report. These are required future cases, with expected outcomes:

### Profile/model unit tests
1. New profile: valid distinct `id` and `player_uuid`; persist/reload exact values.
2. Legacy JSON without `player_uuid`: assign `offlineUUID(username)`; preserve `id` and selected ID.
3. Restart/reload: UUID remains unchanged.
4. Future rename: username changes while player UUID remains constant.
5. Profile switching: each profile retains its own UUID.
6. Microsoft sign-in/out/account switching: local identity remains unchanged.
7. Different names: distinct legacy offline UUIDs.
8. Same names: collision explicitly detected/reported; no silent data split.
9. Duplicate `id` or `player_uuid`: reject/quarantine without destructive regeneration.
10. Corrupt JSON, missing required fields, null/malformed UUID, unknown future version: preserve original and surface recovery state.

### SteelMC login/security tests
11. Integrated connection receives trusted persistent UUID and visible username.
12. Generic offline login with arbitrary Hello UUID still uses `offlineUUID(requested_username)`.
13. Remote clients cannot select the integrated trusted-identity constructor/context.
14. No regression to online authenticated profile validation.

### Persistence/integration tests
15. Seed inventory/equipment, Ender Chest, position, health/game mode, XP/score, stats, permissions and known-player mapping.
16. Reconnect with same UUID after rename: same records load.
17. Confirm global and every domain storage path remain keyed by the same UUID.
18. Confirm world region/build files are unchanged.
19. Two profiles with distinct UUIDs do not leak data across each other.

### Crash/recovery tests
20. Interrupt writes at each stage (before temp, during temp, after sync, after backup, after publish, before directory sync).
21. Restart and verify a valid previous or fully published document is recovered.
22. Retry migration repeatedly: idempotent; never changes a stored UUID.
23. If a destination is corrupt or collides, retain source and require explicit recovery.

Unit tests, integration tests, CI, and real Android-device tests must be reported separately. None is claimed as passed here.

## 11. Risks and unresolved points

- The current profile-store error path can overwrite corrupt/invalid profile state; fix before any identity migration.
- Existing duplicate names and UUIDs require non-destructive conflict policy.
- Profiles renamed before this migration may need manual/historical identity recovery.
- Older app versions may not preserve a UUID across a later rename.
- The integrated identity API must be inaccessible to ordinary remote connections.
- **Advancement persistence limitation confirmed at the inspected SteelMC pin:** `steel-core/src/player/mod.rs` contains `Player.advancements: SyncMutex<PlayerAdvancement>`, initializes it with `PlayerAdvancement::default()`, and includes the explicit comment `// TODO save the advancement`. No advancement field/serialization path was found in the inspected `PersistentPlayerData`/player-storage flow. Advancements therefore do not persist in this inspected revision. This is a separate SteelMC limitation; a stable player UUID alone will not fix it. Any future advancement-persistence work needs its own design, storage compatibility review, and tests.
- The report did not run builds, tests, CI, or Android runtime checks.

## 12. Recommended implementation order (not authorized)

### Phase 2.2-B — Profile model and durable JSON migration
Add independent `player_uuid`, explicit document versioning, legacy-compatible initialization, strict validation, non-destructive corruption handling, durable atomic writes/recovery, and profile-store tests. No world-data migration.

### Phase 2.2-C — SteelMC trusted integrated identity
Implement dedicated per-connection internal identity path; integrated login uses it. Generic offline login and Hello trust boundary remain unchanged. Add security/regression tests.

### Phase 2.2-D — Pomme identity plumbing
Use `LocalProfile.player_uuid` for local identity and pass it through the integrated connection path. Keep Microsoft remote identity flow separate. Test profile switching and auth state independence.

### Phase 2.2-E — Existing-world validation and acceptance
Test legacy profile/world fixtures, all discovered data stores, duplicate/corrupt/recovery cases, downgrade behavior, CI, and real Android device. Verify no alternate player-data paths and no world-region changes.

Advancement to each phase requires review of prior results and explicit user authorization.

## 13. Session result

- Phase 2.2-A investigation/design report completed.
- No source files or submodule files changed during the investigation.
- No automated tests, build, CI run, or Android test were run.
- No world/player data was touched.
- No merge, PR, master change, or APK was performed.
- Implementation remains unauthorized; stop and wait for the user.


## 14. Clarification recorded on 2026-10-30 — advancement persistence

The earlier wording in Section 11 understated the evidence. At SteelMC gitlink `e655880a7b1487b2444c70013c14ee86c69d53b3`, `steel-core/src/player/mod.rs` contains the `Player.advancements` field, initializes it with `PlayerAdvancement::default()`, and marks it with `// TODO save the advancement`. The inspected `PersistentPlayerData` and file-storage path do not serialize this field. Therefore advancement progress is **not persisted by this inspected revision**. This is a pre-existing SteelMC limitation, independent of Pomme's proposed persistent player UUID. Do not claim advancement retention as an acceptance result until SteelMC persistence is separately implemented and tested.

No tests or builds were run for this clarification.


## 15. Implementation follow-up — 2026-10-30

The user subsequently authorized the dedicated programming chat to edit code and integrate the findings. This later authorization supersedes the original stop-and-wait boundary for **Phase 2.2-B1–B4 only**; it does not authorize C/D/E or SteelMC changes.

Selective implementation was made on `unofficial-android-menu-access`, using the experimental branch as reference rather than merging it:
- `pomme-client/src/user.rs`: independent `player_uuid`, format version 1, historical offline UUID migration, validation, backup recovery, corrupt-file quarantine, future-version downgrade protection, and additional tests.
- `pomme-client/src/lib.rs`: fail-safe startup handling for profile-store errors.

Source commits:
- `4bd084c8a9fca176da2545b800a8a2eb24c7b3e9`
- `334f5f958bf99a61bc3d67bcc48036b88122c41f`
- `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`

The runtime identity remains username-derived in `UserData::from_local_profile()`; the new persisted `player_uuid` is not yet supplied to SteelMC. That trusted integrated-connection work belongs to C/D.

At the time of the follow-up, CI run `41008897990` for source HEAD `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` was pending. No local Cargo test, build, Android build, or device test was executed. Therefore the implementation is applied but not yet verified.


### Follow-up compatibility correction — format 2

A later cross-chat audit identified that experimental format 1 assigned random `player_uuid` values to new profiles even though the runtime continued using `offline_uuid(username)`. This could have caused an identity discontinuity when C/D eventually consumed the stored field.

Correction applied in source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`:
- Newly created profiles now persist the historical offline UUID used by the current runtime.
- If a different profile resolves to the same player UUID, creation is rejected rather than silently splitting identity.
- Format 1 is treated as the unmerged experiment schema and migrated to format 2, replacing the unused random field with the historical offline UUID.
- Added a test for this migration.
- Formatting changes were applied in `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc`.

CI run `41009337238` for the PR merge ref with source head `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc` was pending at the last check. No local or Android test was run.


### Backup recovery snapshot correction — 2026-10-30

Source commit `c0576191407a1ad53db77c0635c7b0820669e547` changes `restore_backup()` to accept the exact byte slice already validated by the caller. This avoids reopening the backup path after validation and ensures the restored bytes are the bytes that passed schema/integrity checks. CI verification remains pending.
