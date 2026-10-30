# Pomme Android — Continuity / Handoff

Last updated: 2026-10-30

## Current objective
Continue the Android menu/authentication investigation on `unofficial-android-menu-access`.

Target behavior:
- Allow users to navigate the menu without signing in.
- Preserve the authentication checks required for online play and any gameplay path that genuinely requires an authenticated identity.
- Resolve the Android singleplayer discrepancy without weakening unrelated protections.

## Investigation scope already identified
Trace and compare:
- `MenuAction::Connect`
- `MenuAction::PlayWorld`
- `open_world()`
- quick-start / startup argument paths
- Android-specific singleplayer launch and world-creation flow

## Last known state
- The branch `unofficial-android-menu-access` was confirmed to exist on GitHub on 2026-10-30.
- The current branch HEAD was not independently retrieved while this handoff was written. Verify it at the start of the next session.
- The documentation directory/files did not exist at the checked paths before this initial documentation commit.
- No source-code changes are made by this initial documentation handoff.
- CI run #39 was previously reported as still running; re-check the run and the individual `singleplayer`, `android`, and `test` jobs before drawing conclusions.

## Next session — mandatory startup checklist
1. Read this file and all other files in `docs/`.
2. Confirm repository and branch: `sylphie07/Pomme-Android-Test` / `unofficial-android-menu-access`.
3. Fetch the latest branch HEAD and compare it with the commit referenced by the previous session, if available.
4. Inspect uncommitted changes / latest commits before editing.
5. Re-check CI run #39 if its result remains relevant.
6. Continue source-level tracing of all gameplay entry points and Android singleplayer behavior.
7. Update this handoff after meaningful progress.

## Handoff discipline
Each handoff must state:
- exact branch and HEAD SHA;
- files inspected and files changed;
- functions/routes analyzed;
- tests actually run and exact outcomes;
- unresolved questions and known risks;
- the next concrete action.
