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
