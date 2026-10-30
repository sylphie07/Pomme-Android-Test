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

## Current technical focus — Phase 2.2-B1–B4 implementation

**The user explicitly authorized implementation on 2026-10-30.** Work is scoped to the authorized branch `unofficial-android-menu-access` and currently covers profile-store safety, the independent `player_uuid` field, compatible legacy migration, and integrity tests.

Implemented in source (not yet CI-verified at the time of this note):
- `LocalProfile.id` remains the profile-record identifier.
- `LocalProfile.player_uuid` is a separate persistent UUID.
- New profiles start with the historical offline UUID used by the current runtime, so later C/D plumbing can consume the stored value without changing existing saves.
- Legacy profile migration derives the historical offline UUID from the stored username, preserving profile IDs and the selected profile. Newly created profiles also start with that same historical runtime UUID.
- Experimental format-1 documents that stored random UUIDs are migrated to the historical runtime UUID; current document format is version 2.
- Invalid profile JSON is not replaced by a default profile.
- A valid backup can recover a corrupt or missing primary; a corrupt primary is quarantined instead of discarded.
- Experimental format 1 (which stored random UUIDs not used by the runtime) migrates to format 2 and resets the player UUID to the historical offline UUID, preventing a later C/D transition from orphaning world data.
- Backup recovery writes the exact byte snapshot that was validated, avoiding a second read of a potentially changed backup file.
- Future format versions fail closed and are not silently downgraded from an older backup.
- Duplicate profile IDs, player UUIDs, and case-insensitive names are rejected without silently reassigning identity.
- The app startup now handles profile-store errors by logging and exiting instead of continuing with replaced defaults.

Important boundary:
- The new `player_uuid` is persisted but is **not yet used by `UserData::from_local_profile()` or the embedded SteelMC connection**. That remains Phase 2.2-C/D work and is not part of this implementation. New and migrated profile UUIDs now match the runtime's historical offline UUID.
- No SteelMC source or submodule pin has been changed.
- Microsoft/Java authentication behavior has not been intentionally changed by the profile-store patch.

The implementation is based on a selective review of the experimental branch `phase-2.2-b-persistent-local-profile`; it was not wholesale-merged. See the current continuity note and development log for exact commits, limitations, and CI state.

## Previous technical focus
Earlier work investigated menu access without sign-in and traced gameplay entry routes:
- `MenuAction::Connect`
- `MenuAction::PlayWorld`
- `open_world()`
- quick-start / command-line startup arguments

Android singleplayer discrepancy was also investigated. Historical user report: the user successfully launched and played a world on their own Android device from a built APK, but this did not prove the issue resolved for all devices/builds.

## Repository state recorded on 2026-10-30

- Repository: `sylphie07/Pomme-Android-Test`.
- Authorized branch: `unofficial-android-menu-access`.
- Latest verified branch HEAD before this documentation refresh: `9258c2118f744a17591a0dba75a50d3c5f16fa38`.
- Latest source-code commit: `c0576191407a1ad53db77c0635c7b0820669e547`; subsequent changes in this session are documentation-only.
- Phase 2.2-B1–B4 source commits: `4bd084c8a9fca176da2545b800a8a2eb24c7b3e9`, `334f5f958bf99a61bc3d67bcc48036b88122c41f`, `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`, `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`, `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc`, and `c0576191407a1ad53db77c0635c7b0820669e547`.
- SteelMC gitlink: `e655880a7b1487b2444c70013c14ee86c69d53b3`; no submodule update was made.
- `.gitmodules` points to `https://github.com/Steel-Foundation/SteelMC.git`.
- Experimental branch `phase-2.2-b-persistent-local-profile` remains separate and was used only as reference material.
- PR #11 remains open and unmerged. `master` was not modified.
- CI run `41009545949` targeted source HEAD `c0576191407a1ad53db77c0635c7b0820669e547`; its `test` job was still running while lint/fmt/Android/iOS/singleplayer jobs were cancelled after newer commits superseded the run.
- CI run `41009608515` targeted this documentation HEAD `9258c2118f744a17591a0dba75a50d3c5f16fa38` and was pending at the last check.
- No complete CI pass is established. No local Cargo or Android-device test was run.
- GitHub connector access does not expose the user's local Termux worktree; local cleanliness must be checked on-device.

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
