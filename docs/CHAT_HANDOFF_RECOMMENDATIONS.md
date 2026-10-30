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

## Suggested handoff prompt for any new chat

> Continue the Pomme Android task only within the scope previously authorized by the user. First read the repository documentation, verify the live branch/HEAD and SteelMC gitlink, inspect recent commits and the current code path, and read `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`. Do not assume earlier SHAs are current. Separate facts from hypotheses and proposals. Do not edit another branch, master, authentication, world/player data, or SteelMC unless explicitly authorized. Before changing code, state the exact intended files and regression tests. At the end, update continuity/development documentation and report exact changes and tests actually run.

## Current known boundary

Phase 2.2-A design is documented. Phase 2.2-B and all subsequent identity implementation stages remain unauthorized. This document does not grant permission to implement any recommendation.
