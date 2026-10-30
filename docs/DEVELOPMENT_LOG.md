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
