# Pomme Android — Technical Decisions and Constraints

## D-001 — Keep work on the authorized Android branch
- Date: 2026-10-30
- Decision: use `unofficial-android-menu-access` for the current Android menu/authentication investigation.
- Constraint: do not modify `master` or another branch without explicit user authorization.
- Reason: preserve separation between the Android investigation and the repository's other development lines.

## D-002 — Separate menu access from gameplay authentication
- Date: 2026-10-30
- Decision / target behavior: signing in should not be required merely to navigate the menu; authentication checks must remain on gameplay paths that require them.
- Status: target requirement, not yet a verified implementation.
- Reason: avoid blocking menu access while preserving necessary protections for actual play.

## D-003 — Audit every gameplay entry route
- Date: 2026-10-30
- Decision: do not patch only one visible button. Trace `MenuAction::Connect`, `MenuAction::PlayWorld`, `open_world()`, and quick-start/startup arguments, then centralize checks only after understanding the call graph.
- Status: investigation requirement.

## D-004 — Treat Android singleplayer as a separate verification target
- Date: 2026-10-30
- Decision: a successful world launch on the user's device is useful evidence but does not prove the issue is resolved for all Android builds/devices.
- Status: investigation requirement.

## D-005 — Evidence labels are mandatory
- Date: 2026-10-30
- Decision: documentation must distinguish confirmed source facts, user-reported observations, hypotheses, proposed changes, applied changes, and tested results.
- Reason: prevent future chats from treating an idea or an unverified report as completed work.

## D-006 — Update repository memory at the end of every technical session
- Date: 2026-10-30
- Decision: update `docs/CONTINUITY.md` and append to `docs/DEVELOPMENT_LOG.md` after each meaningful investigation, code-generation, edit, or test session. Update the other docs when their content changes.
- Status: standing workflow rule.
