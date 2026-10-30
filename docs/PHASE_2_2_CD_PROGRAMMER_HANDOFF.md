# Phase 2.2-C/D — Authorized programmer handoff

Date: 2026-10-30
Repository: `sylphie07/Pomme-Android-Test`
Authorized branch: `unofficial-android-menu-access`
PR: #11 → `master` (must remain open and unmerged)
Last observed PR HEAD: `dedebc588065782768665d80b9fc3f6ca3281044`
CI run: `41012173438` (#94), all applicable jobs completed successfully.

## 1. Task and priority

**Priority: next implementation task after closing Phase 2.2-B1–B4.**

Implement:
- **Phase 2.2-C:** a trusted, typed, server-side identity handoff for the embedded SteelMC connection, scoped to that concrete connection.
- **Phase 2.2-D:** pass the selected persisted local profile's exact `player_uuid` from Pomme through the embedded singleplayer launch path into that trusted handoff.

The user explicitly authorized C/D implementation, including necessary SteelMC submodule changes, in the current conversation on 2026-10-30. This authorization supersedes older documentation sentences that say C/D are not authorized. This handoff records that authorization; it does not authorize merge, master changes, release, APK publication, or unrelated follow-up work.

## 2. Mandatory preflight — do not skip

Before editing:
1. Read all of `docs/CONTINUITY.md`, `docs/PROJECT_CONTEXT.md`, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, `docs/CHAT_HANDOFF_RECOMMENDATIONS.md`, this handoff, and `docs/PHASE_2_2_C_D_TRUSTED_IDENTITY_DESIGN.md`.
2. Verify the live branch is exactly `unofficial-android-menu-access`; verify live HEAD, PR #11 head/base/state, recent commits, and working-tree status. The recorded HEAD above is a checkpoint, not permission to assume the branch has not moved.
3. Verify the actual SteelMC gitlink using Git tree metadata / `git ls-tree HEAD third_party/SteelMC` and `git submodule status`; inspect the pinned SteelMC commit itself. Do not rely on an old documented pin. The last previously documented pin is `e655880a7b1487b2444c70013c14ee86c69d53b3`, but it must be independently rechecked.
4. Confirm the final B1–B4 source revision is included in the current branch and review its complete diff. Do not repeat or rewrite B1–B4 unless a concrete regression is found.
5. Confirm CI run #94 corresponds to the current PR HEAD before relying on it. Recorded result: `test`, `singleplayer`, `android`, `ios`, `fmt`, Ubuntu/macOS/Windows lint, PR template, changes, and aggregate `ci` all succeeded; launcher jobs were skipped. CI is not Android device acceptance.

If branch, HEAD, PR state, gitlink, or working tree differs, reconcile and document before proceeding. Do not discard or overwrite uncommitted user work.

## 3. Design authority

Use `docs/PHASE_2_2_C_D_TRUSTED_IDENTITY_DESIGN.md` as the technical starting point, but revalidate every symbol and call path against current source. Expected route to inspect includes:
- Pomme's selected `core.user.local_profile` and `open_world()`.
- `pomme-singleplayer::LaunchOptions`.
- Embedded Steel server construction and the concrete memory/in-process connection.
- Steel login handshake and server-side `GameProfile` creation.
- `ConnectArgs` / client-side session bookkeeping.

Do not assume the proposed exact API shape is correct until inspected. Prefer a typed, explicit internal Rust API over ambient/global state, packet fields, username conventions, or listener-wide mutable identity.

## 4. Non-negotiable security invariants

1. Only the embedded Pomme singleplayer launch path may supply trusted local identity.
2. Identity must be bound to the specific accepted embedded connection / player admission, not a shared listener or reusable server-wide session field that another connection can inherit.
3. Never trust a UUID supplied by the client in an ordinary login Hello packet. Changing the packet UUID alone is not a solution.
4. Ordinary offline TCP clients without an explicit trusted embedded context must retain existing offline UUID behavior.
5. Microsoft online-mode authentication and verification must remain intact. A local trusted identity must never bypass Mojang/Microsoft authentication for remote multiplayer.
6. Remote multiplayer must continue to use the verified, unexpired Microsoft session identity/token. Local profile identity must not leak into remote `ConnectArgs`.
7. A mismatched login username must be rejected or handled by a clearly documented canonical-name rule with tests. Do not silently bind an unrelated name to the persisted UUID.
8. Do not weaken gameplay authorization gates or menu authentication policy.
9. Keep `LocalProfile.id` (profile-record ID) separate from `LocalProfile.player_uuid` (player identity).

## 5. Compatibility and user-data invariants

- For every existing local profile, the server-side `GameProfile.id()` must equal the exact persisted `player_uuid`.
- B1–B4 deliberately made new and migrated profiles retain the historical `offline_uuid(username)` used by existing world/player-data keys. Do not regenerate, normalize, hash again, or derive a replacement UUID in C/D.
- Do not copy, rename, migrate, or rewrite world/player-data files as a workaround.
- Preserve profile IDs, selected-profile ID, username, and exact persisted UUID.
- Account sign-in, sign-out, refresh, and switching must not overwrite the selected local profile identity.
- Steel player data and statistics are keyed by server-side profile UUID; verify the actual current storage paths and every relevant UUID-keyed subsystem.
- Advancement persistence is a separately documented SteelMC limitation. Do not claim C/D fixes it.

## 6. Implementation order

### C — SteelMC trusted context
1. Trace the current login and connection lifecycle from the embedded server constructor to per-connection login state and server-side profile creation.
2. Design the narrowest typed trusted-local-identity carrier that can only be constructed by the embedded server integration.
3. Bind it to the concrete connection/player login, with explicit ownership and lifetime. Prove ordinary TCP constructors cannot receive or inherit it.
4. At the server-side profile/admission boundary, use the trusted identity only for the embedded local connection; keep normal offline and online-mode paths unchanged.
5. Add Steel-side regression tests before plumbing Pomme into it.

### D — Pomme plumbing
1. Add the persisted `local_profile.player_uuid` to the embedded launch options through an explicit typed field.
2. Ensure the embedded in-memory connection and server-side profile resolve to that exact UUID.
3. Ensure client-side `ConnectArgs.uuid` / bookkeeping matches the same persisted UUID without becoming the trust source.
4. Keep remote connection creation on the existing verified Microsoft identity path; add explicit negative tests for accidental local-identity use.
5. Keep identity immutable for the lifetime of a connection; profile switching affects only a subsequent launch.

### Review after implementation
- Review the full Pomme and SteelMC diffs, including submodule pin change and generated/build changes.
- Explain why every changed SteelMC file is necessary and whether a gitlink update is required.
- Keep commits logically separated (SteelMC C, Pomme D, tests/docs) where the repository/submodule workflow permits.

## 7. Required tests / acceptance criteria

### SteelMC
- Embedded trusted identity becomes the admitted server-side `GameProfile.id()`.
- A deliberately different client Hello UUID cannot override it.
- Username mismatch is rejected or follows the tested canonical-name policy.
- Ordinary offline TCP with no trusted context still uses `offline_uuid(username)`, ignoring Hello UUID.
- Online-mode login still requires the existing Mojang verification path.
- A normal TCP connection cannot inherit identity from an embedded connection or shared listener/session.
- Two simultaneous or sequential embedded connections cannot cross-contaminate identities.

### Pomme
- `open_world()` passes the selected profile's exact stored `player_uuid`.
- Embedded `ConnectArgs.uuid` equals that exact UUID.
- A conflicting launch argument or client-supplied UUID cannot override the trusted selected profile.
- Remote `ConnectArgs` require a valid Microsoft session and use the Microsoft identity/token.
- Legacy and format-1 migrated profiles reach Steel with the same UUID the old runtime used.
- Account switching does not mutate local profile identity.
- Switching local profile selection affects only the next connection.

### Existing-world compatibility
Where automated test fixtures permit, assert server profile UUID equals the historical offline UUID used to create the fixture's player data, and that the expected player data record is selected. Do not create destructive migration logic.

Real device acceptance remains Phase 2.2-E and requires separate planning with the user. No deletion of worlds, app data, profiles, or player data is acceptable.

## 8. Explicit exclusions

Do NOT:
- merge PR #11 or change `master`;
- close/rebase/retarget PR #11 without instruction;
- publish or distribute an APK;
- change unrelated menu/authentication behavior;
- implement profile picker UI, token refresh, Android Keystore, advancement persistence, or performance work;
- delete or rewrite user worlds, profiles, player files, or app data;
- claim Phase 2.2-E/device acceptance complete.

## 9. Required reporting and repository documentation

Before first source edit, report the verified branch/HEAD, actual SteelMC pin, planned files, architecture, and tests. After implementation:
- list exact commits and changed files in both repositories;
- list exact commands and tests actually run, with pass/fail/skipped status;
- report CI run IDs and all job conclusions;
- distinguish implemented, locally tested, CI-verified, and device-verified;
- document unresolved risks and the next safe step;
- update `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, and the design/handoff docs so old statements that C/D are unauthorized or blocked on B1–B4 are explicitly superseded.

**Do not claim C/D complete until the security tests and compatibility criteria above pass.**
