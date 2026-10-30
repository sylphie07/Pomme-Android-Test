# Pomme Android — Development Log

This log records investigations, code generation, edits, test runs, and decisions. A proposal is not an implementation; an implementation is not a verified fix unless tests or runtime checks confirm it.

## 2026-10-30 — Session 001: Establish persistent project documentation

### Scope
Create a repository-based memory/handoff system so future ChatGPT conversations can resume work from GitHub instead of relying on the full prior chat transcript.

### Repository checks
- Repository: `sylphie07/Pomme-Android-Test`
- Confirmed branch exists: `unofficial-android-menu-access`.
- Read `README.md` from that branch.
- Checked for the planned documentation files; `docs/PROJECT_CONTEXT.md`, `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/ARCHITECTURE.md`, `docs/DECISIONS.md`, and `docs/KNOWN_ISSUES.md` were absent at the time of inspection.

### Work performed
- Prepared the initial project context, continuity handoff, development log, decisions register, and known-issues register.
- No application/source code was edited in this session.
- No build, automated test, or Android runtime test was run in this session.

### Historical context captured
- Main focus: allow menu navigation while signed out while retaining required authentication checks for gameplay.
- Entry points to trace: `MenuAction::Connect`, `MenuAction::PlayWorld`, `open_world()`, and quick-start/startup arguments.
- Android singleplayer discrepancy remains an investigation item.
- CI run #39 was previously reported in chat as still running; this is not a current CI result and must be checked live.

### Verification status
- Documentation write: confirmed. All five initial documentation files were fetched successfully from the target branch after creation.
- Source changes: none.
- Tests: not run.

### Next action
Retrieve the current branch HEAD, then resume the source-level investigation and update this log with verified findings.


## 2026-10-30 — Session 002: Record Phase 2.2-A persistent identity focus

### Scope
Update repository-backed project memory so a fresh chat can resume the current technical focus without reconstructing several long conversations.

### Repository checks
- Confirmed repository: `sylphie07/Pomme-Android-Test`.
- Confirmed branch exists: `unofficial-android-menu-access`.
- Read `README.md`, `docs/PROJECT_CONTEXT.md`, `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, and `docs/DECISIONS.md` from the working branch.
- `docs/README.md` and `docs/ARCHITECTURE.md` were not found at the checked paths.
- Repository commit search returned `a147ca27e24850e5d6e796e1dc7bd02e1ecf3a72` as the most recent observed commit. Its diff enables the `singleplayer` feature for the Android `pomme-client` dependency in `pomme-android/Cargo.toml`.
- Previous known HEAD `c730ae9e1334646f507f9f068e8774e8950a18a8` is stale.
- The branch-search result confirmed branch existence but did not expose its exact HEAD SHA. The latest observed commit is therefore recorded as a search result, not asserted as a confirmed branch-ref HEAD.

### Work performed
- Recorded the Phase 2.2-A objective, status, known audit findings, compatibility questions, safety constraints, and next action in project documentation.
- Added `docs/PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md` as the focused technical scope/handoff document.
- Updated `docs/PROJECT_CONTEXT.md` and `docs/CONTINUITY.md`.
- No application source code or submodule contents were changed.
- No merge, PR, APK build, or source-code test was performed.

### Current technical status
- Phase 2.2-A design report from the other chat is pending.
- No implementation of `player_uuid` has been authorized or recorded.
- The previously reported SteelMC findings still require source-level verification at the exact pinned submodule SHA before implementation.
- Advancement persistence and any unlisted player-data stores remain unverified.

### Next action
Review the complete Phase 2.2-A report, verify it against the current Pomme branch and pinned SteelMC source, then prepare a separately approved Phase 2.2-B implementation plan.


## 2026-10-30 — Session 003: Complete Phase 2.2-A persistent identity design report

### Scope
Complete the read-only investigation and write a durable handoff so a new chat can continue without relying on the full conversation.

### Repository verification
- Repository: `sylphie07/Pomme-Android-Test`.
- Working branch: `unofficial-android-menu-access`.
- At investigation start, the branch HEAD matched the user's expected SHA `c730ae9e1334646f507f9f068e8774e8950a18a8`.
- When recording this handoff, the live branch endpoint reported HEAD `4c1073a5af9f77b0b8f8c6723598496c1bde7a71`, commit message `docs: fix project context documentation typo`.
- The current branch tree's SteelMC gitlink is `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- `.gitmodules` points to `https://github.com/Steel-Foundation/SteelMC.git`.

### Source inspection performed
Pomme at `c730ae9e1334646f507f9f068e8774e8950a18a8`:
- `pomme-client/src/user.rs`
- `pomme-client/src/app/core.rs`
- `pomme-client/src/app/phases/in_menu.rs`
- `pomme-client/src/net/connection.rs`
- `pomme-singleplayer/src/lib.rs`

SteelMC at `e655880a7b1487b2444c70013c14ee86c69d53b3`:
- `steel-login/src/handlers/login.rs`
- `steel-login/src/tcp_client.rs`
- `steel-core/src/player/profile/mod.rs`
- `steel-core/src/server/player_lifecycle.rs`
- `steel-core/src/player/player_data_storage/mod.rs`
- `steel-core/src/player/player_data_storage/file_storage.rs`
- `steel-core/src/player/player_data_storage/permissions.rs`
- `steel-core/src/player/profile/known_players.rs`
- `steel-core/src/player/player_data.rs`
- `steel-core/src/player/advancement/mod.rs`

### Findings
- Current local player UUID is derived from username; `LocalProfile.id` is a separate profile-record identifier.
- SteelMC offline login treats Hello UUID as untrusted and derives the effective UUID from username.
- Stable identity therefore requires a trusted internal identity handoff for the embedded server, scoped to a concrete connection.
- Recommended design is not to put player identity into a shared `ServerConnectionSession`; use a dedicated integrated connection construction/context path.
- Legacy profiles should receive `player_uuid = offlineUUID(username_currently_stored)`, retaining `LocalProfile.id`.
- If historical UUID is retained, SteelMC's existing UUID-keyed player paths remain unchanged; no world/player file migration is needed just to add the field.
- Profile store's error handling can recreate/persist a default profile after read/parse/validation failure; this is a critical data-preservation issue to fix before migration.
- Duplicate names/UUIDs, previously renamed profiles, corrupt JSON, interrupted writes, and older-client downgrade need explicit non-destructive handling.
- Microsoft/Java authentication remains separate from local identity.
- Advancement persistence was not verified; do not claim it is absent.

### Work performed
- Completed the Phase 2.2-A technical design report in `docs/PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md`.
- Updated `docs/CONTINUITY.md` with exact repository state, findings, restrictions, and startup checklist.
- Appended this session to `docs/DEVELOPMENT_LOG.md`.
- Documentation only. No application source code, SteelMC submodule contents, world files, or player data were changed.

### Verification and tests
- GitHub branch endpoint and current tree were checked.
- Source investigation was read-only.
- No build, unit/integration tests, CI run, or Android-device test was performed.
- No test is claimed as passed.

### Explicitly not performed
- No source changes.
- No UUID/profile migration.
- No profile rename implementation.
- No generic offline login/authentication changes.
- No Microsoft authentication changes.
- No master modification, merge, PR, or APK generation.

### Next action
Wait for the user's explicit authorization. If authorized, begin only Phase 2.2-B after re-reading all docs and re-verifying branch HEAD and SteelMC pin.


## 2026-10-30 — Session 004: Refresh handoff and add recommendations for parallel chats

### Scope
Refresh repository-backed continuity after the Phase 2.2-A report and record actionable review recommendations for other chats working on separate Pomme areas.

### Repository/documentation checks
- Re-read `docs/PROJECT_CONTEXT.md`, `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, and `docs/PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md` from `unofficial-android-menu-access`.
- Confirmed repository identity and branch existence through GitHub.
- Revalidated that commit `0902bb4571bc9210eaffdfc1d8a688d6e5e0b417` exists and contains only a `docs/DEVELOPMENT_LOG.md` change. The preceding known documentation commits `920f75d0cbda76f01132d4f85a70b63af2aa612f` and `2c9e953b7d09ab846a865b4e40b6eb61831fa786` were also inspected.
- The connector's branch search confirms branch existence but does not expose the branch-ref SHA; the last known branch HEAD remains `0902bb4571bc9210eaffdfc1d8a688d6e5e0b417`, subject to mandatory re-verification at the next session start.
- SteelMC pin remains recorded as `e655880a7b1487b2444c70013c14ee86c69d53b3`; no submodule update was performed.
- A missing-file response occurred for an initially guessed report path; the actual report path was then fetched successfully. No repository file was changed by that failed read.

### Findings clarified
- The previous report's advancement wording was too cautious. At the inspected SteelMC pin, `steel-core/src/player/mod.rs` initializes `Player.advancements` with `PlayerAdvancement::default()` and includes the explicit comment `TODO save the advancement`. The inspected persistence model does not serialize advancements. This is a confirmed limitation of that revision; it is separate from player UUID continuity.
- No builds, unit tests, integration tests, CI workflows, or Android-device checks were run during this documentation session.
- GitHub reads cannot prove the user's local Termux worktree is clean.

### Work performed
- Updated continuity handoff and appended this session.
- Added `docs/CHAT_HANDOFF_RECOMMENDATIONS.md` with scoped review guidance for menu/authentication, Android singleplayer, profile identity, SteelMC persistence, and Android build/CI chats.
- Updated the Phase 2.2-A report to state the advancement-persistence limitation precisely.
- Documentation only. No application source, SteelMC submodule contents, world/player data, or APK artifacts were changed.

### Tests and verification
- No tests/build/CI/Android run was executed. No test is claimed as passed.
- The change is documentation-only; its GitHub commit and file contents must be verified after writing.

### Next action
Use the new cross-chat recommendations as review prompts, not implementation authorization. Continue Phase 2.2-B only after the user explicitly approves implementation and the live branch/submodule state is rechecked.


## 2026-10-30 — Session 005: Define dedicated programming-chat handoff workflow

### Scope
Review the repository reminders for parallel Pomme chats and make the handoff to the user's separate programming-only chat explicit. User specifically instructed that this session must not touch programming/source work.

### Repository verification
- Repository: `sylphie07/Pomme-Android-Test`.
- Authorized branch: `unofficial-android-menu-access`.
- Live branch HEAD at start: `bb28e3a4eeedfe5e94540b027024a4dc48014ef9`.
- SteelMC gitlink at that HEAD: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- `.gitmodules` points to `https://github.com/Steel-Foundation/SteelMC.git`.
- Re-read `docs/CONTINUITY.md`, the complete Phase 2.2-A report, `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`, `docs/DEVELOPMENT_LOG.md`, `docs/PROJECT_CONTEXT.md`, and `docs/DECISIONS.md`.

### Finding
The existing cross-chat guide contained general rules and technical-area review prompts, but did not sufficiently formalize the user's dedicated programming chat as the single implementation owner or require every investigation chat to leave structured implementation handoffs.

### Documentation changes
- Expanded `docs/CHAT_HANDOFF_RECOMMENDATIONS.md` with:
  - explicit separation between investigation chats and the dedicated programming chat;
  - a rule that investigation chats leave recommendations and evidence rather than editing code;
  - the programming chat's verification, authorization, implementation, testing, and reporting responsibilities;
  - a required 11-part handoff template;
  - current pending programming items for Phase 2.2-B and separate SteelMC advancement persistence.
- Updated `docs/CONTINUITY.md` with the verified starting HEAD, SteelMC pin, new documentation commit, and scope boundary.

### Verification
- Documentation update commit for the cross-chat guide: `ab67016f7c7eaff56f4c62b39563d065d5e806c7`.
- Continuity update is being recorded in this session; verify its resulting commit and file contents before treating this log entry as fully recorded.
- No tests, builds, CI, or Android-device checks were run.

### Explicitly not performed
- No source-code, Rust, Gradle, build-workflow, or configuration changes.
- No SteelMC source or submodule-pin changes.
- No implementation, cherry-pick, merge, PR, APK generation, or publication.
- No changes to worlds, profiles, or player data.
- Phase 2.2-B remains unauthorized.

### Next action
All investigative chats should use the new handoff template and send pending programming work to the dedicated programming chat. That chat must independently verify state and obtain/confirm user authorization before implementing each task.


## 2026-10-30 — Session 006: Implement Phase 2.2-B1–B4 profile-store foundations

### Authorization and scope
- The user explicitly authorized code edits to correct and integrate findings from the parallel investigation chats.
- Work remained on `unofficial-android-menu-access`.
- The experimental branch `phase-2.2-b-persistent-local-profile` was inspected as reference material. It was not merged or cherry-picked wholesale.
- SteelMC, authentication implementation, worlds, player-data files, `master`, PR merge state, and APK distribution were left untouched.

### Repository checks
- Branch HEAD before source edits: `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`.
- SteelMC gitlink: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- Branch changes since the preceding continuity audit were documentation-only.
- The experimental branch had 22 commits ahead and 22 behind the authorized branch at the time of comparison; its code delta was confined to `pomme-client/src/user.rs` and `pomme-client/src/lib.rs`, plus experimental documentation.
- The experimental implementation had no successful build/test/CI evidence. It was therefore treated as unverified reference code.

### Findings corrected during integration
- The experimental loader recovered a backup only when the primary file was missing, not when it existed but was corrupt.
- Its backup writer copied the primary file without validating the bytes, potentially replacing a good backup with a corrupt primary modified after load.
- It did not reject case-insensitive duplicate names in already serialized versioned profiles.
- Future schema versions were reported as generic invalid data, making downgrade protection indistinguishable from corruption.
- The runtime player UUID still derives from the username; the new field is not yet wired into SteelMC. This remains C/D work.

### Files changed
1. `pomme-client/src/user.rs`
   - Added independent `player_uuid` and versioned JSON representation (reference implementation).
   - Preserved historical offline UUID for legacy profiles while retaining profile-record and selected-profile IDs.
   - Added strict validation for malformed/future documents, duplicate profile IDs, player UUIDs, and case-insensitive names.
   - Added backup recovery for missing or corrupt primary files; corrupt primary is quarantined rather than discarded.
   - Prevented a corrupt primary from overwriting the backup by validating the exact bytes before backup publication.
   - Added regression tests for corrupt-primary recovery/preservation, future-version downgrade prevention, and case-insensitive duplicate names.
2. `pomme-client/src/lib.rs`
   - Handles profile-store load failure by logging and exiting instead of silently proceeding with replacement defaults.

### Commits
- `4bd084c8a9fca176da2545b800a8a2eb24c7b3e9` — profile store implementation and tests.
- `334f5f958bf99a61bc3d67bcc48036b88122c41f` — startup error handling.
- `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` — test assertion cleanup.

### Verification
- GitHub contents API confirmed each source file update and returned the commits above.
- Branch tree confirmed the SteelMC gitlink remained unchanged.
- CI run `41008897990` for source HEAD `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` was pending at the last check.
- CI run `41008828793` targeted the previous source commit and was still in progress at the last check; do not substitute it for the final run.
- No local Rust compiler, Cargo, rustfmt, Android build, or Android device execution was performed in this session.
- New tests are written but are **not yet verified as passing**.

### Remaining risks / next steps
- Inspect final CI jobs and logs; fix any failures on the authorized branch.
- Review the complete diff after CI.
- Verify all `LocalProfileStore::load_or_create` call sites and compile configurations.
- Consider concurrent multi-process access/locking as a separate follow-up; this implementation does not claim cross-process locking.
- Do not connect `player_uuid` to the embedded connection until the trusted per-connection design in C/D is reviewed.
- No claim is made about advancement persistence; SteelMC's existing TODO remains separate.


## 2026-10-30 — Session 007: Audit parallel-chat handoffs and verify Phase 2.2-B follow-up

### Scope
Continue the investigations left by the other Pomme chats, reconcile them against current source, and hand pending work to the dedicated programming chat. No source-code implementation was performed by this investigation chat.

### Repository verification
- Repository: `sylphie07/Pomme-Android-Test`.
- Branch: `unofficial-android-menu-access`.
- Branch HEAD before this log update: `b6ac729ab9e9f02e8ca0693c0d8d281b8042b0f7`.
- Latest source implementation commit inspected: `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce` — `fix(profile): preserve runtime UUIDs for new and experimental profiles`.
- Source files inspected include `pomme-client/src/user.rs`, `pomme-client/src/lib.rs`, `pomme-client/src/app/phases/in_menu.rs`, `pomme-client/src/ui/menu/account.rs`, `pomme-client/src/singleplayer.rs`, `pomme-client/src/account.rs`, `pomme-auth/src/lib.rs`, Android Cargo configuration, and CI/APK workflow files.
- SteelMC pin remains documented as `e655880a7b1487b2444c70013c14ee86c69d53b3`; no SteelMC source or pin was changed. The current compare showed no submodule path change since the previous audited point.
- PR #11 remained open; no review comments or approvals were found.

### Findings
- Menu navigation is not globally blocked by authentication. `authorize_gameplay()` centralizes the observed `Connect`, `PlayWorld`, `open_world()`, and QuickPlay gates. Local singleplayer is permitted without Microsoft authentication; remote multiplayer requires a verified, unexpired session.
- Startup QuickPlay routes through `in_menu::quick_play()` after setup. Focused tests for restore/cancel races and exactly-once pending-action resumption are still missing from `in_menu.rs`.
- B1–B4 profile-store safety implementation exists, including versioned JSON, legacy migration, duplicate validation, backup recovery, quarantine, and regression tests.
- A new-profile UUID continuity risk was found and then addressed by the programmer chat in commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`: new profiles now use the historical offline UUID, the document format advances to version 2, experimental format-1 random UUIDs migrate back to the runtime UUID, and duplicate historical UUID creation is rejected. This is applied in source but not yet CI-verified.
- Runtime `UserData::from_local_profile()` still derives the active UUID from the username. Trusted identity delivery to SteelMC remains Phase 2.2-C/D; those phases must consume the exact stored historical UUID and prove data continuity.
- Profile management UI is not integrated; list/create/select APIs and `set_local_profile()` remain unused/reserved.
- Token refresh during a long-running session and Android Keystore protection remain TODOs.
- Android singleplayer feature is enabled and an ignored embedded-world enter/re-enter test exists, but no Android-device acceptance result was obtained. The APK workflow does not automatically build this feature branch on PRs.
- Advancement persistence remains a separate SteelMC limitation.

### CI inspection
- Run `41009018315` (run #63) on the pre-UUID-fix source failed `ci-fmt`; its test/singleplayer work was superseded/cancelled.
- Latest relevant run `41009217544` (run #70) targets source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`.
- At the latest poll, run #70 had `test` and `singleplayer` cancelled; Android, `fmt`, and Ubuntu/macOS lint were still in progress; iOS and Windows lint were cancelled. No complete successful CI result is established.
- No local `cargo test`, `cargo fmt`, Android build, or Android device test was run by this investigation chat.

### Documentation changes
- Updated `docs/KNOWN_ISSUES.md` with current source status, CI results, UUID compatibility resolution, profile UI gap, missing auth tests, token/Keystore TODOs, APK workflow scope, and advancement persistence.
- Updated `docs/CHAT_HANDOFF_RECOMMENDATIONS.md` with prioritized tasks for the dedicated programming chat and the corrected B1–B4/C/D boundary.
- Updated `docs/DECISIONS.md` and `docs/PROJECT_CONTEXT.md` to match current implementation and profile format version 2.
- Updated `docs/CONTINUITY.md` with this audit, source commit, current CI state, and next safe actions.
- Documentation commits from this audit: `d54f2c704d94c114c58b1a4f90a3d1f30d17c98a`, `8f18d48d8a4a2e4108325b91cecb17e3f21928dd`, `6238ec2df2d4cfef0264041409c9784a54efb1ac`, `309a583548095faf1870a3a6c07563e2e08f16b5`, `cfbf5cb0fd92fe660ac8aefb86945e745763d381`, `b1c6e1cdca7061bc51ea0ee63836e088d05da9a5`, `b55664d23fa77bb6830bb47e221be3a91e761c73`, `03a067a940064a43a8497da21441976531317671`, `df0305e3d9d10b1efcbf7f993b792ca40924bbab`, and `b6ac729ab9e9f02e8ca0693c0d8d281b8042b0f7`.
- No application source, build configuration, workflows, SteelMC, worlds, profiles, player data, APKs, or `master` were changed by this chat.

### Next actions
1. Dedicated programming chat: obtain a complete CI run on the latest source; fix remaining formatting/test/build failures and inspect all logs.
2. Review B1–B4 source diff and tests after CI; do not treat tests written as tests passed.
3. Only after approval, plan C/D trusted integrated identity using the exact stored UUID and tests for all UUID-keyed player data.
4. Keep profile picker UI, auth token lifecycle/Keystore, Android device discrepancy, and advancement persistence as separate tasks.
5. Do not merge, update SteelMC, publish, or generate an APK without explicit authorization.


## 2026-10-30 — Session 007: Resolve new-profile UUID compatibility before C/D

### Trigger
A parallel investigation chat identified that the experimental B1–B4 implementation created random `player_uuid` values for new profiles while the runtime still used `offline_uuid(username)`. Wiring those random values into SteelMC later could make existing player records appear missing.

### Changes applied
- `LocalProfile::new()` now initializes `player_uuid` with the same historical offline UUID the runtime currently uses. The independent profile-record `id` remains a random UUID.
- `create_profile()` rejects a player-UUID collision instead of silently assigning a different UUID that would split an existing identity.
- JSON format advanced from version 1 to version 2.
- Experimental format-1 documents migrate their previously unused random `player_uuid` to `offline_uuid(username)`. This is safe for the inspected experimental implementation because its runtime continued using the name-derived UUID; the stored random field was not yet consumed by SteelMC.
- Added a regression test that checks format-1 migration preserves profile ID and selected-profile ID while replacing the unused random UUID with the historical runtime UUID.
- Applied rustfmt layout corrections after CI logs showed formatting differences in the earlier B1–B4 code.

### Files changed
- `pomme-client/src/user.rs` only.

### Commits
- `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce` — UUID compatibility and format-1 migration.
- `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc` — rustfmt layout cleanup.

### Verification
- GitHub source commit and branch ref verified.
- SteelMC gitlink remained `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- Previous CI run `41009018315` was cancelled after newer commits superseded it. Its formatting output exposed concrete rustfmt changes; those were applied manually.
- Current CI run `41009337238` targets the PR merge ref with PR head `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc`; it was pending at the last check.
- No local `cargo fmt`, `cargo test`, Android build, or device test was run.

### Remaining boundary
- Runtime UUID still comes from username in `UserData::from_local_profile()`; the new field is not yet passed to SteelMC.
- C/D/E remain separate work. No SteelMC source/pin changes, world-file changes, merge, master modification, or APK publication occurred.


## 2026-10-30 — Session 008: Restore validated backup snapshots atomically

### Review finding
The backup recovery path decoded and validated the backup file, then reopened the path inside `restore_backup()` to copy it. A concurrent file replacement between those operations could cause bytes other than the validated snapshot to be restored.

### Change
- `restore_backup()` now accepts the already validated byte slice.
- Both corrupt-primary and missing-primary recovery pass the exact bytes decoded by the caller.
- The temporary recovery file is written from that byte slice, synced, and then published using the existing atomic/rollback path.
- No world, profile, or player data was manually edited; this changes only the profile-store recovery implementation.

### File changed
- `pomme-client/src/user.rs`

### Commit
- `c0576191407a1ad53db77c0635c7b0820669e547` — `fix(profile): restore the exact validated backup snapshot`.

### Verification
- GitHub confirmed the source commit on `unofficial-android-menu-access`.
- SteelMC gitlink remained unchanged at `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- No local Rust command or Android test was run.
- CI for this latest source commit has not yet been confirmed; earlier runs for superseded commits were cancelled.

### Remaining
- Obtain complete CI results for the latest source commit, including fmt, lint, unit tests, Android, and singleplayer.
- Inspect all job logs before declaring B1–B4 verified.


## 2026-10-30 — Session 009: Fix CI-discovered profile-store integration errors

### Scope
Close concrete CI failures in the already authorized Phase 2.2-B1–B4 implementation. No C/D/E implementation, SteelMC change, merge, release, or APK work.

### Repository checks
- Confirmed live branch `unofficial-android-menu-access` at `f7ae3ce885b4e0e3d1c50f866201351d9549ef68` before edits.
- Confirmed `master` at `a147ca27e24850e5d6e796e1dc7bd02e1ecf3a72`.
- Confirmed `third_party/SteelMC` gitlink at `e655880a7b1487b2444c70013c14ee86c69d53b3` and `.gitmodules` URL unchanged.

### Investigation
- Run `41009217544` was cancelled and was not a passing validation.
- Run `41009743471` (#84) completed with failure. Its job logs showed missing test-module import of `LOCAL_PROFILE_FORMAT_VERSION`, an unhandled `Result` at the headless profile-store call site, and two rustfmt differences.
- The same call-site/type errors prevented lint, tests, Android/iOS clippy and singleplayer checks from compiling successfully.

### Changes applied
- `pomme-client/src/user.rs`: imported the schema version constant into the nested test module and applied the two rustfmt changes explicitly reported by CI.
- `pomme-client/src/headless.rs`: handle profile-store loading failure explicitly; log and exit nonzero rather than dereferencing a `Result` or continuing with a missing profile.

### Commits
- `bcca539e67e85959e99ce7bf39ce000065e02ffa` — test import and formatting correction.
- `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e` — headless error handling.

### Verification
- GitHub confirmed both commits and the current branch HEAD `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e`.
- New workflow runs `41010632204` and `41010632271` for that HEAD were pending at the last poll. Runs `41010627093` and `41010626979` for the preceding commit were in progress.
- No corrected-HEAD CI result is yet complete/green.
- No local formatter/test/build command or Android device test was run.
- Therefore source corrections are implemented, but tests are not yet verified as passed.

### Next
Poll the current-HEAD workflow runs, inspect all job logs, then review the full B1–B4 diff and document a trusted server-side identity handoff design for future C/D. Do not implement C/D or alter SteelMC without separate approval.


## 2026-10-30 — Session 010: Prepare trusted integrated-server identity design

### Scope
Prepare the C/D technical handoff without implementing C/D or changing SteelMC. Close the source review of the trust boundary and record required tests.

### Verified source facts
- Pomme `in_menu::open_world` has the selected `core.user.local_profile` available when starting the integrated server.
- `pomme-singleplayer::LaunchOptions` currently has no trusted player identity field; the embedded server is configured offline-mode with encryption disabled.
- SteelMC at `e655880a7b1487b2444c70013c14ee86c69d53b3` derives offline identity from the requested username in `steel-login/src/handlers/login.rs::handle_hello`; its source explicitly notes the hello UUID is client supplied.
- Steel player data and stats are keyed by the server-side `GameProfile.id()` in `FilePlayerDataStorage`.
- Thus a client hello UUID is not a trusted channel, even over the in-memory transport.

### Documentation created
- `docs/PHASE_2_2_C_D_TRUSTED_IDENTITY_DESIGN.md` — proposes a typed in-process Rust handoff from the persisted local profile into a per-connection trusted context in the embedded Steel server. Ordinary TCP connections receive no trusted context; online mode continues Mojang authentication; offline client UUIDs remain ignored.
- Includes compatibility invariants, Steel/Pomme regression tests, end-to-end existing-world acceptance, and boundaries for profile selector, Keystore, advancement persistence, and Phase 2.2-E.

### Commit
- `54a673e5138e8e1cf843a72d527fec8ad644a9ea` — design document only.

### Verification and boundaries
- This is a design artifact, not an implementation or runtime validation.
- No SteelMC source or gitlink, Pomme C/D source, remote auth behavior, profile selector UI, Keystore, advancement persistence, world data, or APK was changed.
- Separate explicit authorization is still required for C/D and any SteelMC modification.
- Rust CI for the corrected B1–B4 source HEAD is still not confirmed. Auxiliary conflict/label workflows were explicitly excluded from CI validation.


## 2026-10-30 — Session 009 follow-up: Apply second rustfmt report

- CI rerun attempt 2 of `41010633965` reached the formatter and reported two additional rustfmt-only differences: layout of the headless `load_or_create` expression and import ordering in the `user.rs` test module.
- Applied the exact requested formatting:
  - `1d7256ec025dd184f9a910aad724d9468341191d` — headless formatting.
  - `406a3d63db222f897338047dae40419baed18db1` — test import ordering.
- The run was superseded by these new source commits and is not a green validation. A fresh complete CI execution is still required.
- No semantic behavior, authentication policy, or SteelMC source/pin was changed in this follow-up.


## 2026-10-30 — Session 011: Add missing profile-store invariant tests

### Review finding
The implementation rejected duplicate profile IDs, duplicate player UUIDs, and a selected-profile ID that did not exist, but those individual rejection branches did not all have direct regression tests.

### Change
Added tests in `pomme-client/src/user.rs` for:
- Duplicate profile-record IDs.
- Duplicate player UUIDs across two different valid names.
- A selected-profile ID absent from the profile list.

These tests directly cover identity collision and selection-integrity checks already implemented by `LocalProfileStore::decode_document`; no runtime behavior was changed.

### Verification status
- Tests are written, not yet confirmed executed.
- The prior CI run on source commit `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e` was superseded after CI found two additional rustfmt layout differences. Those exact differences were corrected in `1d7256ec025dd184f9a910aad724d9468341191d` and `406a3d63db222f897338047dae40419baed18db1`.
- A new complete CI run is required for this consolidated source/test commit.
- No SteelMC, authentication policy, world data, merge, release, or APK change.


## 2026-10-30 — Session 012: Fix invalid format-1 test fixture

- CI run `41011216161` (#92) reached test execution. `fmt`, Android, iOS, and Ubuntu/macOS/Windows lint passed; the Rust test job failed on `experimental_format_one_migrates_to_the_runtime_offline_uuid`.
- Log evidence: `LocalProfileStore::load_or_create` rejected fixture username `ExperimentalPlayer` because it exceeds the existing 16-character local-profile username limit. This was a malformed test fixture.
- Changed only the fixture and its expected-value references to `ExpPlayer`, preserving the production validator and migration behavior.
- Source commit: `8362a0b20b5627db1e66e4954f116165ab591b38`.
- Test execution for the corrected fixture is pending a new CI run. Singleplayer had not yet completed on run #92 at last observation.
- No production logic, SteelMC, authentication policy, C/D/E implementation, profile UI, Keystore, advancement persistence, merge, publication, or APK change.


## 2026-10-30 — Session 013: B1–B4 CI passed; authorize C/D programmer handoff

### CI result
- PR #11, branch `unofficial-android-menu-access`, HEAD checkpoint `dedebc588065782768665d80b9fc3f6ca3281044`.
- GitHub Actions run `41012173438` (#94) completed successfully.
- Successful jobs: `pr-template`, `changes`, `singleplayer`, `lint (ubuntu-latest)`, `test`, `android`, `ios`, `fmt`, `lint (macos-latest)`, `lint (windows-latest)`, and aggregate `ci`.
- `launcher` and `launcher-frontend` were skipped. No Android device acceptance is claimed.
- PR #11 remains open and unmerged; `master` remains untouched.

### Authorization and handoff
- The user explicitly authorized implementation of Phase 2.2-C (trusted identity in SteelMC) and Phase 2.2-D (Pomme plumbing), including necessary SteelMC submodule changes.
- Created `docs/PHASE_2_2_CD_PROGRAMMER_HANDOFF.md` with mandatory preflight, security/compatibility invariants, implementation order, tests, exclusions, and reporting requirements.
- Updated the C/D design and continuity documentation to supersede stale statements that C/D were unauthorized or blocked on B1–B4 CI.
- The live SteelMC gitlink was not independently verified in this documentation session. The programmer must inspect the actual tree entry and pinned commit before editing; do not assume the previously documented pin.

### Scope and boundaries
- This session changed documentation only.
- No Pomme or SteelMC source, submodule gitlink, build/workflow files, worlds, profiles, player data, APK, or `master` were changed.
- No merge, publication, or device test was performed.


## Session 012 — Phase 2.2-C/D preflight and SteelMC write-access blocker (2026-10-30)

### Authorization and scope
- The user explicitly authorized Phase 2.2-C (trusted per-connection identity in SteelMC) and Phase 2.2-D (Pomme plumbing), including required SteelMC submodule changes.
- This session completed the mandatory documentation and remote-state preflight. No source code was changed because the connected GitHub integration has read-only permission to the upstream SteelMC repository; creating a working branch there returned HTTP 403.

### Live repository state
- Pomme repository: `sylphie07/Pomme-Android-Test`.
- Authorized branch: `unofficial-android-menu-access`.
- Live HEAD at preflight: `c7749941e52510ab4ced755f542b9b3a2589c443`.
- PR #11: open, base `master`, head `unofficial-android-menu-access` at the same SHA; not merged. Do not merge or modify master.
- Actual `third_party/SteelMC` gitlink: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- `.gitmodules` points to `https://github.com/Steel-Foundation/SteelMC.git`.
- GitHub cannot establish the state of the user's local Termux worktree; no claim is made that it is clean.

### CI B1–B4 confirmation
- Workflow run `41012173438` (#94) is completed with overall conclusion `success` for PR head `dedebc588065782768665d80b9fc3f6ca3281044`.
- `pr-template`, `changes`, `singleplayer`, `lint (ubuntu-latest)`, `test`, `android`, `ios`, `fmt`, `lint (macos-latest)`, `lint (windows-latest)`, and aggregate `ci` succeeded.
- `launcher` and `launcher-frontend` were skipped by workflow filtering. This is automated CI evidence, not Android device acceptance. Current branch HEAD has subsequent documentation-only commits beyond the tested SHA.

### Confirmed source path
- Pomme `open_world()` calls `singleplayer::open(...)`, then creates `Transport::Memory`; `connect_args()` currently uses `core.user.uuid` for the memory connection.
- `pomme-singleplayer::LaunchOptions` currently has no trusted player identity. Its embedded server constructs `JavaTcpClient::from_transport(..., Arc::new(ServerConnectionSession::default()), ...)`.
- SteelMC at the pinned commit has a per-`JavaTcpClient` connection object, but `handle_hello()` deliberately ignores the Hello UUID and derives `offline_uuid(requested_username)` in offline mode. Online-mode login uses the existing encryption/Mojang verification path.
- The trusted context must be attached to the concrete `JavaTcpClient` instance, not `ServerConnectionSession`, and must be absent from the ordinary TCP constructor.

### Access blocker
- Repository metadata for `Steel-Foundation/SteelMC` reports connected-account permissions `pull: true`, `push: false`.
- Attempt to create branch `pomme-trusted-local-identity` from the pinned commit failed with HTTP 403: `Resource not accessible by integration`.
- Therefore SteelMC C cannot be committed or pinned from this connection. Pomme D depends on the new SteelMC API, so no incomplete Pomme source wiring was applied.
- Required unblock: create a fork at `sylphie07/SteelMC` (or grant the connected GitHub integration write access to an authorized SteelMC fork). Then create the implementation branch from exact upstream pin `e655880a7b1487b2444c70013c14ee86c69d53b3`, implement/test C there, and update the Pomme submodule URL/gitlink to that fork commit before D.

### Changes and verification in this session
- No application source, SteelMC source, gitlink, Cargo/build configuration, worlds, profiles, player data, master, PR merge state, or APK was changed.
- Read-only inspection only; no Rust tests, builds, or new CI were run.
- No implementation or test is claimed as complete for C/D.

### Next step
- User creates the writable SteelMC fork (no new implementation authorization is needed; authorization for C/D is already recorded). Resume with the exact pinned base, then implement C and its isolation/security tests before plumbing D. Preserve the historical UUID and Microsoft online authentication.


## 2026-10-30 — Phase 2.2-C/D implementation, CI #120, and final audit

### Objective
The user clarified the intended behavior: local singleplayer must work without signing in; only remote multiplayer requires Microsoft authentication. This is the intended Bedrock-like separation and is distinct from bypassing authentication on remote servers.

### Implementation summary
- SteelMC trusted identity is implemented in the `sylphie07/SteelMC` fork and pinned at `c5098cdd8a4ec1176e00d737df24a2861b4055ac`.
- SteelMC changed files relative to the previous pin `e655880a7b1487b2444c70013c14ee86c69d53b3`:
  - `steel-login/src/tcp_client.rs`: typed `TrustedLocalPlayer` and explicit embedded constructor; identity is held per `JavaTcpClient`, never on the shared listener session. Normal TCP constructors leave it absent.
  - `steel-login/src/handlers/login.rs`: exact username match and trusted UUID for embedded offline login; mismatch rejection; ordinary offline UUID fallback; trusted context forbidden in online mode; regression tests.
  - `steel-login/src/lib.rs`: public re-export for the typed embedded API.
- Pomme wiring:
  - `pomme-client/src/app/phases/in_menu.rs`: selected persisted profile username/UUID passed to the integrated world; memory `ConnectArgs` uses that same UUID; remote args still come from `multiplayer_identity()`.
  - `pomme-client/src/singleplayer.rs` and `pomme-singleplayer/src/lib.rs`: explicit launch fields carry the identity into the embedded server connection.
  - `pomme-client/src/headless.rs`: headless local-world launch also carries the local profile UUID.
  - `pomme-client/src/net/connection.rs`: after memory login, the server-returned profile UUID must equal the expected persisted UUID.
  - `.gitmodules`, `Cargo.lock`, `pomme-singleplayer/Cargo.toml`, and `justfile` update the fork pin, UUID dependency, and focused Steel login validation.
- Key Pomme commits include `cb934b2f98a9582c2a7a6a2c62cf7513a5e4695f` (bind trusted identity to embedded connection), `a6551a823b3ead87809fd375e8692e84f2ea13df` (launch options), `36f6f2ddf97b6690a169eb4c6124fb291123e84b` (menu/local profile plumbing), `d141a82e91cef91d46acb909f6e1350f2e01a2ae` and `164ea8015ae720c0d42b96b232bd0c1e18071fe9` (connection/singleplayer tests), and final pin/lint commit `e902fa1cb733edabd1349877dd49b3d8a8a74d42`.
- SteelMC C changes comprise 14 commits relative to the prior pin; final gitlink is `c5098cdd8a4ec1176e00d737df24a2861b4055ac`.

### Audit and regression results
- Exact persisted `LocalProfile.player_uuid` is passed unchanged through `open_world`, `LaunchOptions`, `TrustedLocalPlayer`, and the server-side offline `GameProfile`. The hello packet UUID is not the trust source.
- A trusted identity is tied to one embedded `JavaTcpClient`; standard TCP construction provides `None` and preserves `offline_uuid(username)`.
- Username mismatch is rejected. Two trusted identities resolve independently.
- Trusted local identity cannot be used with online mode. Remote Pomme connections still require a verified, unexpired Microsoft session and use its UUID/token; the server's Mojang verification path remains intact.
- B1–B4 historical UUID migration ensures existing world/player-data keys are not rewritten or replaced.
- No concrete implementation defect was found in the final audit; no additional source edit was needed.

### Automated validation
- Branch HEAD at audit: `e902fa1cb733edabd1349877dd49b3d8a8a74d42`.
- PR #11: open, targeting `master`, unmerged.
- CI #120, run `41020133603`, overall success for that exact HEAD. The branch HEAD equals the tested SHA; no later source changes require another run. The subsequent continuity/development-log commits are documentation-only. Do not poll or rerun Actions solely to refresh the result.
- Regression tests included in the CI-validated source cover trusted identity resolution, username mismatch, per-identity isolation, online-mode guard, normal offline UUID derivation, integrated singleplayer login/enter-reenter, memory-connection UUID consistency, and profile/account authorization invariants.
- No local Rust commands, APK build, or Android-device validation were performed in this audit.

### Remaining limitations / separate work
- Phase 2.2-E: verify an existing world on the actual Android device, confirming inventory, position, stats, and save/reopen continuity. Preserve all user data.
- Profile picker UI is not integrated; the underlying store supports multiple profiles.
- Android Keystore/token refresh and advancement persistence remain separate tasks.
- Non-blocking test-hardening opportunities: add a wire-level conflicting-Hello-UUID test and a constructor-level TCP non-inheritance test. Source paths currently enforce the intended separation; no defect was observed.

### Scope boundaries
No merge of PR #11, no changes to `master`, no APK build/publication, and no destructive world/profile/player-data operation.


## 2026-10-30 — Session 014: Fix Release singleplayer login gate

### Scope and preflight
- Repository: `sylphie07/Pomme-Android-Test`.
- Authorized branch: `unofficial-android-menu-access`; `master` was not targeted.
- Re-read `pomme-client/src/user.rs`, `docs/CONTINUITY.md`, and this development log before editing.
- The Release branch of `UserData::for_launch()` returned `true` as its second tuple value even though the local integrated server is offline-mode and `allows_singleplayer()` already permits local play. This incorrectly marked sign-in as required at startup.

### Changes applied
- `pomme-client/src/user.rs`: Release now returns `false` for the login-required flag while continuing to initialize from the persisted local profile.
- Updated the Release regression test to assert that login is not required, the exact local profile is retained, singleplayer is allowed, no Microsoft authorization is fabricated, and multiplayer remains denied without a verified Microsoft session.
- No SteelMC code/gitlink, world/profile files, account store, authentication verification, or remote multiplayer gate was changed.

### Verification status
- Source edits were submitted to the authorized GitHub branch; commit SHA is recorded in the continuity note after the source write.
- The regression test is updated but has **not** yet been executed in this session. No CI pass, APK build, or Android-device result is claimed.
- No merge, release, APK generation, or change to `master` was performed.

### Next step
- Run the focused Rust test and the repository CI on this source commit. After automated checks pass, build/install a new APK only under separate authorization and repeat the signed-out singleplayer device test.


## 2026-10-30 — Session 015: Diagnose world creation returning to list

### User-reported behavior
- The user tested the latest APK after the Release Singleplayer authentication correction.
- Signed-out Singleplayer now works and a new world is created, but Pomme returns to the world-selection screen instead of entering the new world. Manually selecting the new world and pressing Play successfully enters it.

### Repository preflight
- Repository: `sylphie07/Pomme-Android-Test`.
- Branch: `unofficial-android-menu-access`.
- Inspected branch HEAD: `730c08c307a9aceaef9241a52dd9265be7f58c34`.
- PR #11 remained open and unmerged.
- SteelMC gitlink at the inspected tree: `c5098cdd8a4ec1176e00d737df24a2861b4055ac`.
- Re-read continuity, development log, and parallel-chat handoff guidance before investigation.

### Investigation and confirmed source cause
- Inspected `pomme-client/src/ui/menu/worlds.rs`, `pomme-client/src/ui/menu/mod.rs`, `pomme-client/src/ui/world_list.rs`, and `pomme-client/src/app/phases/in_menu.rs`.
- `MainMenu::build_create_world()` calls `create_world()` on the Create New World button.
- `MainMenu::create_world()` creates a `WorldSummary`, persists it with `WorldList::create()`, and unconditionally switches to `Screen::WorldList`. It does not return a `MenuAction::PlayWorld` action.
- The existing `MenuAction::PlayWorld { folder }` handler in `update_menu()` calls `open_world()`, which performs the existing Singleplayer authorization, resolves the world folder, starts the integrated server, and connects through the in-memory transport.
- Conclusion: the create handler is missing the transition into the existing PlayWorld path. This is a source-confirmed explanation matching the user report; no Android/Docker/authentication defect is implicated by this particular flow.

### Recommended change and handoff
- On successful persistence, return `MenuAction::PlayWorld` using the exact generated `summary.folder` (not the display name) so duplicate-name suffixes are handled safely.
- On persistence failure, do not attempt launch; remain in a safe menu state and report the error.
- Preserve the existing central auth/gameplay gate; do not bypass `open_world()` or alter remote multiplayer policy.
- Add focused regression tests for empty/non-empty save lists, duplicate names, persistence failure, manual PlayWorld, QuickPlay, signed-out Release Singleplayer, and protected remote multiplayer.
- The detailed 11-point handoff was appended to `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`.

### CI and verification
- Rechecked CI run ID `41032914173` (#125): all listed jobs, including Android, singleplayer, tests, fmt, platform lint, and aggregate CI, completed successfully.
- This CI run predates the proposed auto-launch change and does not verify it.
- No local tests, builds, formatting commands, or independent device reproduction were performed in this investigation.

### Scope
- Documentation/handoff only. No source code, workflow, SteelMC, world/profile data, APK, or `master` changes.
- No source implementation was performed; the dedicated programming chat remains implementation owner.


## 2026-10-30 — Session 016: User-device confirmation of per-world inventory isolation

### User-reported Android acceptance
- The user tested the current APK that still exhibits the separate world-creation auto-launch issue.
- The user explicitly confirms that each existing world retains its own independent inventory.
- This is positive device-level evidence that inventories are not being shared or cross-contaminated between worlds in the tested APK.
- Scope of the evidence: per-world inventory separation only. The user did not report checking player position, stats, Ender Chest, all other player-data fields, or a complete close/reopen/save cycle in this message; do not claim those are verified.

### Impact on Phase 2.2-E
- Mark existing-world inventory isolation as **user-tested / passed for the tested APK**.
- Phase 2.2-E remains partially open for broader save continuity checks (position, stats, Ender Chest and clean save/close/reopen), without deleting or migrating user data.
- The world-creation auto-launch defect remains separate and unresolved in source: the newly created world is saved, but the UI returns to the world list instead of automatically entering it.

### Repository / CI checkpoint
- Repository: sylphie07/Pomme-Android-Test; authorized branch: unofficial-android-menu-access.
- PR #11 remains open and unmerged; master remains unchanged.
- Current PR head observed: 03d17d2d7f7baba484b125b6102c4b195e707750.
- CI run #126 (41108688700) for the observed PR head completed successfully, including Android, tests, singleplayer, fmt, lint, and aggregate CI.
- CI success is automated validation and is distinct from the user's APK/device test. Do not infer that CI specifically covers the unimplemented world auto-launch change.

### Scope of this session
- Documentation-only update recording the user's device result. No source code, SteelMC pin, world/profile data, APK, merge, or master change was made.

### Next step
- Keep the auto-launch repair as the immediate programming task. Preserve this successful per-world inventory result as a regression acceptance criterion. After the code fix, rebuild only with the applicable authorization and verify that creating a world enters it automatically while existing worlds still retain separate inventories.


## 2026-10-30 — Session 017: User-device confirmation of position and death/respawn behavior

### User-reported Android acceptance
- The user tested the current APK and confirmed that re-entering a world places the player at the exact location where they left it.
- The user also confirmed the expected death flow: the player loses the applicable carried items and respawns at the world spawn.
- Record these as **user-tested / passed** for position persistence across world reload and the reported death/respawn behavior on the tested APK.
- This is user-reported device evidence, not an independently reproduced test or CI result. The report does not separately identify statistics fields, Ender Chest behavior, or a complete app-force-stop/restart cycle; do not claim those are verified.

### Phase 2.2-E impact
- Per-world inventory isolation: previously user-tested / passed (Session 016).
- Position on world re-entry: user-tested / passed (this session).
- Death item-loss and spawn respawn behavior: user-tested / passed (this session).
- Statistics persistence and Ender Chest continuity remain unconfirmed. Keep the remaining checks non-destructive and preserve all worlds/player data.

### Repository preflight and scope
- Repository: sylphie07/Pomme-Android-Test.
- Authorized branch: unofficial-android-menu-access.
- PR #11 is open and unmerged, targeting master. Live PR head at preflight: 4fd37ba485adbeef54759841854bc6c0498b97ad.
- Re-read docs/CONTINUITY.md, docs/DEVELOPMENT_LOG.md, and docs/KNOWN_ISSUES.md before this documentation update.
- Documentation-only change. No source, workflow, SteelMC gitlink, world/profile/player data, APK, master, or PR merge changes.
- No automated tests or new CI run were performed by this documentation update.

### Next step
- Continue with the separately diagnosed world-creation auto-launch fix. Preserve inventory isolation, position continuity, and death/respawn behavior as device regression criteria. Statistics and Ender Chest remain to be checked if the user can do so safely without deleting or resetting data.


## 2026-10-30 — Session 018: Ender Chest and entity implementation status from device test

### User-reported Android acceptance
- The user tested the Ender Chest on the current APK and confirmed that it works.
- The user also reports that passive animals such as cows, pigs, and sheep now appear in the world.
- Hostile mobs do not appear. The user attributes this to missing Minecraft Rust implementations, not to the Pomme world/player-data persistence path.
- Record Ender Chest behavior as **user-tested / passed** on the tested APK. Record passive-entity visibility and missing hostile mobs as a gameplay implementation observation, not as a Pomme persistence defect. The attribution to Minecraft Rust is the user's assessment and has not been independently source-audited in this session.

### Phase 2.2-E impact
- Per-world inventory isolation: user-tested / passed (Session 016).
- Position continuity on world re-entry: user-tested / passed (Session 017).
- Death item-loss and spawn respawn behavior: user-tested / passed (Session 017).
- Ender Chest: user-tested / passed (this session).
- Statistics persistence and full force-stop/restart lifecycle remain unconfirmed.
- Passive animals render/spawn according to the user's observation; hostile mob implementation remains outside the Pomme persistence scope unless a separate reproducible Pomme-specific defect is found.

### Scope and verification
- This entry records user-reported Android behavior only; no independent runtime reproduction, source inspection of Minecraft Rust mob systems, automated test, or CI run was performed here.
- No Rust source, SteelMC gitlink, workflow, APK, world/profile/player data, master branch, or PR merge was changed.
- Next programming priority remains the separately diagnosed world-creation auto-launch fix. Keep the verified persistence behaviors as regression criteria.


## 2026-10-30 — Session 019: User confirmation of statistics, XP and progression

### User-reported Android acceptance
- The user clarifies that statistics are working correctly on the tested APK.
- XP behavior and progression are also confirmed: collecting XP grants it to the player, death causes the expected XP loss, and progression behaves correctly.
- Record statistics and XP/progression behavior as **user-tested / passed**, based on the user's device report. Do not infer additional untested statistic categories or claim an independently reproduced test.

### Scope clarification — Minecraft Rust entities
- The absence of hostile mobs is not part of the current Pomme programming task. The user clarifies that the APK is based on a Minecraft Rust version that still lacks substantial gameplay implementations.
- Keep hostile-mob availability as an upstream Minecraft Rust gameplay limitation/observation, not a Pomme defect or Phase 2.2-E persistence failure. No Rust source diagnosis or change was performed.

### Phase 2.2-E impact
- User-tested / passed: per-world inventory isolation; position continuity on world re-entry; death/item-loss/spawn-respawn behavior; Ender Chest; statistics; XP collection/loss and progression.
- Still outstanding: a complete non-destructive force-stop/restart save lifecycle and any player-data fields not explicitly covered by the user's tests.
- Next Pomme programming task remains the separately diagnosed create-world auto-launch repair.

### Repository and scope
- Preflight: PR #11 remains open and unmerged, targeting master; authorized branch is `unofficial-android-menu-access`; live PR head before this documentation update was `a566f1ec613e82afa479eeadfbca3a78262ad27d`.
- Documentation-only update. No source, Minecraft Rust/SteelMC code or pin, APK, workflow, worlds, profiles, player data, master, or merge was changed. No automated test or CI result is claimed.



## 2026-10-30 — Session 020: Implement create-world auto-launch correction

### Authorization and preflight
- The user explicitly authorized proceeding with the remaining Pomme fixes and preparing a new APK.
- Repository: `sylphie07/Pomme-Android-Test`; authorized branch: `unofficial-android-menu-access`.
- Preflight branch HEAD: `87e00c5b6640c0b5d224d4de12e51b80dd6d0aea`; PR #11 open/unmerged, base `master`; no changes to `master`.
- Re-read project context, continuity, development log, decisions, known issues, handoff recommendations, create-world source, world-list persistence, central menu action handling, CI workflow and Android APK workflow.
- Current PR head after implementation: `546ec38f7d07d1784a0951cbfbb437088ffedf01`.

### Source change applied
- File: `pomme-client/src/ui/menu/worlds.rs`.
- `build_create_world()` now carries a `MenuAction` result and returns the action produced by the Create New World button.
- `create_world()` now returns a menu action instead of switching unconditionally back to `Screen::WorldList`.
- New `persist_created_world()` persists the summary first, captures the exact folder identifier, and returns `MenuAction::PlayWorld { folder }` only after `WorldList::create()` succeeds.
- The existing `in_menu::update_menu()` → `open_world()` route performs the actual launch and preserves the existing Singleplayer authorization gate, world resolution, embedded server start, and memory transport. No authentication policy was bypassed.
- If persistence fails, the helper logs the error and returns `MenuAction::None`; the create screen remains active and no launch is requested.
- Three regression tests were added: exact folder on successful creation; duplicate display name resolves to the generated suffixed folder; persistence failure emits no PlayWorld action and preserves the blocking file.
- No change to `WorldList::create()`, profile identity, UUIDs, remote multiplayer, SteelMC, or player/world data.

### Verification state at log time
- GitHub branch ref and PR #11 were re-read after source changes: head `546ec38f7d07d1784a0951cbfbb437088ffedf01`, PR open/unmerged.
- CI run #134 / ID `41126486791` was created for this head and was pending.
- Android APK workflow run ID `41126478882` was observed for the same head and was in progress.
- The three new tests are present in source but have not yet been confirmed executed. No local `cargo test`, `cargo fmt`, build, or device test was run in this chat.
- The prior CI pass #125 predates this fix and is not evidence for it.

### Scope and next steps
- Source-only application fix plus tests; build workflow is automatically triggered by the authorized branch push. APK artifact is not yet confirmed.
- Preserve all existing user worlds, profiles, and player data. Do not ask the user to reset or delete data.
- After CI and APK workflow finish, inspect their final job outcomes and artifact. Then user-device acceptance should verify: create world enters it immediately; duplicate names launch the new folder; existing worlds remain intact and retain independent inventory, position, death/respawn, Ender Chest, stats and XP/progression behavior.
- PR #11 remains open/unmerged. No release or merge performed.
