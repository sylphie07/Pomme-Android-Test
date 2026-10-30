# Pomme Android — Project Context

Last updated: 2026-10-30

## Repository and scope
- Repository: `sylphie07/Pomme-Android-Test`
- Authorized working branch: `unofficial-android-menu-access`
- This is an Android-focused continuation/fork workflow for Pomme. Do not assume upstream desktop behavior describes this branch.
- Keep all work scoped to the authorized branch. Do not modify `master` or other branches without explicit authorization.

## Main project objective
Make the Android build usable without requiring Microsoft authentication merely to navigate the menu, while preserving authentication and safety checks required for gameplay.

Target behavior:
- Menu navigation available while signed out.
- Persistent local profiles for offline singleplayer.
- Local profile identity kept separate from Microsoft/Java account identity.
- Online/multiplayer authentication remains protected and uses the official authentication flow.
- Existing player/world data must not become orphaned or appear lost because a profile is renamed or migrated.

## Current technical focus — Phase 2.2-A
**Design persistent player identity per local profile.** We are currently in the investigation/design stage only. The next implementation phase is not authorized until the Phase 2.2-A report is reviewed and the design is explicitly approved.

The current audit findings supplied in the prior technical report:
1. Pomme currently derives the offline player UUID from the profile name.
2. SteelMC offline login recalculates the offline UUID from the requested username; it does not accept an arbitrary client Hello UUID as trusted identity.
3. SteelMC indexes player persistence using the game profile UUID. The reported affected areas include global player data, domain player data, statistics, permissions, and known-player records.
4. Renaming a profile changes its name-derived offline UUID. The old data may remain on disk but will no longer be selected by the new identity.
5. `LocalProfile.id` already identifies the local profile record with a persistent UUID, but is not currently the same thing as the player's SteelMC identity.
6. Candidate design: add a separate persistent `player_uuid` to `LocalProfile`, preserving `id` as the profile-record ID.
7. For existing profiles, candidate migration is to initialize `player_uuid` to the historical offline UUID of the profile's current username. This should preserve existing SteelMC storage keys if the handoff is implemented correctly.
8. The persistent identity must reach the embedded SteelMC through a trusted internal mechanism. Do not trust arbitrary UUIDs supplied by generic offline clients or weaken generic offline login.
9. Advancement persistence and any unlisted storage domains remain unverified; do not claim complete coverage without source inspection and tests.

These are recorded audit findings/design hypotheses from the prior report, not yet independently revalidated against the current submodule source in this session. The detailed scope and verification requirements are in [PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md](PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md).

## Previous technical focus
Earlier work investigated menu access without sign-in and traced gameplay entry routes:
- `MenuAction::Connect`
- `MenuAction::PlayWorld`
- `open_world()`
- quick-start / command-line startup arguments

Android singleplayer discrepancy was also investigated. Historical user report: the user successfully launched and played a world on their own Android device from a built APK, but this did not prove the issue resolved for all devices/builds.

## Repository state recorded on 2026-10-30
- Branch existence: `unofficial-android-menu-access` was returned by GitHub branch search.
- Most recent commit returned by repository commit search: `a147ca27e24850e5d6e796e1dc7bd02e1ecf3a72` — “Add 'singleplayer' feature to pomme-client dependency”.
- That commit changes `pomme-android/Cargo.toml` to enable the `singleplayer` feature for the Android `pomme-client` dependency.
- Previous session's known HEAD was `c730ae9e1334646f507f9f068e8774e8950a18a8`; it is historical and must not be treated as current.
- SteelMC submodule SHA previously recorded: `e655880a7b1487b2444c70013c14ee86c69d53b3`; verify again before implementation.
- The connector used for this documentation update confirmed the branch exists and fetched files from it, but did not expose a dedicated branch-head SHA field. Therefore `a147ca27...` is the latest commit observed in repository search, not a separately confirmed branch-ref HEAD. Verify the exact HEAD at the start of the next coding session.
- Existing documentation files confirmed in `docs/`: `PROJECT_CONTEXT.md`, `CONTINUITY.md`, `DEVELOPMENT_LOG.md`, and `DECISIONS.md`. `docs/ARCHITECTURE.md` and `docs/README.md` were not found at the checked paths.

## Device / environment
- User develops and tests from Android and has no desktop PC.
- Test phone: Infinix Hot 60 Pro+.
- User can use GitHub and Termux/Android workflows.
- Avoid desktop-only instructions unless offering a realistic Android alternative.

## Working rules
- At the start of each session, read all relevant `docs/`, confirm branch and exact HEAD, inspect recent commits and worktree/diff where available before editing.
- Distinguish source-verified facts, test results, user reports, hypotheses, and proposals.
- Never claim a proposal is implemented or tested until verified.
- Do not delete profiles or player data.
- Do not silently change player UUIDs or migrate storage without a proven compatibility plan.
- Do not weaken official Microsoft/Java authentication or trust arbitrary offline Hello UUIDs.
- No merge, PR, APK generation, publishing, or changes to `master` without explicit authorization.
- After every meaningful technical session, update `docs/CONTINUITY.md` and append to `docs/DEVELOPMENT_LOG.md`; update other documentation when relevant.
