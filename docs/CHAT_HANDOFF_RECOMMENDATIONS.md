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
- Do not implement Phase 2.2-B until the user explicitly authorizes it.
- If authorized, begin with the profile JSON safety issue before adding `player_uuid`: corrupt, incomplete, or future-version JSON must never be silently replaced by a default profile.
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
- **Phase 2.2-B1–B4 — persistent local profile identity:** implementation is explicitly authorized by the user on 2026-10-30 and has begun on `unofficial-android-menu-access`. Source changes add `player_uuid`, versioned JSON, historical UUID migration, strict duplicate validation, corrupt/missing-primary backup recovery, quarantine preservation, future-version downgrade protection, and regression tests. CI and Android verification remain pending. The experimental branch is reference material only and was not wholesale-merged. The new field is not yet wired into SteelMC; C/D remain separate downstream phases.

- **B1–B4 CI verification:** source commit `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` had CI run `41008897990` (run #59) cancelled. The parent-source run `41008828793` (run #58) failed `ci-fmt` due rustfmt differences in `pomme-client/src/user.rs`; its other substantive jobs were cancelled/skipped. Re-run CI on the latest source HEAD; do not treat the earlier run #56 (`fd945ed...`, before B1–B4) as validation of the profile implementation.
- **P2.2-C/D identity continuity blocker:** `LocalProfile::new()` creates a random `player_uuid`, but the runtime still uses `offline_uuid(username)` in `UserData::from_local_profile()` and `set_local_profile()`. Legacy migration uses the historical offline UUID, but profiles created during B1–B4 can already have world data keyed by the name-derived UUID. Before wiring the random stored UUID into SteelMC, define a non-destructive migration/compatibility path for these newly created profiles. Never simply switch UUIDs and claim world continuity.
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
