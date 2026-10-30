# Pomme Android — Project Context

Last updated: 2026-10-30

## Repository and scope
- Repository: `sylphie07/Pomme-Android-Test`
- Working branch for the Android menu/authentication investigation: `unofficial-android-menu-access`
- This is an Android-focused continuation/fork workflow for Pomme. Do not assume the upstream desktop behavior or the repository README fully describes the current Android branch.
- Keep work scoped to the explicitly authorized branch. Do not modify `master` or other branches unless the user explicitly authorizes it.

## Main project objective
Make the Android build usable without requiring Microsoft authentication merely to navigate the menu, while preserving the authentication and safety checks required when actually joining or starting gameplay.

## Current technical focus
1. Trace all routes that can start or connect to gameplay:
   - `MenuAction::Connect`
   - `MenuAction::PlayWorld`
   - `open_world()`
   - quick-start / command-line startup arguments and any equivalent launch paths
2. Identify the best central point(s) for authentication checks:
   - menu navigation should remain available while signed out;
   - online/server actions must retain the authentication they require;
   - singleplayer/world startup must retain whatever checks and identity assumptions are genuinely required by the implementation.
3. Investigate the Android-specific singleplayer discrepancy and determine whether it is caused by UI routing, platform gating, runtime setup, authentication, or another dependency.

## User-reported project history (historical context; re-verify against current source)
- The user previously reported that an APK built from a branch installed and launched successfully on their Android device.
- On first run, it downloaded the Minecraft version and local game components; the user then successfully entered and played a world.
- Other users reportedly continued to report world-creation problems, so the user's successful test does not establish that the issue is fixed for all Android devices/builds.
- A previous conversation tracked GitHub Actions CI run #39: https://github.com/sylphie07/Pomme-Android-Test/actions/runs/40985480909 . The last recorded status in that conversation still showed relevant jobs running; this status is stale and must be checked live before relying on it.

## Device / environment context
- The user develops and tests from Android and does not have a desktop PC available.
- User's test phone: Infinix Hot 60 Pro+.
- The user can use GitHub and Termux/Android workflows.
- Avoid assuming the user can run desktop-only build or debugging steps.

## Working rules
- Inspect the current branch, HEAD commit, worktree/diff where available, and relevant source before editing.
- Distinguish verified repository facts from user-reported observations and hypotheses.
- Never describe proposed/generated code as applied until the repository confirms it.
- Never describe a change as tested unless a test was actually run and its result observed.
- Record failed attempts and rejected approaches; explain why they were rejected when known.
- Keep changes small and scoped. Preserve unrelated work.
- Before any potentially destructive operation, explain the effect and obtain explicit authorization.
- At the end of each technical session, update the documentation in `docs/` and leave a precise handoff.
