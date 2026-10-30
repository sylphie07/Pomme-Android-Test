# Phase 2.2-A — Persistent Player Identity per Local Profile

Last updated: 2026-10-30
Status: **Design/investigation only — implementation not authorized**

## 1. Purpose
Design a stable player identity for each local Pomme profile so changing the displayed username does not change the identity SteelMC uses to locate that player's persistent data.

This work must preserve existing singleplayer data and keep local profile identity separate from official Microsoft/Java account identity.

## 2. Repository baseline to verify before work
- Repository: `sylphie07/Pomme-Android-Test`
- Intended branch: `unofficial-android-menu-access`
- Latest commit surfaced by repository commit search on 2026-10-30: `a147ca27e24850e5d6e796e1dc7bd02e1ecf3a72`.
- Its inspected diff enables the `singleplayer` feature on the Android `pomme-client` dependency in `pomme-android/Cargo.toml`.
- Previous session's known HEAD `c730ae9e1334646f507f9f068e8774e8950a18a8` is historical, not current.
- Previous SteelMC submodule SHA: `e655880a7b1487b2444c70013c14ee86c69d53b3`; verify current pin before relying on it.
- Exact branch-ref HEAD was not exposed by the available branch-search result. Re-check it before source changes.

## 3. Existing identity concepts
- `LocalProfile.id`: persistent identifier for the local profile record.
- Current offline player identity (reported): UUID derived from the profile username.
- Proposed `LocalProfile.player_uuid`: separate persistent identity for the player inside the embedded game server.

Do not conflate these identifiers.

## 4. Audit findings to verify
The prior audit report states:
1. Pomme calculates the offline UUID from the profile name.
2. SteelMC offline login recalculates the UUID from the requested username and does not trust the client Hello UUID.
3. SteelMC uses the game-profile UUID as a key for player persistence.
4. Reported affected stores include global player data, domain player data, statistics, permissions, and known-player records.
5. A username rename changes the name-derived UUID and can make prior data appear unavailable even if files remain.
6. Advancement persistence was not conclusively verified.

Treat these as report-derived claims until checked against the exact Pomme source and exact pinned SteelMC source.

## 5. Candidate design to evaluate
Add a distinct persistent `player_uuid` to `LocalProfile`, preserving `id`.

For old profiles that lack this field, evaluate initializing it to the historical offline UUID derived from the profile's currently stored username. The objective is to keep existing storage keys unchanged, avoiding a broad file migration.

The embedded SteelMC must receive this identity through a trusted internal mechanism. Evaluate at least:
- An internal server/session context passed only by Pomme to its embedded server.
- An embedded-server-specific configuration or construction path.
- A narrowly scoped extension to the server connection/session API.

For every option, assess API impact, trust boundary, compatibility, testability, and risk of allowing arbitrary remote clients to impersonate another UUID.

Do not accept a client-provided UUID in generic offline login as the solution.

## 6. Required migration and edge-case analysis
Investigate:
- Old profiles with no `player_uuid`.
- New profile creation.
- Missing/null/malformed fields and corrupt JSON.
- Duplicate profile names and possible UUID collisions.
- Interrupted writes and recovery, preserving the existing atomic-write behavior.
- Profile switching and app restart.
- Username changes after migration.
- Multiple local profiles and data isolation.
- Microsoft sign-in/sign-out without changing local identity.
- Whether any world/player data file migration is actually needed.
- All player-data stores, including inventory, equipment, position, health, experience, statistics, permissions, known players, domain data, and advancements where applicable.

Never delete old data or guess which profile owns ambiguous data.

## 7. Required tests to propose
At minimum:
1. Create a new local profile and persist its identity.
2. Load a legacy profile with no `player_uuid`.
3. Restart and confirm identity stability.
4. Rename the profile and confirm player UUID remains unchanged.
5. Switch between two local profiles and confirm isolation.
6. Confirm Microsoft login/logout does not mutate local profile identity.
7. Confirm integrated singleplayer receives the intended trusted identity.
8. Confirm generic offline login does not accept arbitrary Hello UUIDs as trusted identity.
9. Verify persistence of inventory, equipment, position, XP, stats, permissions, and every other discovered store.
10. Test malformed JSON, missing fields, duplicate identities, and interrupted writes without destructive recovery.

Separate unit tests, integration tests, CI, and real Android-device tests. Do not claim one category proves another.

## 8. Deliverables for the Phase 2.2-A report
The report must include:
- Exact repository branch, HEAD, and SteelMC submodule SHA inspected.
- Files and functions inspected, with source references.
- Verified current identity flow from profile selection to SteelMC game profile.
- Verified list of persistence stores and their UUID keys.
- Comparison of trusted identity-handoff alternatives.
- Recommended design and why.
- Legacy-profile migration behavior.
- Whether storage-file migration is needed and evidence.
- Security/trust-boundary analysis.
- Edge cases, risks, and tests.
- Exact implementation order for Phase 2.2-B onward.
- Explicit list of unresolved questions.

## 9. Scope restrictions
Until the user explicitly approves implementation:
- Do not edit application source code.
- Do not change the SteelMC submodule.
- Do not implement profile rename.
- Do not alter generic offline authentication.
- Do not migrate, rename, delete, or rewrite player data.
- Do not modify `master`.
- Do not merge, open a PR, publish, or generate an APK.

## 10. Next action
Obtain and review the complete Phase 2.2-A report from the separate analysis chat. Verify its claims against the current repository and pinned SteelMC source. Only after the design is reviewed should a Phase 2.2-B implementation plan be presented for explicit user approval.
