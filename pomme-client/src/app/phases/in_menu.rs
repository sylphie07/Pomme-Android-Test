use crate::app::core::AppCore;
use crate::app::phases::{Gfx, Panorama};
use crate::app::{PendingGameAction, QuickPlay};
use crate::net::connection::{ConnectArgs, Transport};
use crate::singleplayer::{self, World};
use crate::ui::menu::{MenuAction, PanoramaTheme};

pub enum MenuUpdateResult {
    None,
    Connect {
        connect_args: Box<ConnectArgs>,
        world: Option<World>,
    },
    Quit,
}

#[derive(Clone, Copy)]
enum GameplayMode {
    Singleplayer,
    Multiplayer,
}

/// Identity and permission are separate: the local profile identifies the
/// integrated singleplayer player, while a verified Microsoft session is
/// required only for remote multiplayer.
fn authorize_gameplay(core: &mut AppCore, pending: PendingGameAction, mode: GameplayMode) -> bool {
    let allowed = match mode {
        GameplayMode::Singleplayer => core.user.allows_singleplayer(cfg!(not(debug_assertions))),
        GameplayMode::Multiplayer => core.user.allows_multiplayer(),
    };
    if allowed {
        return true;
    }

    if core.deferred_game_action.is_some() {
        tracing::warn!(
            "Rejected a gameplay request because another request is already waiting for account authorization"
        );
    } else {
        core.deferred_game_action = Some(pending);
    }
    core.menu.require_account(&core.input.key_bindings);
    core.menu.restore_account();
    core.menu.open_account(&core.input.key_bindings);
    false
}

/// Resumes exactly one gameplay request after a successful account action.
fn start_pending(core: &mut AppCore, pending: PendingGameAction) -> MenuUpdateResult {
    match pending {
        PendingGameAction::Connect {
            server,
            username,
            protocol,
        } => connect_remote(core, server, username, protocol),
        PendingGameAction::PlayWorld { folder } => open_world(core, &folder),
    }
}

/// Creates a remote connection only after the launch policy authorizes
/// gameplay.
fn connect_remote(
    core: &mut AppCore,
    server: String,
    username: String,
    protocol: Option<i32>,
) -> MenuUpdateResult {
    if !authorize_gameplay(
        core,
        PendingGameAction::Connect {
            server: server.clone(),
            username: username.clone(),
            protocol,
        },
        GameplayMode::Multiplayer,
    ) {
        return MenuUpdateResult::None;
    }
    core.audio.stop_menu_music();
    MenuUpdateResult::Connect {
        connect_args: Box::new(connect_args(
            core,
            Transport::Remote { server, protocol },
            username,
        )),
        world: None,
    }
}

/// Starts the integrated server for save folder `folder` and connects to it;
/// on failure the menu shows why.
pub fn open_world(core: &mut AppCore, folder: &str) -> MenuUpdateResult {
    if !authorize_gameplay(
        core,
        PendingGameAction::PlayWorld {
            folder: folder.to_owned(),
        },
        GameplayMode::Singleplayer,
    ) {
        return MenuUpdateResult::None;
    }
    // Vanilla `quickplay.error.invalid_identifier`.
    let opened = core
        .menu
        .world_to_launch(folder)
        .ok_or_else(|| "Could not find a world with the provided identifier".to_owned())
        .and_then(|(summary, dir)| {
            singleplayer::open(
                &summary,
                &dir,
                core.view_distance(),
                core.user.local_profile.username.clone(),
                core.user.local_profile.player_uuid,
            )
        });
    let (world, client_end) = match opened {
        Ok(opened) => opened,
        Err(reason) => {
            tracing::error!("Failed to open {folder}: {reason}");
            core.menu.show_disconnect(reason, &core.input.key_bindings);
            return MenuUpdateResult::None;
        }
    };
    core.menu.world_played(folder);
    core.audio.stop_menu_music();
    let username = core.user.local_profile.username.clone();
    MenuUpdateResult::Connect {
        connect_args: Box::new(connect_args(
            core,
            Transport::Memory {
                end: client_end,
                protocol: singleplayer::PROTOCOL,
            },
            username,
        )),
        world: Some(world),
    }
}

/// Starts `--quick-access-multiplayer` or `--quick-access-singleplayer`.
pub fn quick_play(core: &mut AppCore, play: QuickPlay) -> MenuUpdateResult {
    match play {
        QuickPlay::Multiplayer(server) => {
            let username = core
                .user
                .multiplayer_identity()
                .map(|(name, _, _)| name.to_owned())
                .unwrap_or_else(|| core.user.username.clone());
            connect_remote(core, server, username, None)
        }
        QuickPlay::Singleplayer(folder) => open_world(core, &folder),
    }
}

fn apply_account(core: &mut AppCore, gfx: &mut Gfx, account: Option<pomme_auth::AuthAccount>) {
    let account_uuid = account
        .as_ref()
        .and_then(|account| uuid::Uuid::parse_str(&account.uuid).ok());
    let account_name = account
        .as_ref()
        .map(|account| account.username.clone())
        .unwrap_or_else(|| core.user.username.clone());
    let access_token = account.as_ref().map(|account| account.access_token.clone());

    // This mutates only the Microsoft session. The selected local profile,
    // local username and offline UUID remain unchanged.
    core.user.set_microsoft_account(account);

    core.menu
        .set_account(account_name, access_token, &core.input.key_bindings);
    if let Some(uuid) = account_uuid {
        gfx.renderer.load_player_skin(&uuid, &core.tokio_rt);
        core.menu.set_player_face(Some(&uuid));
    } else {
        core.menu.set_player_face(None);
    }
}

pub fn connect_args(core: &AppCore, transport: Transport, username: String) -> ConnectArgs {
    let (username, uuid, access_token) = match &transport {
        Transport::Remote { .. } => {
            let (account_name, account_uuid, token) = core
                .user
                .multiplayer_identity()
                .expect("remote connection requires a valid Microsoft Java session");
            (
                account_name.to_owned(),
                account_uuid,
                Some(token.to_owned()),
            )
        }
        Transport::Memory { .. } => (username, core.user.local_profile.player_uuid, None),
    };
    ConnectArgs {
        transport,
        username,
        uuid,
        access_token,
        view_distance: core.view_distance(),
        chat_options: core.menu.chat_options,
        skin_options: core.menu.skin_options(),
    }
}

pub fn update_menu(
    core: &mut AppCore,
    dt: f32,
    gfx: &mut Gfx,
    panorama: &mut Panorama,
) -> MenuUpdateResult {
    if let Some(account) = core.menu.take_restored_account() {
        apply_account(core, gfx, Some(account));
        if let Some(pending) = core.deferred_game_action.take() {
            return start_pending(core, pending);
        }
    }

    panorama.update(dt);

    core.audio.start_menu_music();
    core.audio.update_menu_music(dt);

    let sw = gfx.renderer.screen_width() as f32;
    let sh = gfx.renderer.screen_height() as f32;

    // No chat in the menus; F2 results just log.
    for result in gfx.renderer.take_screenshot_messages() {
        match result {
            Ok(path) => tracing::info!("Saved screenshot as {path}"),
            Err(err) => tracing::warn!("Couldn't save screenshot: {err}"),
        }
    }

    let menu_input = core.build_menu_input(dt);

    let result = core
        .menu
        .build(sw, sh, &menu_input, &mut core.input.key_bindings, |t, s| {
            gfx.renderer.menu_text_width(t, s)
        });
    core.audio.set_volumes(core.menu.category_volumes());
    let action = result.action;

    let cursor_icon = if result.cursor_pointer {
        winit::window::CursorIcon::Pointer
    } else {
        winit::window::CursorIcon::Default
    };
    if core.input.cursor_moved_this_frame() {
        gfx.window.set_cursor(cursor_icon);
    }

    if let Some(images) = core.menu.menu_atlas_images() {
        gfx.renderer.update_favicon_atlas(&images);
    }

    // TODO: menu screens (a server MOTD, say) draw object glyphs as their
    // fallback sprite; dropping what they drew keeps those keys out of the
    // next session's atlas.
    gfx.renderer.drain_drawn_inline_objects();

    if let Err(e) = gfx.renderer.render_menu(
        &gfx.window,
        panorama.scroll(),
        result.elements,
        core.input.cursor_pos(),
        core.menu.skin_preview(),
    ) {
        tracing::error!("Render error: {e}");
    }

    core.input.clear_just_pressed_actions();

    core.sync_display_mode(&gfx.window);

    gfx.renderer.set_vsync(core.menu.vsync);
    gfx.renderer
        .set_menu_blur_radius(core.menu.menu_background_blurriness);

    if core.menu.rescan_packs {
        core.menu.rescan_packs = false;
        core.resource_packs.scan_local_packs();
        core.menu.available_packs = core.resource_packs.available_local_packs().to_vec();
        core.menu.active_packs = core.resource_packs.active_pack_info();
    }

    if let Some((name, enable)) = core.menu.pack_toggle.take() {
        if enable {
            core.resource_packs.enable_local_pack(&name);
        } else {
            core.resource_packs.disable_local_pack(&name);
        }
        core.menu.active_packs = core.resource_packs.active_pack_info();
        core.menu.available_packs = core.resource_packs.available_local_packs().to_vec();
    }

    if core.menu.reload_assets {
        core.reload_pack_assets(&mut gfx.renderer);
    }
    core.apply_font_options(&mut gfx.renderer);

    if result.clicked_button {
        gfx.renderer.trigger_skin_swing();
        core.audio.play_ui_click();
    }

    match action {
        MenuAction::Connect {
            server,
            username,
            protocol,
        } => return connect_remote(core, server, username, protocol),
        MenuAction::PlayWorld { folder } => return open_world(core, &folder),
        MenuAction::ChangeTheme(theme) => {
            gfx.renderer
                .reload_panorama(&theme.panorama_dir(&core.data_dirs));
            if theme == PanoramaTheme::Bedrock {
                gfx.renderer
                    .load_bedrock_sprites(&core.data_dirs.bedrock_ui_dir());
            }
            core.menu.start_transition_open();
        }
        MenuAction::Account(account) => {
            if account.is_none() {
                if let Some(account_uuid) = core.user.microsoft_profile_uuid() {
                    core.menu.forget_account(&account_uuid);
                }
                core.deferred_game_action = None;
            }
            let signed_in = account.is_some();
            apply_account(core, gfx, account);
            if signed_in && let Some(pending) = core.deferred_game_action.take() {
                return start_pending(core, pending);
            }
        }
        MenuAction::CancelPending => {
            core.deferred_game_action = None;
        }
        MenuAction::ImportSkin => {
            crate::util::request_skin_import();
        }
        MenuAction::Quit => {
            return MenuUpdateResult::Quit;
        }
        MenuAction::None => {}
    }

    MenuUpdateResult::None
}
