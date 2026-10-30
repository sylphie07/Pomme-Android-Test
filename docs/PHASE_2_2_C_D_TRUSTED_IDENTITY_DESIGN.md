# Phase 2.2-C/D — Trusted integrated-server player identity design

Last updated: 2026-10-30

## Status and authority

**Design only. No C/D implementation is authorized by this document.** Phase 2.2-B1–B4 validation is still being closed. Any future SteelMC source change, submodule pin update, or Pomme C/D implementation requires its own explicit user authorization. Phase 2.2-E remains separate.

This design deliberately does not change remote authentication, profile-selector UI, Android Keystore, or advancement persistence.

## Objective

Allow the integrated SteelMC server to use the selected local profile's persisted `LocalProfile.player_uuid` as the authoritative player-data key, while preserving the historical offline UUID already used by existing worlds. The identity must cross a trusted in-process server boundary; it must never be accepted from the Java login packet or other client-controlled offline input.

## Verified architecture at the inspected revisions

Repository: `sylphie07/Pomme-Android-Test`; branch `unofficial-android-menu-access`; inspected source HEAD `5a8de7d95f375ad93969b5098244d1a3c3bf5d5e`. SteelMC is pinned at `e655880a7b1487b2444c70013c14ee86c69d53b3`.

### Pomme path

- `pomme-client/src/app/phases/in_menu.rs::open_world` has access to `core.user`, starts the integrated server via `singleplayer::open(...)`, then creates a `Transport::Memory` connection.
- `connect_args` currently sets the memory transport UUID from `core.user.uuid`.
- `pomme-client/src/net/connection.rs` serializes a Java `ServerboundHello` containing a name and `profile_id`. This is a protocol packet and therefore cannot be the trust source.
- `pomme-singleplayer/src/lib.rs::LaunchOptions` currently carries world settings and the server transport, but no trusted player identity. Its embedded server runs with `online_mode: false` and `encryption: false`.
- The embedded server constructs `JavaTcpClient::from_transport` with a default `ServerConnectionSession`.

### SteelMC login and persistence path

- At the inspected pin, `steel-login/src/handlers/login.rs::handle_hello` explicitly treats the hello UUID as client supplied. In offline mode it derives `GameProfile` from `offline_uuid(requested_username)`; it does not trust the UUID in the hello packet.
- Online mode follows the existing encryption and Mojang session-server authentication path and produces an authenticated profile.
- The resulting `GameProfile` is carried through the pre-play state and used for player admission.
- `steel-core/src/player/player_data_storage/file_storage.rs` saves and loads per-domain player data and stats by the UUID from the server-side player profile. Therefore the UUID chosen by the login handler becomes a save-data key.

These observations explain why simply changing the UUID sent in Pomme's hello packet would not work: the offline login handler intentionally ignores it.

## Proposed trusted identity channel

Use a typed, server-side Rust value, conceptually:

```rust
struct TrustedLocalPlayer {
    username: String,
    player_uuid: Uuid,
}
```

The exact type name and API are implementation decisions for the separately authorized C/D work.

1. **Pomme source of truth:** take both fields from the selected persisted `core.user.local_profile`. Use `local_profile.player_uuid`, not a UUID parsed from launch arguments, the hello packet, or an arbitrary client-supplied field.
2. **Pass into the embedded server:** add the typed identity to `pomme_singleplayer::LaunchOptions`. The value travels over the in-process Rust API into the integrated server startup; it is not serialized onto the Java protocol.
3. **Per-connection Steel handoff:** pass the optional trusted identity into the embedded `JavaTcpClient` at construction (or an equivalently explicit per-connection server-side context). Prefer a per-connection field/argument over putting identity on a listener-wide session object, to avoid accidental sharing with other connections.
4. **External TCP default:** Steel's ordinary TCP constructor must always create a connection with no trusted-local identity. A remote client cannot request or populate this field.
5. **Offline embedded login:** only when the server is explicitly configured for the embedded offline path and the trusted identity is present, validate that the hello username matches the trusted local username, then construct the server `GameProfile` from the trusted UUID and trusted name. Ignore the hello packet's UUID entirely.
6. **Ordinary offline servers:** when no trusted local identity is supplied, preserve current vanilla-compatible `offline_uuid(requested_username)` behavior. Never start trusting the UUID field in a client hello.
7. **Online-mode guard:** online-mode login must continue through the existing encryption and Mojang authentication flow. A trusted offline identity must never bypass that flow; reject an invalid configuration that combines trusted-local identity with online mode rather than silently changing authentication policy.
8. **Pomme client side:** for `Transport::Memory`, pass the same persisted `local_profile.player_uuid` in `ConnectArgs` so client-side chat/session bookkeeping agrees with the server's profile. Remote `Transport::Remote` must continue to use only `multiplayer_identity()` from a verified, unexpired Microsoft Java session.

The trust boundary is the internal Rust API between Pomme and its embedded Steel server. The Java login packet remains untrusted, even when the transport happens to be an in-memory pipe.

## Compatibility invariants

- For every already-existing profile, the format-2 `player_uuid` must remain exactly equal to the historical runtime UUID that previously keyed that profile's worlds.
- The legacy and experimental format-1 migrations in B1–B4 must preserve profile IDs and selected-profile ID while migrating to the historical runtime UUID.
- C/D must not regenerate identities, derive a new UUID from the name after profile load, or rewrite existing world/player files.
- Server-side `GameProfile.id()` must equal the persisted `player_uuid` before player admission and data loading.
- Microsoft sign-in/sign-out and account switching must not overwrite the selected local profile identity.
- Remote authentication behavior and gameplay authorization gates must remain unchanged.

## Required regression tests before C/D can be considered safe

### Steel-side tests (require separately authorized SteelMC changes)

- Trusted embedded identity is used as the admitted `GameProfile` UUID.
- A deliberately different UUID in the client's hello packet is ignored.
- A hello username mismatch against the trusted local identity is rejected (or follows an explicitly documented, tested canonical-name rule).
- Ordinary offline TCP connections without the trusted context still use vanilla `offline_uuid(username)`, regardless of the hello packet UUID.
- Online-mode authentication still requires the existing Mojang verification path; trusted-local context cannot skip it.
- The normal TCP constructor cannot inherit an identity from an embedded connection or shared listener session.

### Pomme-side tests (require separately authorized C/D changes)

- `open_world` supplies the selected profile's persisted `player_uuid` to the embedded launch options.
- The memory `ConnectArgs.uuid` equals that same persisted UUID.
- Launch arguments containing a different UUID cannot override the selected profile's trusted embedded identity.
- Remote `ConnectArgs` still require a valid Microsoft session and use the Microsoft profile UUID/token.
- A migrated format-1 profile and a legacy profile reach the server with the same UUID that the old runtime used.
- Switching selected local profiles changes the trusted identity only for the next connection and does not mutate another profile's identity.

### End-to-end acceptance

- Open an existing world whose player data was created under the historical offline UUID; confirm the same inventory, position, stats, and other existing player state load under that exact UUID.
- Save, close cleanly, reopen, and confirm continuity.
- Do not use a destructive migration or ask the user to delete worlds/app data.
- This remains separate from Android-device acceptance in Phase 2.2-E.

## Explicitly separate workstreams

- **Profile selector UI:** list/create/select UX and persistence integration are not part of the trusted-channel design.
- **Android Keystore:** secure storage for Microsoft credentials is a separate security task; the local profile JSON must not contain access/refresh tokens.
- **Advancement persistence:** SteelMC's advancement-save limitation is separate from selecting the correct player UUID. Do not claim C/D fixes advancements.
- **Phase 2.2-E:** existing-world and real Android-device validation remains separately scoped.

## Implementation gate

Do not edit `third_party/SteelMC`, change its gitlink, change the Pomme connection code, or implement C/D from this proposal until the user explicitly authorizes that phase. First close B1–B4 CI validation and review its full source diff.


## Authorization and implementation status — 2026-10-30

The user explicitly authorized implementation of Phase 2.2-C and 2.2-D, including necessary SteelMC submodule changes, after B1–B4 CI completed successfully. This supersedes the earlier implementation-gate paragraph above insofar as it says explicit authorization is still pending.

The authorization does **not** include merging PR #11, modifying `master`, publishing/distributing an APK, or implementing unrelated workstreams. The programmer must perform the preflight and follow `docs/PHASE_2_2_CD_PROGRAMMER_HANDOFF.md`.

B1–B4 CI checkpoint: run `41012173438` (#94), associated with PR #11 HEAD `dedebc588065782768665d80b9fc3f6ca3281044`, completed successfully for tests, singleplayer, Android, iOS, fmt, Ubuntu/macOS/Windows lint, PR template, changes, and aggregate CI. Launcher jobs were skipped. This is automated CI evidence, not Android device acceptance.

The previously recorded SteelMC pin `e655880a7b1487b2444c70013c14ee86c69d53b3` is a historical checkpoint only; programmer must verify the live gitlink and pinned source before edits.