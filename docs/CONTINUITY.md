# Pomme Android — Continuity / Handoff

Last updated: 2026-10-30

## Current phase

**Phase 2.2-B1–B4 — profile-store safety and persistent local identity implementation.**

Implementation was explicitly authorized by the user on 2026-10-30. Source changes have been committed to the authorized branch. CI/device verification is pending; do not call this phase complete yet.

## Repository state

- Repository: `sylphie07/Pomme-Android-Test`
- Authorized branch: `unofficial-android-menu-access`
- Branch HEAD at the start of the implementation session: `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc`
- Source commits:
  - `4bd084c8a9fca176da2545b800a8a2eb24c7b3e9` — profile model, validation, recovery, and tests.
  - `334f5f958bf99a61bc3d67bcc48036b88122c41f` — fail safely during app startup if profile storage cannot be loaded.
  - `0510cdc0735f1904f84c6e4c4aa436eb83bde1dc` — cleanup of recovery test assertions.
  - `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce` — ensure new profiles retain the runtime offline UUID and migrate experimental format 1 to format 2.
  - `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc` — apply rustfmt layout to the profile-store code/tests.
  - `c0576191407a1ad53db77c0635c7b0820669e547` — restore the exact validated backup snapshot instead of reopening the backup path.
  - `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce` — preserve runtime UUID compatibility for new profiles and migrate experimental format-1 identities.
- Latest verified branch HEAD before this continuity refresh: `df0305e3d9d10b1efcbf7f993b792ca40924bbab`. This continuity refresh itself creates a newer documentation-only commit; re-check the branch ref before the next edit.
- SteelMC gitlink: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- `.gitmodules`: `https://github.com/Steel-Foundation/SteelMC.git`.
- No SteelMC update was made.
- `master` was not modified. No merge, release, or APK was performed.
- GitHub cannot establish whether the user's local Termux worktree has uncommitted changes; check it separately on-device.

## Implemented source changes

### `pomme-client/src/user.rs`
- Added `LocalProfile.player_uuid`, independent from `LocalProfile.id`. The current JSON format is version 2.
- Added versioned JSON; the current document format is version 2.
- New profiles start with the historical `offline_uuid(username)` used by the current runtime.
- Experimental format-1 records that stored random UUIDs migrate to the historical runtime UUID before the field is used by SteelMC. Legacy documents also migrate to the historical runtime UUID.
- Creating a profile that collides with an existing historical player UUID is rejected rather than silently assigning a different identity.
- Legacy documents and experimental format-1 documents receive the historical offline UUID derived from their currently stored username. Profile IDs and selected-profile ID are preserved. Newly created profiles also begin with the historical runtime UUID, so wiring the field into C/D will not change existing world keys.
- Reject malformed, incomplete, unsupported, duplicate-ID, duplicate-player-UUID, invalid-name, and case-insensitive duplicate-name records without silently reassigning identity.
- Existing invalid primary JSON is no longer replaced with a fresh default profile.
- A valid `.bak` may recover a corrupt or missing primary. A corrupt primary is retained under a unique `.corrupt-<uuid>` filename.
- Future format versions return `ErrorKind::Unsupported` and are not downgraded from an older backup.
- Before replacing the backup, the exact existing primary bytes are validated so a corrupted primary cannot overwrite the known-good backup. Backup recovery writes the same validated byte snapshot rather than reopening the file.
- Added regression tests for corrupt-primary recovery, preservation of the corrupt original, future-version downgrade prevention, and duplicate names, alongside the experimental tests.

### `pomme-client/src/lib.rs`
- `LocalProfileStore::load_or_create` now returns `Result`.
- Startup logs the failure and exits rather than silently continuing with a newly generated profile if existing profile data is invalid or unrecoverable.

## Identity boundary — important

The new `player_uuid` is currently stored and migrated, but `UserData::from_local_profile()` still derives the runtime UUID from the username. New profiles and experimental format-1 profiles now store that same historical UUID, so the later C/D wiring can preserve current world keys. This is deliberate: the trusted identity handoff into the embedded SteelMC connection is Phase 2.2-C/D, not completed by B1–B4. New and migrated profile UUIDs now match the historical runtime UUID, so C/D must consume that exact value rather than regenerate it.

Do not claim that player UUID continuity is already active in live world sessions. Do not claim advancement persistence; the inspected SteelMC pin has a separate advancement-save TODO.

## Verification state

- GitHub source writes: confirmed by returned commit SHAs.
- GitHub branch HEAD and SteelMC gitlink: verified after source commits.
- CI run `41009217544` (run #70) targeted source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`. At the latest poll, `test` and `singleplayer` were cancelled; Android, `fmt`, and Ubuntu/macOS lint were still in progress; iOS and Windows lint were cancelled. No complete successful CI result is established.
- Earlier runs #58/#59/#63 were failed or cancelled and do not establish a pass for the latest source.
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

## Documentation commit chain

The implementation-session documentation commits were:
- `b590d42d543d724ddf542cf1ed8ec9c84933c822`
- `216154300bc94955eec92de9270be1e1844c4831`
- `8fc6e15b62c9baab4a4de6a02ec5b8fffca03ccd`
- `465241fa0de6f9ebaf977a47c5b6829fb6c60e1f`
- `735d6b528bc68c3730cc793d2046933b33bbe624`
- `790f7a9a973cf2384de91b1bad6b3c89315ee1a8`
- `aee098d4ca5b5afc4ecb9533657f3de8c405784e`

## Cross-chat compatibility correction — 2026-10-30

Another investigation chat identified that the experimental implementation generated random `player_uuid` values for newly created profiles even though the runtime still used `offline_uuid(username)`. This would have created a data-key transition risk in C/D. Corrected in source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`:
- New profiles now set `player_uuid` to the historical offline UUID.
- A UUID collision with another profile is rejected instead of silently generating a different identity.
- JSON format is now version 2. Experimental format 1 migrates its unused random UUID to the historical offline UUID, preserving profile IDs and the selected profile.
- Added a regression test for format-1 migration.
- Backup recovery now writes the exact bytes validated from the backup, avoiding a validation/read race.
- Formatting cleanup commit: `c9e5909bd783555d7d15ef4e25a1ffbe831b59cc`.
- CI run `41009337238` for this PR head was pending at the last check.

## Session update — 2026-10-30

The user authorized code edits to integrate the other chats' findings. The experimental branch was used as reference only; no cherry-pick or merge was performed. Source commits are listed above. Documentation is being synchronized with the actual implementation and verification status.


## Session 007 — Cross-chat audit and profile UUID compatibility follow-up (2026-10-30)

### Repository verification
- Repository: `sylphie07/Pomme-Android-Test`.
- Branch: `unofficial-android-menu-access`.
- Exact branch HEAD before this continuity update: `df0305e3d9d10b1efcbf7f993b792ca40924bbab`, obtained from the live branch ref.
- SteelMC pin remains recorded as `e655880a7b1487b2444c70013c14ee86c69d53b3`; no SteelMC source or pin was changed. The comparison from the prior audited point showed no `third_party/SteelMC` path change.
- PR #11 remained open; no review comments or approvals were found.

### Source findings
- `authorize_gameplay()` centralizes the observed `Connect`, `PlayWorld`, `open_world()`, and QuickPlay gates. Menu navigation itself is not globally blocked; local singleplayer is allowed without Microsoft auth; remote multiplayer requires a verified, unexpired Microsoft session.
- Startup QuickPlay is routed through `in_menu::quick_play()` after setup. Focused tests for restore/cancel races and exactly-once pending-action resumption are still absent from `in_menu.rs`.
- `LocalProfile.player_uuid` remains stored but runtime `UserData::from_local_profile()` still derives the active UUID from the username; trusted SteelMC identity remains C/D work.
- A compatibility issue was found: new profiles had random stored UUIDs even though pre-C/D world data was keyed by `offline_uuid(username)`. The programmer chat addressed this in source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`: new profiles now use the historical runtime UUID; the document schema advances to version 2; experimental format-1 random UUIDs migrate back to the runtime UUID; duplicate historical UUID creation is rejected.
- Profile management UI is not yet integrated; store APIs for list/create/select and `set_local_profile()` remain unused/reserved.
- Account code still has a long-session token-refresh TODO and Android Keystore TODO. Android singleplayer feature and an ignored embedded-world enter/re-enter test exist, but no device acceptance result was obtained. The APK workflow does not automatically build this feature branch on PRs.
- Advancement persistence remains a separate SteelMC limitation.

### CI state
- Latest relevant run: `41009217544` (run #70), for source commit `6b78afb8aca27e9a3aad3c5077ee223f0d8967ce`.
- At the latest poll, `test` and `singleplayer` were cancelled; Android, `fmt`, and Ubuntu/macOS lint were still in progress; iOS and Windows lint were cancelled. This is not a green run.
- Previous run #63 failed `ci-fmt` on the earlier profile source; run #70 superseded it but has not produced a complete validation result.

### Documentation changes from this audit
- `docs/KNOWN_ISSUES.md`: recorded menu/auth source status, current CI, UUID migration resolution, profile UI gap, test gaps, token/Keystore TODOs, Android APK workflow scope, and advancement persistence.
- `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`: added implementation handoffs and updated B1–B4/UUID compatibility status.
- `docs/DECISIONS.md`: reconciled menu/auth and profile migration decisions with source state.
- `docs/PROJECT_CONTEXT.md`: updated profile schema and migration behavior to version 2.
- No source code, Rust/build configuration, workflows, SteelMC, worlds, profiles, player data, APKs, or `master` were changed by this investigation chat.
- No local tests, builds, formatting commands, or Android device checks were run by this investigation chat.

### Documentation commits from this audit
- `d54f2c704d94c114c58b1a4f90a3d1f30d17c98a` — known issues and verification handoffs.
- `8f18d48d8a4a2e4108325b91cecb17e3f21928dd` — detailed implementation handoffs.
- `6238ec2df2d4cfef0264041409c9784a54efb1ac` — align B1–B4 authorization.
- `309a583548095faf1870a3a6c07563e2e08f16b5` — reconcile technical decisions.
- `cfbf5cb0fd92fe660ac8aefb86945e745763d381` — record CI run #63.
- `b1c6e1cdca7061bc51ea0ee63836e088d05da9a5` — refresh CI handoff.
- `b55664d23fa77bb6830bb47e221be3a91e761c73` — update UUID compatibility and CI status.
- `03a067a940064a43a8497da21441976531317671` — refresh programmer handoff.
- `df0305e3d9d10b1efcbf7f993b792ca40924bbab` — update project context for profile format 2.

### Next safe actions
1. Dedicated programming chat: wait for a complete CI run on the latest source; resolve remaining formatter/lint/test failures before calling B1–B4 verified.
2. Before C/D: design and test trusted per-connection identity while preserving the now-stable historical UUID exactly.
3. Keep profile picker UI, token refresh/Keystore, Android device discrepancy, and advancement persistence as separate scoped tasks.
4. Do not begin C/D/E, update SteelMC, merge, publish, or generate an APK without their own explicit authorization.


## Session 009 — Close CI-discovered B1–B4 compile and format gaps (2026-10-30)

### Repository verification before edits
- Repository: `sylphie07/Pomme-Android-Test`.
- Authorized branch: `unofficial-android-menu-access`.
- Initial live branch HEAD: `f7ae3ce885b4e0e3d1c50f866201351d9549ef68`.
- `master` remains `a147ca27e24850e5d6e796e1dc7bd02e1ecf3a72`.
- SteelMC gitlink remains `e655880a7b1487b2444c70013c14ee86c69d53b3`; `.gitmodules` still points to `https://github.com/Steel-Foundation/SteelMC.git`.
- No SteelMC source or gitlink changes were made.

### CI evidence and defects found
- Run `41009217544` was cancelled; it did not validate B1–B4.
- The first complete CI attempt on the then-current docs/source HEAD, run `41009743471` (#84), failed. Logs identified:
  - Seven test references to `LOCAL_PROFILE_FORMAT_VERSION` missing from the child test module's imports.
  - `headless.rs` treated `LocalProfileStore::load_or_create()` as a direct store even though it now returns `Result`.
  - `cargo fmt --check` found two formatting differences in `user.rs`.
  - The required aggregate check consequently failed. Android, iOS, test, singleplayer and lint were not green.
- The run exposed real source errors, not merely a cancelled/incomplete workflow.

### Source corrections applied
- `bcca539e67e85959e99ce7bf39ce000065e02ffa` — imported the format-version constant into the test module and applied the two exact rustfmt changes reported by CI.
- `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e` — updated the headless profile-store call site to handle storage errors by logging and exiting rather than proceeding without the saved profile.
- These corrections preserve the fail-closed storage behavior; no identity/authentication policy or SteelMC code was changed.

### Current verification state
- Current branch HEAD after source corrections: `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e`.
- Fresh CI runs associated with this source HEAD were observed: `41010632204` and `41010632271` (pending at the last poll). Runs `41010627093` and `41010626979` for the immediately preceding source commit were still in progress at that poll.
- No CI run for the corrected HEAD has yet been confirmed complete and green.
- No local `cargo fmt`, `cargo test`, Android build, or Android device test was run in this session.
- Tests remain written but not yet confirmed executed successfully on the corrected HEAD.

### Scope and next action
1. Poll CI for HEAD `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e`; inspect all failed jobs/logs and wait for the required aggregate `ci` job.
2. Review the complete B1–B4 source diff and verify migration/recovery/compatibility test coverage.
3. Prepare a documentation-only C/D trusted-identity design. Do not implement C/D or modify SteelMC without separate authorization.
4. Keep profile selector UI, Android Keystore, and advancement persistence separate.
5. Do not merge, publish, create an APK, change `master`, or change the SteelMC pin.


## Phase 2.2-C/D design prepared — documentation only (2026-10-30)

- Added `docs/PHASE_2_2_C_D_TRUSTED_IDENTITY_DESIGN.md` at commit `54a673e5138e8e1cf843a72d527fec8ad644a9ea`.
- The design follows the inspected Pomme in-menu → `pomme-singleplayer::LaunchOptions` → Steel `JavaTcpClient::from_transport` path and Steel offline login behavior at pin `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- Key security finding: Steel's offline `handle_hello` deliberately ignores the client-supplied hello UUID and derives `offline_uuid(username)`. Therefore changing only Pomme's packet UUID cannot implement persistent identity.
- Proposed channel: a typed server-side identity supplied from the selected persisted local profile through the in-process embedded-server Rust API into a per-connection trusted context. Ordinary TCP connections get no such context; online-mode logins continue through Mojang authentication; the hello UUID remains untrusted.
- This is a proposal only. No C/D source, SteelMC code, submodule pin, remote authentication, profile selector UI, Keystore, or advancement behavior was changed.
- Required test matrix and compatibility invariants are documented in the design file. C/D still require separate authorization.

### Current source/CI checkpoint
- Source-fix HEAD: `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e`.
- Follow-up documentation commit for design: `54a673e5138e8e1cf843a72d527fec8ad644a9ea`.
- The only completed CI result on the preceding source HEAD `f7ae3ce885b4e0e3d1c50f866201351d9549ef68` is failure `41009743471`; it exposed and motivated the source fixes.
- Runs `41010632204` and `41010632271` were auxiliary `pull_request_target` conflict/label workflows, not Rust CI. They must not be counted as source validation.
- No completed Rust CI run for the corrected source HEAD has been confirmed yet. Continue checking for a `pull_request` run; if no run was triggered, use an authorized GitHub Actions rerun/dispatch path or ask the user to trigger the CI workflow from GitHub. Do not call B1–B4 verified until the required `ci` job and its dependent code jobs pass.


### Session 009 follow-up — second CI formatter pass
- Rerun attempt 2 of workflow `41010633965` reached `ci-fmt` and exposed two additional rustfmt layout requirements: the headless profile-store expression and the nested test-module import ordering.
- Applied the exact formatter output:
  - `1d7256ec025dd184f9a910aad724d9468341191d` — format the headless load/error expression.
  - `406a3d63db222f897338047dae40419baed18db1` — order test imports as rustfmt requires.
- The workflow attempt was superseded by these source commits; it is not a passing validation. The latest source tree must receive a complete CI run after the final branch updates.
- No behavior change beyond the already documented fail-closed headless handling; no SteelMC change.


## Session 011 — Extend profile validation regression coverage (2026-10-30)

- During the full B1–B4 diff review, added focused regression tests for three invariants already enforced by `decode_document`: duplicate profile-record IDs, duplicate player UUIDs across different names, and a selected-profile ID that does not exist.
- These tests complement existing legacy/format-1 migration, backup recovery, corrupt-primary preservation, future-version downgrade prevention, case-insensitive duplicate-name, and account/auth compatibility tests.
- Source and documentation updates are committed together to provide one stable CI-triggering branch update.
- The latest complete Rust CI result remains pending; prior run `41010633965` failed the two newly reported formatting checks before being superseded. Do not call B1–B4 verified until a complete successful CI run finishes.
- No authentication logic, C/D/E code, SteelMC source/pin, world data, merge, release, or APK was changed.


### Session 012 — Correct format-1 migration test fixture
- CI run `41011216161` (#92), source HEAD `ff37877abd2aef8a1902859c55ce1744f96c607c`, confirmed `fmt`, Android, iOS, and lint jobs green (Ubuntu, macOS, Windows), but `test` failed.
- The failure was isolated to `experimental_format_one_migrates_to_the_runtime_offline_uuid`: its fixture username `ExperimentalPlayer` exceeded the existing 16-character local-name limit. The production validation correctly rejected the invalid fixture; this was a test-data defect, not a migration-code defect.
- Replaced the fixture name consistently with valid `ExpPlayer`. This does not change production code or accepted username rules.
- New source commit: `8362a0b20b5627db1e66e4954f116165ab591b38`.
- Run #92's singleplayer job was still in progress at the last check; regardless, the failing test job means #92 cannot validate B1–B4. A fresh full CI run on the corrected HEAD is required.
- No SteelMC, C/D/E, authentication, profile UI, Keystore, advancement persistence, merge, publication, or APK changes.


## 2026-10-30 — Phase 2.2-B1–B4 CI closed; C/D authorized

### Verified checkpoint
- PR #11 is open, unmerged, and targets `master`; branch: `unofficial-android-menu-access`.
- PR HEAD at the checkpoint: `dedebc588065782768665d80b9fc3f6ca3281044`.
- CI run `41012173438` (#94) completed successfully: `test`, `singleplayer`, `android`, `ios`, `fmt`, Ubuntu/macOS/Windows lint, PR template, changes, and aggregate `ci` all succeeded. `launcher` and `launcher-frontend` were skipped.
- This closes the B1–B4 automated CI blocker. It does not establish Android device acceptance.
- `.gitmodules` identifies `third_party/SteelMC` as `https://github.com/Steel-Foundation/SteelMC.git`. The live gitlink SHA was not independently read during this check; the old documented pin must not be assumed current.

### Authorization correction
The user explicitly authorized Phase 2.2-C and 2.2-D implementation, including necessary SteelMC submodule changes. This supersedes earlier statements in this document and older handoffs that C/D were unauthorized or blocked pending B1–B4 CI. Authorization does not include merging PR #11, modifying `master`, publishing an APK, or unrelated workstreams.

### Programmer handoff
- Authoritative implementation order and acceptance criteria: `docs/PHASE_2_2_CD_PROGRAMMER_HANDOFF.md`.
- Technical architecture baseline: `docs/PHASE_2_2_C_D_TRUSTED_IDENTITY_DESIGN.md`.
- Before edits, programmer must freshly verify branch/HEAD, PR state, worktree, actual SteelMC gitlink and pinned source, and complete B1–B4 diff. The checkpoint SHA is not a substitute for live verification.
- Preserve exact persisted `LocalProfile.player_uuid` and historical world-data keys. Never trust client Hello UUIDs; ordinary offline TCP remains unchanged; remote multiplayer retains Microsoft verification.
- Do not modify `master`, merge/close PR #11, publish an APK, or delete/alter user data.
- Phase 2.2-E real Android existing-world acceptance remains subsequent work and must be reported separately.

### Scope of this documentation update
Only repository documentation was changed. No Pomme source, SteelMC source/gitlink, build configuration, workflows, world/profile/player data, APK, or `master` was changed by this handoff session.