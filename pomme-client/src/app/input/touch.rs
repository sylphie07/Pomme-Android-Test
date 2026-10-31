//! Touch controls for gameplay, Bedrock-style: a move stick on the left,
//! dragging elsewhere to look, a tap to use, a still hold to destroy, and
//! buttons for jump, inventory and pause. Outside gameplay a touch is the
//! mouse pointer, so every screen works unchanged.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use glam::Vec2;
use serde::{Deserialize, Serialize};
use winit::event::TouchPhase;
use winit::keyboard::KeyCode;

use super::InputKey;
use super::action::Action;

/// A press lifted within this, having stayed still, is a use.
const TAP_TIME: Duration = Duration::from_millis(250);
/// A press held still this long starts destroying.
const HOLD_TIME: Duration = Duration::from_millis(300);
/// How far a press may drift, in GUI units, and still count as still.
const STILL_SLOP: f32 = 4.0;
/// The stick's travel, in GUI units.
pub const STICK_RADIUS: f32 = 30.0;
/// Mouse counts per GUI unit dragged.
const LOOK_SCALE: f32 = 2.5;
/// The share of the screen width, from the left, where a press is the stick.
const STICK_ZONE: f32 = 0.4;
pub const BUTTON_RADIUS: f32 = 14.0;
/// Vertical movement needed before a menu touch becomes a scroll gesture.
const MENU_SCROLL_SLOP: f32 = 8.0;
/// Convert finger pixels to the menu's wheel-like scroll units.
const MENU_SCROLL_PIXELS_PER_UNIT: f32 = 20.0;

/// Where the controls sit on a `width` x `height` screen at GUI scale `gs`.
#[derive(Clone, Copy)]
pub struct TouchContext {
    pub gameplay: bool,
    pub scrollable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TouchHudLayout {
    #[serde(default = "default_touch_positions")]
    pub positions: [[f32; 2]; 4],
    #[serde(default = "default_touch_scales")]
    pub scales: [f32; 4],
    #[serde(default = "default_touch_visibility")]
    pub visible: [bool; 4],
    /// User-created buttons that synthesize keyboard input through KeyBindings.
    #[serde(default)]
    pub custom_buttons: Vec<CustomTouchButton>,
}

/// A configurable touch button that simulates one supported keyboard key.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomTouchButton {
    pub input_id: String,
    pub position: [f32; 2],
    pub scale: f32,
    #[serde(default = "default_custom_button_visible")]
    pub visible: bool,
}

fn default_custom_button_visible() -> bool {
    true
}

pub const MAX_CUSTOM_TOUCH_BUTTONS: usize = 12;

fn default_touch_positions() -> [[f32; 2]; 4] {
    [[0.96, 0.92], [0.78, 0.07], [0.88, 0.07], [0.96, 0.07]]
}
fn default_touch_scales() -> [f32; 4] {
    [1.0; 4]
}
fn default_touch_visibility() -> [bool; 4] {
    [true; 4]
}
impl Default for TouchHudLayout {
    fn default() -> Self {
        Self {
            positions: default_touch_positions(),
            scales: default_touch_scales(),
            visible: default_touch_visibility(),
            custom_buttons: Vec::new(),
        }
    }
}
impl TouchHudLayout {
    pub fn sanitized(mut self) -> Self {
        for position in &mut self.positions {
            for coordinate in position {
                *coordinate = if coordinate.is_finite() {
                    coordinate.clamp(0.04, 0.96)
                } else {
                    0.5
                };
            }
        }
        for scale in &mut self.scales {
            *scale = if scale.is_finite() {
                scale.clamp(0.6, 1.8)
            } else {
                1.0
            };
        }
        self.custom_buttons.truncate(MAX_CUSTOM_TOUCH_BUTTONS);
        self.custom_buttons.retain_mut(|button| {
            if button.input_id.len() > 128
                || !matches!(
                    InputKey::from_input_id(&button.input_id),
                    Ok(InputKey::Keyboard(_))
                ) {
                return false;
            }
            for coordinate in &mut button.position {
                *coordinate = if coordinate.is_finite() {
                    coordinate.clamp(0.04, 0.96)
                } else {
                    0.5
                };
            }
            button.scale = if button.scale.is_finite() {
                button.scale.clamp(0.6, 1.8)
            } else {
                1.0
            };
            true
        });
        self
    }
    fn index(button: TouchButton) -> usize {
        match button {
            TouchButton::Jump => 0,
            TouchButton::Chat => 1,
            TouchButton::Inventory => 2,
            TouchButton::Pause => 3,
        }
    }
    pub fn position(&self, button: TouchButton) -> Vec2 {
        let [x, y] = self.positions[Self::index(button)];
        Vec2::new(x, y)
    }
    pub fn scale(&self, button: TouchButton) -> f32 {
        self.scales[Self::index(button)]
    }
    pub fn is_visible(&self, button: TouchButton) -> bool {
        self.visible[Self::index(button)]
    }
}
#[derive(Clone)]
pub struct TouchLayout {
    pub width: f32,
    pub height: f32,
    pub gs: f32,
    pub hud: TouchHudLayout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TouchButton {
    Jump,
    Chat,
    Inventory,
    Pause,
}

impl TouchButton {
    pub const ALL: [Self; 4] = [Self::Jump, Self::Chat, Self::Inventory, Self::Pause];

    pub fn label(self) -> &'static str {
        match self {
            Self::Jump => "Jump",
            Self::Chat => "Chat",
            Self::Inventory => "Inv",
            Self::Pause => "| |",
        }
    }

    /// The action a tap presses; Jump is held instead.
    fn tapped_action(self) -> Option<Action> {
        match self {
            Self::Jump => None,
            Self::Chat => Some(Action::OpenChat),
            Self::Inventory => Some(Action::ToggleInventory),
            Self::Pause => Some(Action::OpenMenu),
        }
    }
}

impl TouchLayout {
    pub fn button_centre(&self, button: TouchButton) -> Vec2 {
        let p = self.hud.position(button);
        Vec2::new(p.x * self.width, p.y * self.height)
    }
    pub fn button_radius(&self, button: TouchButton) -> f32 {
        BUTTON_RADIUS * self.gs * self.hud.scale(button)
    }

    /// Where the stick rests until a press places it.
    pub fn stick_home(&self) -> Vec2 {
        let inset = 1.6 * STICK_RADIUS * self.gs;
        Vec2::new(inset, self.height - inset)
    }

    fn button_at(&self, p: Vec2) -> Option<TouchButton> {
        TouchButton::ALL.into_iter().find(|&b| {
            self.hud.is_visible(b) && p.distance(self.button_centre(b)) <= self.button_radius(b)
        })
    }

    pub fn custom_button_centre(&self, index: usize) -> Vec2 {
        let [x, y] = self.hud.custom_buttons[index].position;
        Vec2::new(x * self.width, y * self.height)
    }

    pub fn custom_button_radius(&self, index: usize) -> f32 {
        BUTTON_RADIUS * self.gs * self.hud.custom_buttons[index].scale
    }

    fn custom_button_at(&self, p: Vec2) -> Option<KeyCode> {
        self.hud
            .custom_buttons
            .iter()
            .enumerate()
            .find_map(|(index, button)| {
                if !button.visible
                    || p.distance(self.custom_button_centre(index))
                        > self.custom_button_radius(index)
                {
                    return None;
                }
                match InputKey::from_input_id(&button.input_id) {
                    Ok(InputKey::Keyboard(key)) => Some(key),
                    _ => None,
                }
            })
    }

    fn hotbar_slot_at(&self, p: Vec2) -> Option<u8> {
        crate::ui::hud::hotbar_slot_at(self.width, self.height, self.gs, (p.x, p.y))
    }
}

/// What a touch did, for `InputState` to apply as mouse and key input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TouchOutput {
    /// Mouse counts to turn by.
    Look(Vec2),
    DestroyStart,
    DestroyEnd,
    /// A destroy that outlived gameplay: stop it without a release.
    DestroyCancel,
    Use,
    /// A tap of an action's key.
    Tap(Action),
    /// A custom HUD button's simulated keyboard press/release.
    VirtualKeyPress(KeyCode),
    VirtualKeyRelease(KeyCode),
    Hotbar(u8),
    PointerMove(Vec2),
    /// Vertical menu scroll in wheel-like units.
    Scroll(f32),
    PointerDown,
    PointerUp,
}

enum Role {
    Stick {
        origin: Vec2,
        current: Vec2,
        /// [`STICK_RADIUS`] in pixels.
        radius: f32,
    },
    Look {
        start: Vec2,
        last: Vec2,
        since: Instant,
        still: bool,
        destroying: bool,
    },
    Jump,
    CustomKey {
        key: KeyCode,
    },
    Pointer {
        start: Vec2,
        last: Vec2,
        pressed: bool,
        scrolling: bool,
        scrollable: bool,
    },
    /// A tap already handled on press; ignored until it lifts.
    Spent,
}

#[derive(Default)]
pub struct TouchControls {
    touches: HashMap<u64, Role>,
    /// Any touch has arrived, so the overlay shows; only phones feed them.
    seen: bool,
}

impl TouchControls {
    pub fn seen(&self) -> bool {
        self.seen
    }

    /// Feeds one touch event. `gameplay` is whether the world has the input
    /// (no screen, chat or pause menu open); a touch keeps the role it
    /// started with until it lifts.
    pub fn on_touch(
        &mut self,
        id: u64,
        phase: TouchPhase,
        p: Vec2,
        context: TouchContext,
        layout: &TouchLayout,
        now: Instant,
    ) -> Vec<TouchOutput> {
        self.seen = true;
        match phase {
            TouchPhase::Started => self.start(id, p, context, layout, now),
            TouchPhase::Moved => self.drag(id, p, layout),
            TouchPhase::Ended => self.end(id, now, true, context.gameplay),
            TouchPhase::Cancelled => self.end(id, now, false, context.gameplay),
        }
    }

    fn start(
        &mut self,
        id: u64,
        p: Vec2,
        context: TouchContext,
        layout: &TouchLayout,
        now: Instant,
    ) -> Vec<TouchOutput> {
        let (role, out) = if !context.gameplay {
            (
                Role::Pointer {
                    start: p,
                    last: p,
                    pressed: false,
                    scrolling: false,
                    scrollable: context.scrollable,
                },
                vec![TouchOutput::PointerMove(p)],
            )
        } else if let Some(button) = layout.button_at(p) {
            match button.tapped_action() {
                Some(action) => (Role::Spent, vec![TouchOutput::Tap(action)]),
                None => (Role::Jump, Vec::new()),
            }
        } else if let Some(key) = layout.custom_button_at(p) {
            let already_held = self
                .touches
                .values()
                .any(|role| matches!(role, Role::CustomKey { key: held } if *held == key));
            (
                Role::CustomKey { key },
                if already_held {
                    Vec::new()
                } else {
                    vec![TouchOutput::VirtualKeyPress(key)]
                },
            )
        } else if let Some(slot) = layout.hotbar_slot_at(p) {
            (Role::Spent, vec![TouchOutput::Hotbar(slot)])
        } else if p.x < layout.width * STICK_ZONE {
            (
                Role::Stick {
                    origin: p,
                    current: p,
                    radius: STICK_RADIUS * layout.gs,
                },
                Vec::new(),
            )
        } else {
            (
                Role::Look {
                    start: p,
                    last: p,
                    since: now,
                    still: true,
                    destroying: false,
                },
                Vec::new(),
            )
        };
        self.touches.insert(id, role);
        out
    }

    fn drag(&mut self, id: u64, p: Vec2, layout: &TouchLayout) -> Vec<TouchOutput> {
        match self.touches.get_mut(&id) {
            Some(Role::Stick { current, .. }) => {
                *current = p;
                Vec::new()
            }
            Some(Role::Look {
                start, last, still, ..
            }) => {
                let delta = (p - *last) / layout.gs * LOOK_SCALE;
                *last = p;
                if p.distance(*start) > STILL_SLOP * layout.gs {
                    *still = false;
                }
                vec![TouchOutput::Look(delta)]
            }
            Some(Role::Pointer {
                start,
                last,
                pressed,
                scrolling,
                scrollable,
            }) => {
                let delta = p - *last;
                let total = p - *start;
                *last = p;
                if *scrolling {
                    vec![
                        TouchOutput::PointerMove(p),
                        TouchOutput::Scroll(delta.y / (MENU_SCROLL_PIXELS_PER_UNIT * layout.gs)),
                    ]
                } else if *pressed {
                    // A recognized drag owns the pointer until touch release.
                    vec![TouchOutput::PointerMove(p)]
                } else if *scrollable
                    && total.y.abs() > MENU_SCROLL_SLOP * layout.gs
                    && total.y.abs() > total.x.abs() * 1.25
                {
                    *scrolling = true;
                    vec![
                        TouchOutput::PointerMove(p),
                        TouchOutput::Scroll(total.y / (MENU_SCROLL_PIXELS_PER_UNIT * layout.gs)),
                    ]
                } else if total.length() > MENU_SCROLL_SLOP * layout.gs
                    && (!*scrollable
                        || total.x.abs() > MENU_SCROLL_SLOP * layout.gs
                            && total.x.abs() > total.y.abs() * 1.25)
                {
                    // Inventory/crafting screens retain vertical item drags.
                    *pressed = true;
                    vec![TouchOutput::PointerDown, TouchOutput::PointerMove(p)]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

    /// A look touch lifted after a screen opened stays out of that screen.
    fn end(&mut self, id: u64, now: Instant, lifted: bool, gameplay: bool) -> Vec<TouchOutput> {
        match self.touches.remove(&id) {
            Some(Role::CustomKey { key })
                if self
                    .touches
                    .values()
                    .any(|role| matches!(role, Role::CustomKey { key: held } if *held == key)) =>
            {
                Vec::new()
            }
            Some(Role::CustomKey { key }) => vec![TouchOutput::VirtualKeyRelease(key)],
            Some(Role::Look {
                destroying: true, ..
            }) => vec![if gameplay {
                TouchOutput::DestroyEnd
            } else {
                TouchOutput::DestroyCancel
            }],
            Some(Role::Look { since, still, .. })
                if gameplay && lifted && still && now.duration_since(since) < TAP_TIME =>
            {
                vec![TouchOutput::Use]
            }
            Some(Role::Pointer {
                pressed: true,
                scrolling: false,
                ..
            }) => vec![TouchOutput::PointerUp],
            Some(Role::Pointer {
                pressed: false,
                scrolling: false,
                last,
                ..
            }) if lifted => vec![
                TouchOutput::PointerMove(last),
                TouchOutput::PointerDown,
                TouchOutput::PointerUp,
            ],
            Some(Role::Pointer { .. }) => Vec::new(),
            _ => Vec::new(),
        }
    }

    /// Once a frame: a look press held still long enough starts destroying,
    /// while the world has the input.
    pub fn tick(&mut self, now: Instant, gameplay: bool) -> Vec<TouchOutput> {
        let mut out = Vec::new();
        if !gameplay {
            let mut held_keys = HashSet::new();
            for role in self.touches.values_mut() {
                if let Role::CustomKey { key } = role {
                    held_keys.insert(*key);
                    *role = Role::Spent;
                }
            }
            out.extend(held_keys.into_iter().map(TouchOutput::VirtualKeyRelease));
            return out;
        }
        for role in self.touches.values_mut() {
            if let Role::Look {
                since,
                still: true,
                destroying,
                ..
            } = role
                && !*destroying
                && now.duration_since(*since) >= HOLD_TIME
            {
                *destroying = true;
                out.push(TouchOutput::DestroyStart);
            }
        }
        out
    }

    /// The stick's deflection, unit length at most, `+y` up like a gamepad
    /// stick.
    pub fn stick(&self) -> Option<Vec2> {
        self.touches.values().find_map(|role| match role {
            Role::Stick {
                origin,
                current,
                radius,
            } => {
                let v = (*current - *origin) / *radius;
                Some(Vec2::new(v.x, -v.y).clamp_length_max(1.0))
            }
            _ => None,
        })
    }

    /// Where a held stick was placed, for drawing.
    pub fn stick_origin(&self) -> Option<Vec2> {
        self.touches.values().find_map(|role| match role {
            Role::Stick { origin, .. } => Some(*origin),
            _ => None,
        })
    }

    pub fn jump_held(&self) -> bool {
        self.touches.values().any(|role| matches!(role, Role::Jump))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use glam::Vec2;
    use winit::event::TouchPhase;
    use winit::keyboard::KeyCode;

    use super::{
        Action, CustomTouchButton, TouchButton, TouchContext, TouchControls, TouchHudLayout,
        TouchLayout, TouchOutput,
    };

    const MENU: TouchContext = TouchContext {
        gameplay: false,
        scrollable: true,
    };
    const INVENTORY: TouchContext = TouchContext {
        gameplay: false,
        scrollable: false,
    };
    const GAMEPLAY: TouchContext = TouchContext {
        gameplay: true,
        scrollable: false,
    };

    static LAYOUT: std::sync::LazyLock<TouchLayout> = std::sync::LazyLock::new(|| TouchLayout {
        width: 1600.0,
        height: 900.0,
        gs: 2.0,
        hud: TouchHudLayout {
            positions: [[0.96, 0.92], [0.78, 0.07], [0.88, 0.07], [0.96, 0.07]],
            scales: [1.0; 4],
            visible: [true; 4],
            custom_buttons: Vec::new(),
        },
    });
    /// Right of the stick zone, above the hotbar, away from the buttons.
    const LOOK: Vec2 = Vec2::new(1000.0, 400.0);
    const STICK: Vec2 = Vec2::new(200.0, 600.0);

    fn ms(start: Instant, ms: u64) -> Instant {
        start + Duration::from_millis(ms)
    }

    fn touch(
        c: &mut TouchControls,
        id: u64,
        phase: TouchPhase,
        p: Vec2,
        at: Instant,
    ) -> Vec<TouchOutput> {
        c.on_touch(id, phase, p, GAMEPLAY, &LAYOUT, at)
    }

    #[test]
    fn a_quick_still_tap_uses() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        assert!(touch(&mut c, 1, TouchPhase::Started, LOOK, t).is_empty());
        assert!(c.tick(ms(t, 100), true).is_empty());
        assert_eq!(
            touch(&mut c, 1, TouchPhase::Ended, LOOK, ms(t, 150)),
            [TouchOutput::Use]
        );
        assert!(c.seen());
    }

    #[test]
    fn a_look_touch_stays_out_of_a_screen_opened_under_it() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        touch(&mut c, 1, TouchPhase::Started, LOOK, t);
        assert!(c.tick(ms(t, 300), false).is_empty());
        assert!(
            c.on_touch(1, TouchPhase::Ended, LOOK, INVENTORY, &LAYOUT, ms(t, 350))
                .is_empty()
        );

        touch(&mut c, 2, TouchPhase::Started, LOOK, t);
        assert_eq!(c.tick(ms(t, 300), true), [TouchOutput::DestroyStart]);
        assert_eq!(
            c.on_touch(2, TouchPhase::Ended, LOOK, INVENTORY, &LAYOUT, ms(t, 400)),
            [TouchOutput::DestroyCancel]
        );
    }

    #[test]
    fn a_still_hold_destroys_until_lifted() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        touch(&mut c, 1, TouchPhase::Started, LOOK, t);
        assert_eq!(c.tick(ms(t, 300), true), [TouchOutput::DestroyStart]);
        assert!(c.tick(ms(t, 400), true).is_empty());
        assert_eq!(
            touch(&mut c, 1, TouchPhase::Ended, LOOK, ms(t, 500)),
            [TouchOutput::DestroyEnd]
        );
    }

    #[test]
    fn a_drag_looks_and_neither_uses_nor_destroys() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        touch(&mut c, 1, TouchPhase::Started, LOOK, t);
        let moved = LOOK + Vec2::new(40.0, -20.0);
        assert_eq!(
            touch(&mut c, 1, TouchPhase::Moved, moved, ms(t, 50)),
            [TouchOutput::Look(Vec2::new(50.0, -25.0))]
        );
        assert!(c.tick(ms(t, 400), true).is_empty());
        assert!(touch(&mut c, 1, TouchPhase::Ended, moved, ms(t, 100)).is_empty());
    }

    #[test]
    fn the_stick_clamps_to_unit_length_with_y_up() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        touch(&mut c, 2, TouchPhase::Started, STICK, t);
        touch(
            &mut c,
            2,
            TouchPhase::Moved,
            STICK + Vec2::new(0.0, -30.0),
            t,
        );
        assert_eq!(c.stick(), Some(Vec2::new(0.0, 0.5)));
        touch(
            &mut c,
            2,
            TouchPhase::Moved,
            STICK + Vec2::new(500.0, 0.0),
            t,
        );
        assert_eq!(c.stick(), Some(Vec2::new(1.0, 0.0)));
        touch(&mut c, 2, TouchPhase::Ended, STICK, t);
        assert_eq!(c.stick(), None);
    }

    #[test]
    fn move_and_look_at_once() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        touch(&mut c, 1, TouchPhase::Started, STICK, t);
        touch(&mut c, 2, TouchPhase::Started, LOOK, t);
        touch(
            &mut c,
            1,
            TouchPhase::Moved,
            STICK + Vec2::new(60.0, 0.0),
            t,
        );
        let look = touch(&mut c, 2, TouchPhase::Moved, LOOK + Vec2::new(8.0, 0.0), t);
        assert_eq!(look, [TouchOutput::Look(Vec2::new(10.0, 0.0))]);
        assert_eq!(c.stick(), Some(Vec2::new(1.0, 0.0)));
    }

    #[test]
    fn buttons_and_hotbar_fire_on_press() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        let pause = LAYOUT.button_centre(TouchButton::Pause);
        assert_eq!(
            touch(&mut c, 1, TouchPhase::Started, pause, t),
            [TouchOutput::Tap(Action::OpenMenu)]
        );
        assert!(touch(&mut c, 1, TouchPhase::Ended, pause, t).is_empty());
        assert_eq!(
            touch(
                &mut c,
                4,
                TouchPhase::Started,
                LAYOUT.button_centre(TouchButton::Chat),
                t
            ),
            [TouchOutput::Tap(Action::OpenChat)]
        );

        touch(
            &mut c,
            2,
            TouchPhase::Started,
            LAYOUT.button_centre(TouchButton::Jump),
            t,
        );
        assert!(c.jump_held());
        touch(&mut c, 2, TouchPhase::Ended, Vec2::ZERO, t);
        assert!(!c.jump_held());

        // The hotbar's first slot starts 1 unit in from its left edge.
        let first_slot = Vec2::new(LAYOUT.width / 2.0 - 91.0 * 2.0 + 10.0, LAYOUT.height - 10.0);
        assert_eq!(
            touch(&mut c, 3, TouchPhase::Started, first_slot, t),
            [TouchOutput::Hotbar(0)]
        );
    }

    #[test]
    fn custom_keyboard_button_sends_press_and_releases_on_cancel_or_gameplay_loss() {
        let (mut controls, now) = (TouchControls::default(), Instant::now());
        let mut layout = (*LAYOUT).clone();
        layout.hud.custom_buttons.push(CustomTouchButton {
            input_id: "key.keyboard.r".into(),
            position: [0.5, 0.5],
            scale: 1.0,
            visible: true,
        });
        let centre = layout.custom_button_centre(0);
        assert_eq!(
            controls.on_touch(90, TouchPhase::Started, centre, GAMEPLAY, &layout, now),
            [TouchOutput::VirtualKeyPress(KeyCode::KeyR)]
        );
        assert_eq!(
            controls.on_touch(90, TouchPhase::Cancelled, centre, GAMEPLAY, &layout, now),
            [TouchOutput::VirtualKeyRelease(KeyCode::KeyR)]
        );

        assert_eq!(
            controls.on_touch(91, TouchPhase::Started, centre, GAMEPLAY, &layout, now),
            [TouchOutput::VirtualKeyPress(KeyCode::KeyR)]
        );
        assert!(
            controls
                .on_touch(92, TouchPhase::Started, centre, GAMEPLAY, &layout, now)
                .is_empty()
        );
        assert_eq!(
            controls.tick(ms(now, 16), false),
            [TouchOutput::VirtualKeyRelease(KeyCode::KeyR)]
        );
        assert!(
            controls
                .on_touch(91, TouchPhase::Ended, centre, MENU, &layout, now)
                .is_empty()
        );
        assert!(
            controls
                .on_touch(92, TouchPhase::Ended, centre, MENU, &layout, now)
                .is_empty()
        );
    }


    #[test]
    fn shared_custom_key_releases_only_after_the_last_finger_lifts() {
        let (mut controls, now) = (TouchControls::default(), Instant::now());
        let mut layout = (*LAYOUT).clone();
        layout.hud.custom_buttons.push(CustomTouchButton {
            input_id: "key.keyboard.r".into(),
            position: [0.5, 0.5],
            scale: 1.0,
            visible: true,
        });
        let centre = layout.custom_button_centre(0);
        assert_eq!(
            controls.on_touch(1, TouchPhase::Started, centre, GAMEPLAY, &layout, now),
            [TouchOutput::VirtualKeyPress(KeyCode::KeyR)]
        );
        assert!(
            controls
                .on_touch(2, TouchPhase::Started, centre, GAMEPLAY, &layout, now)
                .is_empty()
        );
        assert!(
            controls
                .on_touch(1, TouchPhase::Ended, centre, GAMEPLAY, &layout, now)
                .is_empty()
        );
        assert_eq!(
            controls.on_touch(2, TouchPhase::Ended, centre, GAMEPLAY, &layout, now),
            [TouchOutput::VirtualKeyRelease(KeyCode::KeyR)]
        );
    }

    #[test]
    fn outside_gameplay_a_touch_is_the_pointer() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        let p = Vec2::new(300.0, 200.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Started, p, INVENTORY, &LAYOUT, t),
            [TouchOutput::PointerMove(p)]
        );
        assert_eq!(
            c.on_touch(1, TouchPhase::Ended, p, INVENTORY, &LAYOUT, t),
            [
                TouchOutput::PointerMove(p),
                TouchOutput::PointerDown,
                TouchOutput::PointerUp,
            ]
        );
    }

    #[test]
    fn vertical_menu_drag_becomes_scroll_and_releases_the_pointer() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        let start = Vec2::new(500.0, 400.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Started, start, MENU, &LAYOUT, t),
            [TouchOutput::PointerMove(start)]
        );

        let moved = start + Vec2::new(3.0, -80.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Moved, moved, MENU, &LAYOUT, t),
            [TouchOutput::PointerMove(moved), TouchOutput::Scroll(-2.0),]
        );

        let moved_again = moved + Vec2::new(0.0, -40.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Moved, moved_again, MENU, &LAYOUT, t),
            [
                TouchOutput::PointerMove(moved_again),
                TouchOutput::Scroll(-1.0),
            ]
        );
        assert!(
            c.on_touch(1, TouchPhase::Ended, moved_again, MENU, &LAYOUT, t)
                .is_empty()
        );
    }

    #[test]
    fn vertical_inventory_drag_remains_a_pointer_drag() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        let start = Vec2::new(500.0, 400.0);
        c.on_touch(1, TouchPhase::Started, start, INVENTORY, &LAYOUT, t);
        let moved = start + Vec2::new(2.0, 50.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Moved, moved, INVENTORY, &LAYOUT, t),
            [TouchOutput::PointerDown, TouchOutput::PointerMove(moved)]
        );
        assert_eq!(
            c.on_touch(1, TouchPhase::Ended, moved, INVENTORY, &LAYOUT, t),
            [TouchOutput::PointerUp]
        );
    }

    #[test]
    fn horizontal_drag_keeps_pointer_ownership_after_vertical_motion() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        let start = Vec2::new(500.0, 400.0);
        c.on_touch(1, TouchPhase::Started, start, MENU, &LAYOUT, t);
        let horizontal = start + Vec2::new(80.0, 5.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Moved, horizontal, MENU, &LAYOUT, t),
            [
                TouchOutput::PointerDown,
                TouchOutput::PointerMove(horizontal)
            ]
        );
        let vertical = horizontal + Vec2::new(0.0, 80.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Moved, vertical, MENU, &LAYOUT, t),
            [TouchOutput::PointerMove(vertical)]
        );
        assert_eq!(
            c.on_touch(1, TouchPhase::Ended, vertical, MENU, &LAYOUT, t),
            [TouchOutput::PointerUp]
        );
    }

    #[test]
    fn horizontal_menu_drag_remains_a_pointer_drag() {
        let (mut c, t) = (TouchControls::default(), Instant::now());
        let start = Vec2::new(500.0, 400.0);
        c.on_touch(1, TouchPhase::Started, start, MENU, &LAYOUT, t);
        let moved = start + Vec2::new(80.0, 5.0);
        assert_eq!(
            c.on_touch(1, TouchPhase::Moved, moved, MENU, &LAYOUT, t),
            [TouchOutput::PointerDown, TouchOutput::PointerMove(moved)]
        );
        assert_eq!(
            c.on_touch(1, TouchPhase::Ended, moved, MENU, &LAYOUT, t),
            [TouchOutput::PointerUp]
        );
    }
}
