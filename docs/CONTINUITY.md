# Pomme Android — Continuity / Handoff

Last updated: 2026-10-30

## Current phase

**Phase 2.2-B1–B4 — profile-store safety and persistent local identity implementation.**

<badge> </badge>
Implementation was explicitly authorized by the user on 2026-10-30. Source changes have been committed to the authorized branch. CI/device verification is pending; do not call this phase complete yet.

## Repository state

- Repository: `sylphie07/Pomme-Android-Test`
- Authorized branch: `unofficial-android-menu-access`
- Branch HEAD at the start of the implementation session: `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`
- Source commits:
  - `4bd084c8a9fca176da2545b800a8a2eb24c7b3e9` — profile model, validation, recovery, and tests.
  - `334f5f958bf99a61bc3d67bcc48036b88122c41f` — fail safely during app startup if profile storage cannot be loaded.
  - `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` — cleanup of recovery test assertions.
- The current live branch HEAD must be rechecked before the next edit because documentation commits follow these source commits.
- SteelMC gitlink: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- `.gitmodules`: `https://github.com/Steel-Foundation/SteelMC.git`.
- No SteelMC update was made.
- `master` was not modified. No merge, release, or APK was performed.
- GitHub cannot establish whether the user's local Termux worktree has uncommitted changes; check it separately on-device.

## Implemented source changes

### `pomme-client/src/user.rs`
- Added `LocalProfile.player_uuid`, independent from `LocalProfile.id`.
- Added JSON `format_version = 1`.
- Legacy documents receive the historical offline UUID derived from their currently stored username. Profile IDs and selected-profile ID are preserved.
- Reject malformed, incomplete, unsupported, duplicate-ID, duplicate-player-UUID, invalid-name, and case-insensitive duplicate-name records without silently reassigning identity.
- Existing invalid primary JSON is no longer replaced with a fresh default profile.
- A valid `.bak` may recover a corrupt or missing primary. A corrupt primary is retained under a unique `.corrupt-<uuid>` filename.
- Future format versions return `ErrorKind::Unsupported` and are not downgraded from an older backup.
- Before replacing the backup, the exact existing primary bytes are validated so a corrupted primary cannot overwrite the known-good backup.
- Added regression tests for corrupt-primary recovery, preservation of the corrupt original, future-version downgrade prevention, and duplicate names, alongside the experimental tests.

### `pomme-client/src/lib.rs`
- `LocalProfileStore::load_or_create` now returns `Result`.
- Startup logs the failure and exits rather than silently continuing with a newly generated profile if existing profile data is invalid or unrecoverable.

## Identity boundary — important

The new `player_uuid` is currently stored and migrated, but `UserData::from_local_profile()` still derives the runtime UUID from the username. This is deliberate: the trusted identity handoff into the embedded SteelMC connection is Phase 2.2-C/D, not completed by B1–B4.

Do not claim that player UUID continuity is already active in live world sessions. Do not claim advancement persistence; the inspected SteelMC pin has a separate advancement-save TODO.

## Verification state

- GitHub source writes: confirmed by returned commit SHAs.
- GitHub branch HEAD and SteelMC gitlink: verified after source commits.
- CI for source HEAD `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`: run `41008897990` was pending when last checked.
- Earlier CI run `41008828793` targeted parent source HEAD `334f5f958bf99a61bc3d67bcc48036b88122c41f` and was still in progress; it is not the final validation target.
- No local `cargo test`, `cargo fmt`, Android build, or Android device test was run in this session.
- No test is yet claimed as passed for these new changes.

## Remaining work and boundaries

1. Wait for CI on the final source revision; fix compile, formatting, lint, and test failures if any.
2. Review the complete source diff after CI.
3. Check all call sites and feature configurations for the changed `load_or_create` result type.
4. B1–B4 are not complete until automated checks pass and compatibility concerns are addressed.
5. Phase 2.2-C (trusted per-connection identity in SteelMC) and 2.2-D (Pomme plumbing into the integrated connection) remain separate downstream work. Do not alter the SteelMC gitlink without explicit scope and justification.
6. Phase 2.2-E requires existing-world and Android-device validation. User must not be instructed to delete worlds or app data.
7. SteelMC advancement persistence remains a separate issue.

## Required next-chat startup checklist

1. Read `docs/PROJECT_CONTEXT.md`, this file, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`, and the Phase 2.2-A report.
2. Verify exact live branch HEAD, recent commits, and SteelMC gitlink again.
3. Inspect the full diff and latest CI run for the actual branch HEAD.
4. If working from Termux, inspect local branch, HEAD, and `git status` separately.
5. Preserve all worlds, profiles, and player data. Never regenerate UUIDs to silence an error.
6. Do not implement C/D/E, update SteelMC, merge, publish, or generate an APK without the relevant explicit authorization.

## Session update — 2026-10-30

The user authorized code edits to integrate the other chats' findings. The experimental branch was used as reference only; no cherry-pick or merge was performed. Source commits are listed above. Documentation is being synchronized with the actual implementation and verification status.
