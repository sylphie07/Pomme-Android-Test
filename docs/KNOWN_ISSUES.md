# Pomme Android — Known Issues and Open Questions

Last updated: 2026-10-30

## KI-001 — Menu/gameplay authentication separation
- Source review at the current branch shows a centralized `authorize_gameplay()` gate in `pomme-client/src/app/phases/in_menu.rs`.
- `MenuAction::Connect`, `MenuAction::PlayWorld`, `open_world()`, and both QuickPlay modes route through the gate. Startup QuickPlay calls `in_menu::quick_play()` after setup.
- `UserData::allows_singleplayer()` permits local play without Microsoft authorization; `allows_multiplayer()` requires a verified, unexpired session with a non-empty token and valid account UUID.
- Menu navigation itself is not globally blocked by the account gate.
- Status: implementation is present by static source inspection. End-to-end behavior, cancellation races, and exactly-once pending-action resumption still need automated verification.

## KI-002 — Android singleplayer discrepancy
- The Android dependency enables the `singleplayer` Cargo feature in `pomme-android/Cargo.toml`.
- `pomme-client/src/singleplayer.rs` contains an ignored integration test that opens a real embedded SteelMC world, joins through the in-memory protocol, leaves, and re-enters the same save in one process.
- CI's `singleplayer` job invokes `just ci-singleplayer`, but the relevant run for the B1–B4 source revision was cancelled before the test completed.
- The user previously reported a successful world launch on their own Android device; this does not establish compatibility across devices/builds.
- Status: unresolved as a general Android issue. No current device logs or reproducible failure from another device were available in this investigation.

## KI-003 — Phase 2.2-B1–B4 CI verification incomplete
- Parent-source run `41008828793` (run #58): `ci-fmt` failed because rustfmt differences remained in `pomme-client/src/user.rs`; the other substantive jobs were cancelled or skipped. This run targeted the parent source revision, not the final B1–B4 source commit.
- Final-source run `41008897990` (run #59), associated with source commit `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`: cancelled. The connector returned no job records for this run, so no successful validation can be inferred.
- Newer run `41009018315` (run #63), on the revision before the latest profile fix: `ci-fmt` failed with rustfmt diffs in `pomme-client/src/user.rs`; `test` and `singleplayer` were cancelled after a newer commit superseded it. Do not use this run to validate the latest profile fix.
- Latest run `41009217544` (run #70), for commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`, was in progress at the latest poll. `test`, `singleplayer`, Android, iOS, lint, and fmt jobs had started; no final conclusions were available yet.
- Run `41008473575` (run #56) targeted the earlier `fd945ed...` revision, before the B1–B4 source commits. Android Clippy, format, lint, iOS Clippy, and PR-template jobs passed; `test` and `singleplayer` were cancelled and aggregate `ci` failed. It does not validate B1–B4.
- Latest CI run `41009337238` targets the PR merge ref with source head `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc`; it was pending at the last check. No complete CI pass is established yet.

## Reporting format
For each issue, record:
- reproduction conditions;
- exact branch and commit;
- relevant source paths/functions;
- logs or CI links;
- attempted fixes and outcomes;
- current status and next diagnostic step.


## KI-004 — Persistent player UUID is stored but not yet used by the live game identity
- Scope: Phase 2.2-B1–B4.
- Source fact: `LocalProfile.player_uuid` is now persisted and migrated, but `UserData::from_local_profile()` still derives the runtime UUID from the username.
- Impact: this implementation alone does not yet preserve the player's runtime UUID across a username change in SteelMC.
- Required follow-up: trusted per-connection identity in SteelMC (Phase 2.2-C) and Pomme integration (Phase 2.2-D), with generic offline Hello UUIDs remaining untrusted.
- Status: known and intentionally not addressed by B1–B4.

## KI-005 — Phase 2.2-B1–B4 automated verification pending
- The profile-store tests have been added but were not run locally in this session.
- CI runs for superseded source commits were cancelled; no complete successful run has yet been established for the latest source.
- Latest source commit: `c0576191407a1ad53db77c0635c7b0820669e547`.
- Required: inspect the latest PR CI run and its fmt, lint, unit-test, Android, and singleplayer jobs. Then perform Android/device acceptance when a reproducible APK is separately authorized.
- Status: pending; do not treat written tests as passed.

## KI-006 — New-profile UUID transition compatibility
- Previous risk: experimental format 1 stored random `player_uuid` values while the runtime still used `offline_uuid(username)`.
- Correction: commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce` now creates new profiles with the historical offline UUID and migrates experimental format 1 to format 2 using that same UUID.
- Backup recovery now restores the exact validated byte snapshot in commit `c0576191407a1ad53db77c0635c7b0820669e547`.
- Runtime still derives UUID from username until C/D. Before wiring `player_uuid` into SteelMC, re-verify this invariant and test inventory, position, Ender Chest, stats, and all UUID-keyed records.
- Status: compatibility correction applied in source; CI verification pending.

## KI-007 — Local profile management UI is not integrated
- `LocalProfileStore` is loaded at startup and its storage API includes `profiles()`, `create_profile()`, and `select_profile()`.
- These APIs and `UserData::set_local_profile()` are marked `dead_code`/reserved for a future selection flow. No player-facing profile picker or create/switch flow was found in the inspected menu code.
- Status: profile persistence foundation exists; multi-profile user experience remains incomplete.

## KI-008 — Gameplay authorization and deferred QuickPlay lack focused regression tests
- `pomme-client/src/app/phases/in_menu.rs` contains `authorize_gameplay()`, `start_pending()`, `connect_remote()`, `open_world()`, and `quick_play()`, but no local test module was found in that file.
- `account.rs` has only limited UI tests; no focused coverage was found for restore-vs-interactive-sign-in races, cancel/back while restoring, QuickPlay resumption, or exactly-once launch.
- Required tests: signed-out menu navigation; offline singleplayer; signed-out remote rejection; valid/expired session; startup QuickPlay multiplayer/singleplayer; restore success/failure; cancel/back clearing the pending action; one and only one resumed launch.
- Status: static routing looks centralized; behavior is not fully regression-tested.

## KI-009 — Microsoft token lifecycle and Android credential storage
- `pomme-client/src/account.rs` documents a TODO to refresh again when a long session outlives the day-long Minecraft token.
- The Android `SecretStore` is a JSON file in app-private storage (`accounts.json`); the code contains a TODO to wrap it with Android Keystore.
- Status: known follow-up. Do not weaken account ownership/authentication checks or change the authentication flow as part of profile work.

## KI-010 — Android APK workflow does not automatically build this feature branch
- `.github/workflows/android-apk.yml` triggers on `push` to `master` for selected paths and `workflow_dispatch`; it has no `pull_request` trigger for `unofficial-android-menu-access`.
- The current investigation found no APK artifact/run proving the current feature-branch source builds into an installable APK.
- Status: distribution/device verification pending. Any workflow change or APK publication needs its own authorization.

## KI-011 — SteelMC advancement persistence remains separate
- At the recorded SteelMC pin `e655880a7b1487b2444c70013c14ee86c69d53b3`, `Player.advancements` is initialized in memory and marked `TODO save the advancement`; the inspected player-data persistence path does not serialize it.
- Status: separate SteelMC handoff. Do not bundle it into Phase 2.2-C/D or change the submodule without authorization.


## KI-012 — Existing-world per-world inventory isolation (device check)
- User report dated 2026-10-30: on the current APK, each world retains its own independent inventory.
- Status: **user-tested / passed for per-world inventory separation** on the tested APK. This is device feedback, not an independently reproduced test in this chat.
- Limits: the report does not separately confirm player position, stats, Ender Chest, every other player-data field, or a clean save/close/reopen cycle. Keep those checks open under Phase 2.2-E.
- Regression requirement: after the world-creation auto-launch fix, verify that creating/entering a new world does not disturb the independent inventory of any existing world.
