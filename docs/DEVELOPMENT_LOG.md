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
