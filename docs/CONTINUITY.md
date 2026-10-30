# Pomme Android — Continuity / Handoff

Last updated: 2026-10-30

## Current phase
**Phase 2.2-A — Persistent player identity design completed.**

Status: investigation and design report recorded in [PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md](PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md). **No implementation is authorized yet. Wait for explicit user approval before Phase 2.2-B.**

## Current objective
Give each local profile a stable player identity that survives username changes, while preserving existing singleplayer player data and keeping local identity separate from Microsoft/Java authentication.

## Exact repository state at this handoff
- Repository: `sylphie07/Pomme-Android-Test`
- Branch: `unofficial-android-menu-access`
- Live branch HEAD verified while recording this handoff: `4c1073a5af9f77b0b8f8c6723598496c1bde7a71`
- HEAD commit message: `docs: fix project context documentation typo`
- User's expected historical HEAD for the investigation: `c730ae9e1334646f507f9f068e8774e8950a18a8`; that SHA was verified during the investigation, but branch advanced afterward.
- Current SteelMC gitlink: `e655880a7b1487b2444c70013c14ee86c69d53b3`
- Submodule URL: `https://github.com/Steel-Foundation/SteelMC.git`
- `.gitmodules` and current tree were checked.
- This chat's documentation updates will create a new docs-only commit on this branch. Re-check live HEAD after the update.

## Phase 2.2-A findings — essential context
1. Proposed model:
   `LocalProfile { id: Uuid, username: String, player_uuid: Uuid }`
2. `id` remains the profile-record ID. Never repurpose it.
3. Current local player UUID is derived from the username.
4. SteelMC offline `handle_hello()` ignores the client-provided UUID as an identity authority and derives offline UUID from the requested name. Sending a stable UUID in Hello alone will not work.
5. Recommended architecture: trusted internal per-connection identity supplied only through Pomme's embedded/in-process connection constructor. Do not trust arbitrary Hello UUIDs; do not put player identity into a shared listener-scoped `ServerConnectionSession`.
6. Legacy migration: `player_uuid = offlineUUID(username_currently_stored)`, preserving `LocalProfile.id` and selected profile. If this UUID is preserved, existing SteelMC storage paths remain unchanged; no player-file/world-data migration is needed just to add the field.
7. Important defect: current profile-store load error handling can replace corrupt/invalid profile JSON with a new default profile. This must be fixed before migration.
8. Detect duplicate names and UUIDs non-destructively. Same-name profiles currently share the name-derived UUID/data. Never silently split or reassign.
9. Local identity must remain independent of Microsoft account sign-in/out and remote Java authentication.
10. Advancement persistence was not verified in the inspected storage path; do not claim it is absent or migrated.
11. No tests/build/CI/Android checks were run for this design report.

## Required next-chat startup checklist
1. Read `docs/PROJECT_CONTEXT.md`, this file, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, and `docs/PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md`.
2. Verify the exact live branch HEAD and current SteelMC submodule SHA again.
3. Inspect the docs-only commit created by the previous session and confirm source tree unchanged.
4. Review the Phase 2.2-A report before proposing implementation.
5. Do not begin Phase 2.2-B until the user explicitly authorizes it.
6. Preserve all worlds/player data; do not silently regenerate or migrate UUIDs.

## Proposed next phases (not authorized)
- **2.2-B:** profile model, versioned JSON, safe legacy migration, corruption handling, durable writes, unit tests.
- **2.2-C:** trusted integrated per-connection identity in SteelMC; generic offline Hello behavior unchanged.
- **2.2-D:** Pomme plumbing from LocalProfile to integrated connection; Microsoft flow untouched.
- **2.2-E:** existing-world persistence, collision/recovery, CI and Android validation.

## Mandatory constraints
- Work only on `unofficial-android-menu-access`.
- Never modify `master`.
- No merge, PR, or APK unless explicitly requested.
- Keep local profile identity and Microsoft/Java authentication separate.
- Never trust arbitrary UUIDs from generic offline Hello packets.
- Never delete, rename, overwrite, or migrate world/player files without an explicit proven plan and authorization.
- Distinguish design proposals from implemented code and actual test results.

## Handoff requirements
Every future technical session must record exact branch/HEAD, submodule SHA, files inspected/changed, decisions, tests actually run and exact outcomes, unresolved risks, and next action.
