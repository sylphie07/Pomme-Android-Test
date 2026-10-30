# Pomme Android — Continuity / Handoff

Last updated: 2026-10-30

## Current phase
**Phase 2.2-A — Design persistent player identity per local profile.**

Status: investigation/design only. The final report from the separate analysis chat is still pending review in the coordinating chat. Do not implement Phase 2.2-B or make source changes until the user explicitly approves the design.

## Current objective
Give each local profile a stable player identity that survives profile-name changes, while preserving existing singleplayer player data and keeping local identity separate from Microsoft/Java authentication.

Target behavior:
- Menu navigation remains available while signed out.
- Local profiles can be used for offline singleplayer.
- A profile's player UUID remains stable when its displayed username changes.
- Existing profiles preserve their historical name-derived offline UUID during migration, if source verification confirms this is safe.
- Generic offline login must not accept arbitrary client-supplied UUIDs as trusted identity.
- Online play retains the official authentication requirements.

## Last known repository information
- Repository: `sylphie07/Pomme-Android-Test`
- Working branch: `unofficial-android-menu-access` (branch existence verified on 2026-10-30).
- Latest commit surfaced by repository commit search: `a147ca27e24850e5d6e796e1dc7bd02e1ecf3a72`, message: “Add 'singleplayer' feature to pomme-client dependency”.
- Its inspected diff changes `pomme-android/Cargo.toml` to enable `features = ["singleplayer"]` on the Android `pomme-client` dependency.
- Previous known HEAD `c730ae9e1334646f507f9f068e8774e8950a18a8` is outdated.
- Previous known SteelMC submodule SHA: `e655880a7b1487b2444c70013c14ee86c69d53b3`; re-check it before implementation.
- Exact current branch-ref HEAD was not exposed by the available branch-search result; verify it before any code work.
- No source changes, commits, merges, PRs, or APK builds were performed during this documentation update.

## Audit findings recorded from the previous report
1. Pomme derives the current offline player UUID from the username.
2. SteelMC offline login recalculates the UUID from the requested username rather than trusting the UUID in the Hello packet.
3. SteelMC persistence uses the game profile UUID as a key; reported areas include global player data, domain player data, stats, permissions, and known players.
4. A rename therefore changes identity and may make existing data appear absent while old files remain.
5. `LocalProfile.id` identifies the profile record; it should not be repurposed as the player identity.
6. Candidate design is a new `LocalProfile.player_uuid` field.
7. Candidate migration for old profiles: initialize `player_uuid` to the historical offline UUID of the existing username, retaining `LocalProfile.id`.
8. The embedded server needs a trusted internal identity handoff. Do not weaken generic offline login.
9. Advancement persistence and any other unverified storage categories must be explicitly investigated.

These are report-derived findings and remain subject to source-level verification, especially in the SteelMC submodule at its exact pinned commit.

## Immediate next action
Obtain the complete Phase 2.2-A report from the other chat. Review it against the exact repository and SteelMC source revisions. Resolve the trusted identity handoff design and data-compatibility questions. Only then present a Phase 2.2-B implementation plan for user approval.

## Mandatory startup checklist for the next chat
1. Read `docs/PROJECT_CONTEXT.md`, this file, `docs/DEVELOPMENT_LOG.md`, `docs/DECISIONS.md`, and `docs/PHASE_2_2_A_PERSISTENT_PLAYER_IDENTITY.md`.
2. Verify the exact branch and HEAD; do not rely on the historical SHA above.
3. Inspect recent commits and current working state before editing.
4. Verify the SteelMC submodule URL and pinned SHA.
5. Review the pending Phase 2.2-A report and distinguish claims supported by source from assumptions.
6. Do not modify source until the user authorizes implementation.

## Handoff requirements
Every future handoff must record exact branch/HEAD, submodule SHA, files inspected/changed, decisions, tests actually run and exact outcomes, unresolved risks, and the next concrete action.
