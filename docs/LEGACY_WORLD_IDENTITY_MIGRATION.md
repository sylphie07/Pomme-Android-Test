# Legacy world identity recovery — investigation and programmer handoff

Date: 2026-10-30

## Status and scope

**Investigation only. No Rust source, SteelMC source, submodule gitlink, worlds, player files, profile files, build configuration, APK, or master branch was changed.** This document is a technical handoff, not authorization to implement a new migration workflow.

Repository inspected: `sylphie07/Pomme-Android-Test`
- PR #11: open, unmerged, base `master`.
- Inspected Pomme HEAD: `e902fa1cb733edabd1349877dd49b3d8a8a74d42`.
- Inspected live `third_party/SteelMC` gitlink: `c5098cdd8a4ec1176e00d737df24a2861b4055ac`.
- SteelMC historical base pin: `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- Investigation branch: `audit/legacy-world-identity-migration`, based on the inspected Pomme HEAD.

GitHub metadata cannot reveal the programmer's local working-tree changes. This investigation did not touch the programmer's branch or SteelMC branch.

## 1. Problem confirmed in Pomme

In `pomme-client/src/user.rs`:

- `LocalProfileStore::load_or_create()` loads an existing `local-profiles.json` if present.
- If that file and its backup are both absent, it unconditionally creates and persists a profile named `Player`.
- It does not inspect existing worlds, the previous launch username, or saved SteelMC player records before publishing that default.

In `pomme-client/src/lib.rs`, this store is loaded during startup before the menu opens. The selected profile is then used for the integrated singleplayer identity.

In the current `pomme-client/src/app/phases/in_menu.rs::open_world`, Pomme passes `core.user.local_profile.username` and `core.user.local_profile.player_uuid` to `singleplayer::open`. The current trusted connection therefore uses the selected local profile, not the old Microsoft account name.

**Impact:** on an upgraded installation with old worlds but no `local-profiles.json`, the first startup can persist `Player` before the app has discovered the old player identity. The old player files are not deleted, but the next login can look under the new `offline_uuid("Player")` key and appear to be a fresh player. Saving then creates a second identity's data while leaving the old files behind.

This is a source-level compatibility risk. No device reproduction was performed, and no claim is made that a user's world has actually lost data.

## 2. What the old runtime used

At the old Pomme source, `pomme-client/src/user.rs::UserData::from_account` built the runtime user from the stored Microsoft account name. However, SteelMC's offline login handler did not trust the client Hello UUID: it derived the server-side profile from `offline_uuid(requested_username)`.

The old SteelMC login implementation at the historical pin `e655880a7b1487b2444c70013c14ee86c69d53b3` confirms that behavior. Player data storage uses the server-side `GameProfile.id()` as the key.

Therefore, for the old embedded offline path, the important legacy key is generally:

`offline_uuid(the username that actually logged into that world)`

It is **not** necessarily the Microsoft account UUID. The account username is a possible clue, not authoritative proof: users may have renamed accounts, used multiple accounts, or launched with explicit `--username` arguments.

## 3. Strong recovery evidence already present in old worlds

A useful discovery was confirmed in both the historical SteelMC pin and the current SteelMC fork:

- `global/known_players.dat` is a persisted cache of `(UUID, last_known_name, expires_at)` records.
- The cache is updated after a player successfully spawns/admission completes (`Server::record_known_player`, called from `steel-core/src/server/player_admission.rs`).
- The cache is flushed during orderly server shutdown (`steel-core/src/server/run_loop.rs::save_and_shutdown`).
- The cache is bounded to 1,000 entries and uses a one-month expiry policy.
- Steel's file backend stores global player records under `global/players/data/<uuid>`, domain player data under `<domain>/players/data/<uuid>`, and domain statistics under `<domain>/players/stats/<uuid>.toml`.

This is a much better recovery source than guessing from the current Microsoft account. For an offline legacy record, the pair is internally consistent when:

`offline_uuid(last_known_name) == cached_uuid`

The candidate should also be corroborated by an existing player-data file under that UUID in the same world. A name/UUID cache entry by itself is not enough.

### Limitations

- The cache may be absent if the player never completed admission or the server did not shut down cleanly.
- It is bounded and not a permanent audit log.
- Expired records may still be present on disk, but expiry means the cache is not guaranteed to retain them indefinitely.
- A world may contain multiple historical identities if it was played with multiple usernames/accounts.
- A corrupt or unsupported cache must be treated as unavailable evidence, not as proof that the world has no player data.

The parser/decoder for this binary file is SteelMC-internal. Pomme should not duplicate the binary format or hard-code its serialization. Prefer a small read-only Steel API that returns validated known-player entries (or a narrowly scoped world-inspection result).

## 4. Recommended recovery strategy

### Stage A — read-only world inspection before creating the default profile

When `local-profiles.json` is absent:

1. Enumerate existing save folders under the configured saves directory.
2. For each world, read its known-player index through a Steel-owned API.
3. For each entry, validate the offline UUID/name relationship.
4. Corroborate the UUID against existing player-data files in that world. The primary acceptance signal should be a domain player-data file; stats/global records can be additional evidence.
5. Collect candidates per world without writing to the world or player-data directories.

Do not implement this by launching the world with a guessed identity. Discovery must be read-only and must happen before the first trusted login.

### Stage B — deterministic outcomes

- **No worlds and no old player-data records:** create the normal default `Player` profile.
- **Exactly one verified historical identity across all existing worlds:** initialize the local profile with that exact username and UUID, preserving the old world key.
- **Every world has one candidate, but candidates differ between worlds:** do not force one global profile onto all worlds. Either add an explicit per-world identity association or ask the user to select the historical profile for the world being opened.
- **Multiple plausible candidates for a world:** show a recovery choice; never silently select the most recent Microsoft account.
- **Player-data files exist but the known-player index is missing/corrupt/unhelpful:** offer manual recovery by asking for the previous Minecraft username. Compute its offline UUID and preview which worlds contain matching data files before committing the profile.
- **No candidate matches:** preserve everything and allow an explicit fresh identity only after warning that old player data may remain under an unknown UUID.

### Stage C — use account and launcher data only as corroborating candidates

The old `accounts.json` entries and launch arguments may supply candidate usernames. They must not be used as sole proof. Accept a candidate only when its computed offline UUID matches existing player-data files or a validated known-player record in the target world.

Do not use the Microsoft profile UUID as a replacement for the old offline UUID.

### Stage D — already-created default profile

A profile file containing a single `Player` record may already have been generated by an earlier build of B1–B4. It is unsafe to overwrite it automatically: the user may have intentionally created/used that identity, and the file does not currently record whether it was auto-generated.

For such files:
- preserve the current profile and its UUID;
- offer a separate, explicit legacy-world recovery action when old-world evidence is found;
- add the recovered identity as another profile only after user confirmation;
- if worlds require different identities, provide a world-level choice/mapping rather than silently changing the global selection.

For future schema versions, consider recording profile origin/migration state (for example, an unconfirmed generated default) so a later update can distinguish a generated placeholder from a user-confirmed profile. Do not infer that state for old format-2 files without user confirmation.

## 5. Adjacent UUID consistency issue

At inspected Pomme HEAD `e902fa1...`, `UserData::from_local_profile()` and `UserData::set_local_profile()` still recompute `offline_uuid(username)` instead of copying `LocalProfile.player_uuid`.

The current `open_world()` and memory `connect_args()` paths explicitly use `local_profile.player_uuid`, so the trusted embedded identity path has the stored value. However, the duplicated `UserData.uuid` field can diverge if a profile's persisted UUID intentionally differs from its visible username (the central future use case for persistent identity).

Recommended correction for the implementation owner: make local-profile-derived runtime identity use the exact stored `profile.player_uuid`, and add a regression test with a profile whose UUID deliberately does not equal `offline_uuid(username)`. Keep Microsoft multiplayer identity separate.

This is an adjacent consistency item, not evidence that current format-2 profiles (which currently initialize the UUID from the name) are already mismatched.

## 6. Implementation ownership and suggested file scope

Investigation chat has not changed code. The dedicated programmer should first acknowledge the handoff and confirm that it does not conflict with active C/D edits.

Likely Pomme areas:
- `pomme-client/src/user.rs`: separate first-run creation from legacy migration; represent a pending/ambiguous recovery state without overwriting data.
- `pomme-client/src/lib.rs`: perform migration/discovery before persisting the placeholder profile.
- `pomme-client/src/app/phases/in_menu.rs` and menu UI: present explicit recovery choices only where ambiguity exists.
- `pomme-client/src/singleplayer.rs` / `pomme-singleplayer`: expose a safe read-only discovery interface if Steel owns the decoder.

Likely SteelMC area:
- `steel-core/src/player/player_data_storage`: expose a narrow, validated, read-only known-player index query without duplicating the binary format in Pomme.
- Preserve normal storage behavior and do not make the world server rewrite player records during discovery.

No changes should be made to the programmer's active branch until they have reviewed this handoff. Keep the work on an isolated branch/PR if it needs to proceed in parallel.

## 7. Required tests and acceptance criteria

1. Legacy fixture created with the pre-C/D offline login path under username `OldName`; the known-player entry and player-data file both resolve to `offline_uuid("OldName")`.
2. Missing local-profile file plus one verified identity across worlds migrates to that exact UUID, not `offline_uuid("Player")`.
3. Fresh install with no worlds creates the normal default profile.
4. Multiple old identities in one world trigger explicit recovery; no automatic selection.
5. Different identities in different worlds do not get collapsed into one global profile.
6. Missing, corrupt, expired, and unsupported known-player caches are handled without modifying worlds or assuming no player data exists.
7. Account username/launcher argument candidates are accepted only when corroborated by actual player-data files.
8. An already-existing `Player` profile is never overwritten automatically; recovery adds or selects identities only after explicit confirmation.
9. Re-running migration is idempotent; interrupted profile publication preserves the last known-good profile file.
10. Opening a migrated world loads the exact old player-data UUID and retains inventory, position, health, experience, ender chest, stats, and global/domain state after save-close-reopen.
11. Discovery performs no writes to world/player-data files.
12. `UserData::from_local_profile()` and `set_local_profile()` preserve a deliberately non-derived stored UUID.
13. Remote multiplayer continues to use verified Microsoft identity only; generic offline TCP continues to ignore client Hello UUIDs.

## 8. What was not done

- No source or submodule edits.
- No CI, Rust tests, local builds, APK builds, or Android device tests.
- No world was opened, modified, copied, or migrated.
- No user account data was read from a live device.
- No claim is made that every old world has a recoverable known-player index.

## 9. Next recommended order

1. Dedicated programmer reviews this investigation against their latest local C/D work and acknowledges conflicts.
2. Implement the read-only Steel discovery API and fixture tests (or another Steel-owned way to decode the known-player index).
3. Implement Pomme first-run migration and explicit ambiguous recovery.
4. Add the exact stored-UUID consistency fix.
5. Run full CI on the final source revision.
6. Perform non-destructive Android acceptance on a copy/backup of a representative old world, then test the original only after the copy proves continuity.

