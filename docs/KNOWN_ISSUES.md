# Pomme Android — Known Issues and Open Questions

Last updated: 2026-10-30

## KI-001 — Authentication currently coupled to menu access (scope to verify)
- Desired behavior: menu browsing should work while signed out.
- Open question: where are authentication checks currently enforced, and which checks are required only for gameplay?
- Required investigation: trace all gameplay entry paths, not just the menu button handler.
- Status: open; exact current behavior must be verified against the branch source.

## KI-002 — Android singleplayer discrepancy
- Context: the user previously succeeded in creating/entering a world with an APK built from a branch, while other users reportedly continued to encounter world-creation failures.
- Open questions:
  - Is the discrepancy caused by different commits/build configurations?
  - Does the failure occur during menu routing, local server startup, runtime/dependency setup, or world initialization?
  - Are there device- or Android-version-specific differences?
- Status: open; user report is historical and not a universal fix confirmation.

## KI-003 — CI run #39 status requires refresh
- Run: https://github.com/sylphie07/Pomme-Android-Test/actions/runs/40985480909
- Previous chat state: reported as still executing.
- Required check: inspect global conclusion and the `singleplayer`, `android`, and `test` jobs.
- Status: stale / unverified. Do not report as currently running or completed without a live check.

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

## KI-005 — Phase 2.2-B automated verification pending
- The new profile-store tests have been written but were not run locally in this session.
- CI run `41008897990` targeted source HEAD `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` and was pending at the last check.
- Required: inspect final CI jobs and logs, fix failures, then run Android/device acceptance when a reproducible APK is separately authorized.
- Status: pending.
