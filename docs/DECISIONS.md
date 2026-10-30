# Pomme Android — Technical Decisions and Constraints

## D-001 — Keep work on the authorized Android branch
- Date: 2026-10-30
- Decision: use `unofficial-android-menu-access` for the current Android menu/authentication investigation.
- Constraint: do not modify `master` or another branch without explicit user authorization.
- Reason: preserve separation between the Android investigation and the repository's other development lines.

## D-002 — Separate menu access from gameplay authentication
- Date: 2026-10-30
- Decision / target behavior: signing in should not be required merely to navigate the menu; authentication checks must remain on gameplay paths that require them.
- Status: source implementation is present: menu browsing is not globally gated, local singleplayer is permitted without Microsoft authorization, and remote multiplayer is gated by a verified/unexpired session. Automated end-to-end verification remains incomplete.
- Reason: avoid blocking menu access while preserving necessary protections for actual play.

## D-003 — Audit every gameplay entry route
- Date: 2026-10-30
- Decision: do not patch only one visible button. Trace `MenuAction::Connect`, `MenuAction::PlayWorld`, `open_world()`, and quick-start/startup arguments, then centralize checks only after understanding the call graph.
- Status: source review completed for the main menu actions and startup QuickPlay path; focused regression tests remain outstanding.

## D-004 — Treat Android singleplayer as a separate verification target
- Date: 2026-10-30
- Decision: a successful world launch on the user's device is useful evidence but does not prove the issue is resolved for all Android builds/devices.
- Status: a real embedded-world enter/re-enter integration test exists, but CI did not complete it for the B1–B4 source revision and Android-device validation remains outstanding.

## D-005 — Evidence labels are mandatory
- Date: 2026-10-30
- Decision: documentation must distinguish confirmed source facts, user-reported observations, hypotheses, proposed changes, applied changes, and tested results.
- Reason: prevent future chats from treating an idea or an unverified report as completed work.

## D-006 — Update repository memory at the end of every technical session
- Date: 2026-10-30
- Decision: update `docs/CONTINUITY.md` and append to `docs/DEVELOPMENT_LOG.md` after each meaningful investigation, code-generation, edit, or test session. Update the other docs when their content changes.
- Status: standing workflow rule.


## D-007 — Separate profile-record ID from persistent player identity
- Date: 2026-10-30
- Decision status: implemented in Phase 2.2-B1–B4; runtime use of `player_uuid` remains deferred to C/D.
- Preserve `LocalProfile.id` as the identity of the profile record.
- Investigate a separate persistent `player_uuid` for the player's identity in the embedded server.
- Reason: changing a profile's visible username must not silently change the identity used to locate persistent player data.
- Compatibility requirement: existing profiles must retain the historical player UUID if the proposed migration is confirmed safe.

## D-008 — Preserve historical identity for existing local profiles
- Date: 2026-10-30
- Decision status: implemented for valid legacy documents. New profiles currently receive random UUIDs, so compatibility with worlds created before C/D must be resolved before wiring the field into live gameplay.
- Candidate: initialize a missing `player_uuid` from the offline UUID derived from that profile's current stored username; do not regenerate `LocalProfile.id`.
- Reason: preserve existing SteelMC player-data keys rather than forcing a broad storage migration.
- Open questions: malformed/incomplete profile records, duplicate names/UUIDs, interrupted writes, and exact serialization compatibility.

## D-009 — Trusted identity handoff must be embedded-server-specific
- Date: 2026-10-30
- Decision: do not trust arbitrary UUID values from generic offline Hello packets and do not weaken generic offline login.
- Design requirement: investigate an internal trusted handoff from Pomme to its embedded SteelMC server, scoped so ordinary remote/offline clients cannot select another player's identity.
- Status: design requirement; implementation not authorized.

## D-010 — Phase 2.2-A is design-only until explicit approval
- Date: 2026-10-30
- Decision: complete source investigation and a written design report before implementation.
- No source edits, UUID migration, profile rename implementation, merge, PR, or APK generation is authorized by this documentation entry.


## D-011 — Authorize Phase 2.2-B1–B4 implementation
- Date: 2026-10-30
- Decision: the user explicitly authorized the programming chat to edit code to correct and integrate findings from the other Pomme investigation chats. This authorizes Phase 2.2-B1–B4 on `unofficial-android-menu-access`.
- Scope: profile JSON safety/recovery, independent `player_uuid`, compatible legacy migration, and integrity/idempotence tests.
- Exclusions: no SteelMC submodule update, no trusted connection identity (C), no Pomme-to-SteelMC identity plumbing (D), no Android acceptance phase (E), no merge, no master modification, and no APK publication.
- Status: implementation started; automated and Android verification pending.

## D-012 — Fail closed on invalid or future local-profile documents
- Date: 2026-10-30
- Decision: invalid existing profile data must never be silently replaced with a default profile. A valid backup may recover a missing or corrupt primary, while the corrupt original is retained under a unique quarantine filename.
- Future schema versions must not be silently downgraded from an older backup.
- Before updating the backup, validate the exact bytes from the current primary so corrupt content cannot replace the last known-good backup.
- Status: implemented in source; CI verification pending.


## D-013 — Preserve the current runtime UUID for all new profiles until trusted C/D integration
- Date: 2026-10-30
- Decision: newly created local profiles must initially persist the same offline UUID the running game currently uses for `OfflinePlayer:<username>`. Do not assign a random player UUID and later switch the live game to it.
- Compatibility: profile ID remains independent and random. When C/D eventually supplies `player_uuid` to SteelMC, it must preserve the exact current storage key for every profile created during B1–B4.
- Experimental compatibility: format-1 documents from the unmerged experiment stored random player UUIDs that the runtime did not consume. Migrate these to the historical offline UUID and write format 2.
- Status: implemented in source; CI verification pending.


## D-014 — Restore only the exact validated backup snapshot
- Date: 2026-10-30
- Decision: backup recovery must write the exact bytes that were decoded and validated, not reopen the backup path after validation.
- Reason: avoid restoring different contents if the backup file changes between validation and copying.
- Status: implemented in source commit `c0576191407a1ad53db77c0635c7b0820669e547`; CI verification pending.
