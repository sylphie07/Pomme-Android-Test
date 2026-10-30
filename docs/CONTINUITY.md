# Pomme Android — Continuity / Handoff

Last updated: 2026-10-30

## Current phase
**Phase 2.2-A — Persistent player identity design completed.**

Status: investigation and design report recorded in [PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md](PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md). **Phase 2.2-B implementation is not authorized.** The current approved action is documentation maintenance only.

## Current objective
Give each local profile a stable player identity that survives username changes, while preserving existing singleplayer player data and keeping local identity separate from Microsoft/Java authentication.

## Repository state and revision notes
- Repository: `sylphie07/Pomme-Android-Test`
- Authorized branch: `unofficial-android-menu-access`
- Last branch HEAD confirmed in the preceding repository verification: `0902bb4571bc9210eaffdfc1d8a688d6e5e0b417` — `docs: log Phase 2.2-A design investigation`.
- This session re-fetched the documentation from the authorized branch and confirmed that commit object exists. The available branch-search response confirms branch existence but does not itself expose the branch-ref SHA; **every new session must re-check the live HEAD before editing**.
- The intervening commits `920f75d0cbda76f01132d4f85a70b63af2aa612f` and `2c9e953b7d09ab846a865b4e40b6eb61831fa786` update documentation/handoff and the Phase 2.2-A report. Commit `0902bb4571bc9210eaffdfc1d8a688d6e5e0b417` appends the investigation to the development log.
- The earlier `4c1073a5af9f77b0b8f8c6723598496c1bde7a71` is historical, not the latest known documentation revision.
- SteelMC gitlink last verified: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- SteelMC submodule URL: `https://github.com/Steel-Foundation/SteelMC.git`.
- `master` was not modified by this documentation session.
- GitHub connector access does not reveal an Android/Termux uncommitted worktree; do not claim local worktree cleanliness without checking it on-device.

## Phase 2.2-A findings — essential context
1. Proposed model: `LocalProfile { id: Uuid, username: String, player_uuid: Uuid }`.
2. `id` remains the profile-record ID. Never repurpose it.
3. Current local player UUID is derived from the username.
4. SteelMC offline `handle_hello()` ignores the client-provided UUID as an identity authority and derives offline UUID from the requested name. Sending a stable UUID in Hello alone will not work.
5. Recommended architecture: trusted internal per-connection identity supplied only through Pomme's embedded/in-process connection constructor. Do not trust arbitrary Hello UUIDs; do not put player identity into a shared listener-scoped `ServerConnectionSession`.
6. Legacy migration candidate: `player_uuid = offlineUUID(username_currently_stored)`, preserving `LocalProfile.id` and selected-profile ID. If this UUID is preserved, existing SteelMC UUID-keyed storage paths remain unchanged.
7. Current profile-store error handling can replace corrupt/invalid profile JSON with a new default profile. Fix data-preservation behavior before migration.
8. Detect duplicate names and UUIDs non-destructively. Same-name profiles currently share the name-derived UUID/data. Never silently split or reassign.
9. Local identity must remain independent of Microsoft account sign-in/out and remote Java authentication.
10. **Advancement persistence limitation:** at the inspected SteelMC pin, `steel-core/src/player/mod.rs` contains `Player.advancements` initialized to `PlayerAdvancement::default()` and an explicit `TODO save the advancement`. No advancement persistence was found in the inspected player-data storage path. Treat this as a confirmed limitation of the inspected revision, not as a completed feature. It requires separate SteelMC work and verification.
11. No build, unit/integration tests, CI run, or Android-device checks were performed for the Phase 2.2-A design.

## Required next-chat startup checklist
1. Read `docs/PROJECT_CONTEXT.md`, this file, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`, and `docs/PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md`.
2. Verify exact live branch HEAD and current SteelMC gitlink again.
3. Inspect recent commits and confirm whether any source files changed since the Phase 2.2-A baseline.
4. If using Termux, separately inspect local `git status`, branch, and uncommitted changes; GitHub cannot establish the local worktree state.
5. Review the complete Phase 2.2-A report and cross-chat recommendations.
6. Do not begin Phase 2.2-B until the user explicitly authorizes implementation.
7. Preserve all worlds/player data; do not silently regenerate or migrate UUIDs.

## Proposed next phases (not authorized)
- **2.2-B1:** harden profile JSON loading/writing and recovery; preserve corrupt/unknown-version files.
- **2.2-B2:** add separate persistent `player_uuid` to the profile model.
- **2.2-B3:** implement versioned, idempotent legacy migration and duplicate handling.
- **2.2-B4:** profile-store unit tests and corruption/recovery tests.
- **2.2-C:** trusted integrated per-connection identity in SteelMC; generic offline Hello behavior unchanged.
- **2.2-D:** Pomme plumbing from LocalProfile to integrated connection; Microsoft flow untouched.
- **2.2-E:** existing-world persistence, collision/recovery, CI and Android validation.
- Advancement persistence is a separate SteelMC concern; do not silently bundle it into the identity work.

## Mandatory constraints
- Work only on `unofficial-android-menu-access`.
- Never modify `master`.
- No merge, PR, APK, release, or publication unless explicitly requested.
- Keep local profile identity and Microsoft/Java authentication separate.
- Never trust arbitrary UUIDs from generic offline Hello packets.
- Never delete, rename, overwrite, or migrate world/player files without an explicit proven plan and authorization.
- Distinguish source facts, user reports, hypotheses, proposals, applied changes, and tested results.

## Handoff requirements
Every future technical session must record exact branch/HEAD, submodule SHA, files inspected/changed, decisions, tests actually run and exact outcomes, unresolved risks, and next action.

## Documentation-only update — 2026-10-30
This update refreshes the handoff and points future chats to `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`. It records the advancement-persistence TODO and the known revision history. No application source, SteelMC submodule, worlds, or player data were changed by this update. No tests or builds were run.
