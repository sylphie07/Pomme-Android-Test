# Pomme Android — Recommendations for Parallel Development Chats

Last updated: 2026-10-30

## Purpose and authority

This document is a shared review guide for other chats working on different parts of Pomme Android. It is intended to prevent duplicated work, regressions, and contradictory assumptions.

**These are recommendations and review prompts, not authorization to modify code.** Every chat must respect the user's latest explicit scope. Do not start a new phase, alter authentication, modify SteelMC, publish an APK, merge, or change branches without the required approval.

## Rules for every chat

1. Start by reading `docs/PROJECT_CONTEXT.md`, `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, this file, and any relevant phase report.
2. Verify repository, current branch, live HEAD, recent commits, and SteelMC gitlink before editing. Never assume a SHA from an old chat is still current.
3. Keep the work on the specifically authorized branch. Do not modify `master` or another chat's branch.
4. Inspect the current call graph and existing implementation before proposing a patch. Avoid solving only the visible UI button when other entry paths exist.
5. Separate confirmed source facts, user-reported behavior, hypotheses, proposed changes, applied changes, and verified results.
6. Add regression tests where practical. Report exactly which commands ran and their outcomes; do not imply that a test passed because code looks correct.
7. Do not overwrite user data, world data, profile files, save directories, or generated artifacts as part of cleanup.
8. At session end, update the continuity handoff and development log. Record files and SHAs, tests actually run, unresolved issues, and the next safe step.
9. Because the user works from Android/Termux without a desktop PC, prefer reproducible Android-compatible instructions and commands. Do not assume a desktop IDE or local PC is available.

## Chat area: menu navigation and authentication

Recommended review:
- Trace every route that can reach gameplay, not just the main Play button: `MenuAction::Connect`, `MenuAction::PlayWorld`, `open_world()`, quick-start arguments, and any alternate connection/startup route.
- Keep the menu browsable while signed out, but put the required authentication/authorization checks at the actual protected gameplay boundary.
- Distinguish local singleplayer from remote multiplayer. Do not accidentally require Microsoft sign-in for local offline worlds or accidentally allow unauthenticated remote play.
- Review reconnect, direct-connect, server-list, world-list, and command-line/quick-start paths for bypasses or duplicated checks.
- Add tests for signed-out menu navigation, local singleplayer, authenticated multiplayer, rejected unauthenticated multiplayer, and account switching.
- Avoid changing SteelMC generic offline login as a shortcut for menu access.

## Chat area: Android singleplayer and world creation

Recommended review:
- Preserve the Android `singleplayer` Cargo feature currently enabled in `pomme-android/Cargo.toml`; inspect dependency feature resolution before changing it.
- Separate failures in world creation, save-path selection, embedded-server startup, transport initialization, and Android filesystem permissions. Do not label all failures as a Docker issue without evidence.
- Test both a clean first launch and a subsequent launch with an existing world. Verify that worlds are listed and reopened after app restart.
- Record exact Android version, device, app build/commit, world path, and error logs for each report. The user's successful test on one phone is useful but does not establish compatibility across all Android devices.
- Check cold start, interrupted world creation, storage-full behavior, and recovery without deleting the user's existing worlds.
- Keep performance work separate from correctness fixes; capture a baseline (render distance, graphics settings, FPS/stutter observations) before optimization changes.

## Chat area: local profiles and persistent player identity — Phase 2.2

Recommended review:
- Phase 2.2-B1–B4 was explicitly authorized and has been implemented in source; do not repeat that work or treat it as fully verified until CI passes.
- Phase 2.2-C/D/E remain separate downstream phases and are not authorized by the B1–B4 decision.
- New-profile UUID compatibility was corrected in source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`: new profiles store the historical offline UUID, and experimental format 1 migrates to format 2 using that same UUID. Backup recovery now restores the exact validated snapshot in commit `c0576191407a1ad53db77c0635c7b0820669e547`. Verify CI before treating these corrections as accepted.
- Preserve `LocalProfile.id` as the profile-record ID. A new `player_uuid` must have its own lifecycle.
- For valid legacy profiles, the candidate migration is the offline UUID of the currently stored username; preserve the profile ID and selected-profile ID. Handle renamed-before-migration profiles as a recovery case, not a guess.
- Validate duplicate profile IDs, player UUIDs, and case-insensitive names without silently reassigning identities or merging data.
- Version the JSON format, make migration idempotent, define downgrade behavior, and preserve the last known-good file during interrupted writes.
- Keep local identity independent of Microsoft sign-in/out and authenticated Java identity.
- Do not trust client-supplied UUIDs from generic offline Hello packets.

## Chat area: SteelMC integration and persistence

Recommended review:
- Keep trusted local identity scoped to the concrete integrated connection. Do not put per-player identity in a shared listener-scoped `ServerConnectionSession`.
- Keep generic offline login behavior unchanged: a remote/offline Hello UUID is not a trusted identity source.
- Add tests proving that the integrated Pomme path can use its trusted identity while ordinary offline connections cannot spoof it.
- Verify every UUID-keyed store: global player data, per-domain player data, statistics, permissions, and known-player records.
- Preserve the same historical UUID for existing profiles wherever safely possible; this is what keeps existing player records discoverable without copying world files.
- **Advancement limitation:** at SteelMC pin `e655880a7b1487b2444c70013c14ee86c69d53b3`, `Player.advancements` is initialized in memory and marked `TODO save the advancement`; the inspected player-data persistence path does not save it. Track advancement persistence as a separate SteelMC issue with its own schema/versioning and compatibility tests. Do not claim that Phase 2.2 identity work fixes it.
- Do not update the submodule pin unless the user authorizes the SteelMC work and the Pomme integration has been reviewed against the new revision.

## Chat area: Android build, CI, and APK workflow

Recommended review:
- Verify workflow YAML, Android SDK/NDK/toolchain versions, Rust targets, Java/Gradle requirements, and artifact paths as a complete reproducible chain.
- Check whether CI builds the same feature set and target architecture that the user installs on-device.
- Report workflow run IDs, commit SHAs, job outcomes, and artifact availability. A workflow file existing is not evidence that a build passed.
- Keep APK signing, package identity, versioning, and update/install behavior explicit. Avoid producing an APK from an unverified or dirty source state.
- Test install/update behavior on Android without instructing the user to delete worlds or app data as a routine troubleshooting step.
- Before publishing or distributing any APK, obtain explicit user authorization.

## Dedicated chat: programming and implementation queue

The user has designated a separate chat exclusively for programming, applying fixes, and integrating work that other Pomme chats investigate. Treat that chat as the implementation owner, not as another parallel investigator.

### Responsibilities of investigation/review chats
- Do not edit source code, Rust/Gradle/build configuration, workflows, submodules, branches, or generated artifacts unless the user explicitly assigns that chat an implementation task.
- Investigate only within the assigned scope. When a code change is needed, prepare a handoff for the dedicated programming chat instead of implementing it.
- Before handing off, verify the relevant repository, branch, live HEAD, SteelMC gitlink (when relevant), and the exact source revision inspected.
- Record the issue with: priority; observed behavior; confirmed source evidence and file/function locations; distinction between facts and hypotheses; why a change is needed; proposed behavior; affected areas; compatibility/data risks; regression tests and expected outcomes; exact authorization status.
- Include dependencies and ordering. Mark blockers explicitly; do not ask the programmer chat to implement a downstream phase while its prerequisite is unresolved.
- Add the handoff to the relevant documentation and development log so it survives chat changes. Do not assume a verbal summary alone is enough.

### Responsibilities of the dedicated programming chat
- Read the repository continuity docs and every pending handoff before touching code.
- Re-check live branch/HEAD, worktree status where available, recent commits, and SteelMC gitlink. Never rely on stale SHAs or copy another chat's branch without approval.
- Maintain a prioritized implementation queue from the pending investigation handoffs. Resolve conflicts by asking the user or reporting the conflict; do not silently choose incompatible designs.
- Implement only tasks the user has authorized. A handoff is a technical recommendation, not permission by itself.
- Before each task, state the exact files/areas to be changed and the tests planned. Preserve user data and unrelated work.
- After implementation, report exact commits/files, tests actually run and their outcomes, remaining limitations, and any required Android/Termux validation. Update continuity and development logs.
- Do not claim a fix is complete merely because code was edited or tests were written. Distinguish implemented, locally tested, CI-verified, and device-verified states.
- Do not merge, publish, update SteelMC, generate/distribute an APK, or modify master unless the user explicitly authorizes that action.

### Required handoff template
Every investigation chat that leaves programming work pending should record:

1. **Task / priority**
2. **Authorization:** investigation only / implementation explicitly authorized / blocked pending approval
3. **Repository, branch, inspected HEAD, and SteelMC pin**
4. **Problem and impact**
5. **Evidence:** files, symbols, call paths, logs, or reproducible observations
6. **Confirmed facts vs. hypotheses**
7. **Recommended correction and alternatives considered**
8. **Compatibility, security, and user-data risks**
9. **Acceptance criteria and regression tests**
10. **Dependencies, blockers, and recommended order**
11. **What was not changed or tested**

### Current known programming handoffs
- **Phase 2.2-B1–B4 — persistent local profile identity:** implementation is explicitly authorized by the user on 2026-10-30 and has begun on `unofficial-android-menu-access`. Source changes add `player_uuid`, versioned JSON, historical UUID migration, strict duplicate validation, corrupt/missing-primary backup recovery, quarantine preservation, future-version downgrade protection, exact validated-snapshot restoration, and regression tests. CI and Android verification remain pending. The experimental branch is reference material only and was not wholesale-merged. The new field is not yet wired into SteelMC; C/D remain separate downstream phases.

- **B1–B4 CI verification:** run `41009018315` (run #63) failed `ci-fmt` on the pre-fix source and was superseded. Latest run `41009217544` (run #70), for commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`, was still running at the latest poll; test/singleplayer and the platform lint/Android/iOS/fmt jobs had started, but no final result was available. Wait for this run and inspect every job before calling B1–B4 verified.
- **P2.2-C/D identity continuity:** source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce` now initializes new profiles with `offline_uuid(username)`, bumps the profile document to format 2, and migrates experimental format-1 random UUIDs back to the runtime UUID. `create_profile()` rejects duplicate historical UUIDs rather than splitting identity. This addresses the previously identified new-profile mismatch in the profile store; C/D must still consume the exact stored UUID and prove world/player-data continuity before claiming completion.
- **Local-profile UI follow-up:** `LocalProfileStore` is loaded on startup, but `profiles()`, `create_profile()`, `select_profile()`, and `UserData::set_local_profile()` are marked unused/reserved. No player-facing profile picker/create/switch flow was found. Treat UI integration as a separate scoped task; do not imply multiple profiles are currently user-manageable.
- **Menu/authentication regression coverage:** `authorize_gameplay()` appears to centralize the menu's remote and local gameplay gates, and QuickPlay routes through it. However, no test module was found in `in_menu.rs`. Add tests for signed-out menu browsing, offline singleplayer, rejected unauthenticated remote play, valid/expired sessions, both startup QuickPlay paths, restore success/failure, cancel/back during restore, and exactly-once pending-action resumption.
- **Authentication lifecycle/security follow-up:** `pomme-client/src/account.rs` has a TODO for refreshing a day-long Minecraft token during a long-running session. Android stores credentials in app-private `accounts.json`, with an explicit Android Keystore TODO. Keep this separate from local-profile identity and preserve ownership checks.
- **Android singleplayer/packaging verification:** Android Cargo enables the `singleplayer` feature and an ignored embedded-world enter/re-enter integration test exists, but CI did not complete that job for the B1–B4 source revision and no current feature-branch APK artifact/device result was found. The APK workflow is triggered by pushes to `master` or manual dispatch, not PRs on this feature branch. Obtain reproducible device logs for external failures before proposing runtime changes.
- **SteelMC advancement persistence:** separate follow-up item at the recorded SteelMC pin; do not bundle it into identity work or change the submodule without authorization.
- Other chats should append newly discovered programming tasks here and in the development log, without editing code themselves.

## Suggested handoff prompt for any new chat

> Continue the Pomme Android task only within the scope previously authorized by the user. First read the repository documentation, verify the live branch/HEAD and SteelMC gitlink, inspect recent commits and the current code path, and read `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`. Do not assume earlier SHAs are current. Separate facts from hypotheses and proposals. Do not edit another branch, master, authentication, world/player data, or SteelMC unless explicitly authorized. Before changing code, state the exact intended files and regression tests. At the end, update continuity/development documentation and report exact changes and tests actually run.

## Current known boundary

Phase 2.2-A design is documented. Phase 2.2-B1–B4 implementation is authorized and in progress on the designated branch. Phases 2.2-C/D/E, SteelMC changes, merging, publication, and APK distribution remain separate and require their own explicit authorization. This document does not grant permission to implement any additional recommendation.


## Authorization update — 2026-10-30

The user's latest explicit instruction authorizes the dedicated programming chat to implement Phase 2.2-C and 2.2-D, including necessary SteelMC submodule changes. This supersedes older statements in this file that C/D are unauthorized or that B1–B4 CI is still pending.

B1–B4 CI run `41012173438` (#94) completed successfully for test, singleplayer, Android, iOS, fmt, Ubuntu/macOS/Windows lint, and aggregate CI. Launcher jobs were skipped. This does not constitute Android device acceptance.

The programmer must follow `docs/PHASE_2_2_CD_PROGRAMMER_HANDOFF.md`, freshly verify branch/HEAD/PR and actual SteelMC gitlink, and preserve all security and data-continuity invariants. PR #11 must remain open and unmerged; `master` and APK publication are out of scope.

## Programming handoff — automatically open a newly created Singleplayer world (2026-10-30)

1. **Task / priority:** P1 functional-flow correction. After pressing “Create New World”, enter the world immediately instead of returning to the world list.
2. **Authorization:** Investigation only in this chat. No source implementation authorization was issued here; the dedicated programming chat remains the implementation owner. Treat this handoff as a recommendation, not permission by itself.
3. **Repository state inspected:** `sylphie07/Pomme-Android-Test`; branch `unofficial-android-menu-access`; inspected HEAD `730c08c307a9aceaef9241a52dd9265be7f58c34`; PR #11 remains open and unmerged. SteelMC gitlink at this inspected tree: `c5098cdd8a4ec1176e00d737df24a2861b4055ac`. `.gitmodules` remains present. No local Termux worktree status can be inferred from GitHub.
4. **Problem and impact:** User tested the latest APK. Signed-out Singleplayer now works and the world is created, but the app returns to the world-selection screen. The user must select the new world and press Play manually. This differs from the expected Minecraft create-and-enter flow.
5. **Evidence / confirmed source path:** In `pomme-client/src/ui/menu/worlds.rs`, `MainMenu::build_create_world()` calls `self.create_world(key_bindings)` when the Create New World button is clicked (around lines 886–901). `MainMenu::create_world()` builds a `WorldSummary`, calls `self.world_list.create(summary)`, logs an error if persistence fails, and unconditionally calls `self.set_screen(Screen::WorldList, key_bindings)` (around lines 983–1002). It does not emit a gameplay action. The existing action enum in `pomme-client/src/ui/menu/mod.rs` already has `MenuAction::PlayWorld { folder }`. `update_menu()` in `pomme-client/src/app/phases/in_menu.rs` handles that action by calling `open_world()`; `open_world()` resolves the folder with `world_to_launch()`, starts the embedded server with `singleplayer::open()`, marks the world played, and returns a memory-transport Connect result.
6. **Confirmed facts vs hypotheses:** Confirmed: the create handler stores the world and switches to WorldList without requesting launch; the user's report matches this control flow. Likely root cause: missing handoff from successful creation to `MenuAction::PlayWorld`. No evidence currently points to Android filesystem permissions, Docker, or the authentication gate as the cause of this specific behavior.
7. **Recommended correction:** Refactor `create_world()` to return a result/action containing the exact generated save-folder identifier only after `WorldList::create()` succeeds. Have `build_create_world()` return `MenuAction::PlayWorld { folder } ` on successful creation so the existing central gameplay/auth path opens the just-created world. On save failure, do not emit PlayWorld; remain on a safe screen and present/log a clear error. Avoid deriving the folder from the display name: use the exact `summary.folder` selected by `available_folder_name()` to handle duplicate names correctly. Keep the world-list cache consistent with the successfully persisted world.
8. **Compatibility / data risks:** Do not delete, rename, recreate, or overwrite existing saves. Do not alter profile UUIDs, Microsoft auth policy, remote multiplayer checks, SteelMC, or the submodule pin. The new world should be opened exactly once; the normal manual PlayWorld path and startup QuickPlay must continue working. Save failure must not launch a partially created save.
9. **Acceptance criteria / regression tests:** (a) create a new world with an empty saves directory and verify the returned action targets its exact folder; (b) create a world when other saves already exist and verify it targets the new folder, not the first/most-recent world; (c) duplicate world names use the generated suffixed folder and launch that folder; (d) persistence failure emits no PlayWorld action and preserves the user's existing worlds; (e) manual world selection and QuickPlay still launch normally; (f) signed-out Release Singleplayer enters the newly created world without opening the account screen; (g) remote multiplayer still requires verified Microsoft authorization. Add focused unit tests around the create action/result, plus Android device verification after CI.
10. **Dependencies / order:** Implement on `unofficial-android-menu-access` only after the programming chat re-verifies the live branch/HEAD and reads current continuity. Run formatting, focused Rust tests, and full CI before producing a new APK. APK generation/install requires its separate authorization and must not be forced by modifying `master`.
11. **Not changed / not tested:** This investigation made no source-code, workflow, SteelMC, world, profile, or APK changes. No local tests/builds were run. The observed runtime behavior was reported by the user; no independent device reproduction was performed by this chat.



## Programming update — create-world auto-launch fix now authorized and applied (2026-10-30)

- The user has explicitly authorized the programming chat to implement the create-world auto-launch correction and prepare a new APK. This supersedes the older investigation-only authorization text in the earlier handoff entry.
- Applied change on `unofficial-android-menu-access`, source HEAD `546ec38f7d07d1784a0951cbfbb437088ffedf01`, file `pomme-client/src/ui/menu/worlds.rs`.
- Successful save now returns `MenuAction::PlayWorld` with the exact persisted folder identifier. Failed save returns no gameplay action and keeps the create screen open.
- Added regression tests for successful create, generated duplicate-name suffix, and failed persistence.
- Existing `open_world()` path remains the only launch route; no auth bypass or remote multiplayer change.
- CI #134 / run `41126486791` was pending. Android APK build run `41126478882` was in progress for the same head. Neither is considered passed/available until final results are inspected.
- Do not merge PR #11 or modify `master`. Preserve all worlds/profile data and prior Phase 2.2-E device-verified behavior.
