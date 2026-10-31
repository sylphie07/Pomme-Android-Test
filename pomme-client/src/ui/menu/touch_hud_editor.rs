use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use super::*;
use crate::app::input::InputKey;
use crate::app::input::touch::{
    BUTTON_RADIUS, CustomTouchButton, MAX_CUSTOM_TOUCH_BUTTONS, STICK_RADIUS, TouchButton,
    TouchHudLayout,
};

const CUSTOM_INPUT_CHOICES: &[InputKey] = &[
    InputKey::Keyboard(KeyCode::Backquote),
    InputKey::Keyboard(KeyCode::Backslash),
    InputKey::Keyboard(KeyCode::BracketLeft),
    InputKey::Keyboard(KeyCode::BracketRight),
    InputKey::Keyboard(KeyCode::Comma),
    InputKey::Keyboard(KeyCode::Digit0),
    InputKey::Keyboard(KeyCode::Digit1),
    InputKey::Keyboard(KeyCode::Digit2),
    InputKey::Keyboard(KeyCode::Digit3),
    InputKey::Keyboard(KeyCode::Digit4),
    InputKey::Keyboard(KeyCode::Digit5),
    InputKey::Keyboard(KeyCode::Digit6),
    InputKey::Keyboard(KeyCode::Digit7),
    InputKey::Keyboard(KeyCode::Digit8),
    InputKey::Keyboard(KeyCode::Digit9),
    InputKey::Keyboard(KeyCode::Equal),
    InputKey::Keyboard(KeyCode::IntlBackslash),
    InputKey::Keyboard(KeyCode::IntlRo),
    InputKey::Keyboard(KeyCode::IntlYen),
    InputKey::Keyboard(KeyCode::KeyA),
    InputKey::Keyboard(KeyCode::KeyB),
    InputKey::Keyboard(KeyCode::KeyC),
    InputKey::Keyboard(KeyCode::KeyD),
    InputKey::Keyboard(KeyCode::KeyE),
    InputKey::Keyboard(KeyCode::KeyF),
    InputKey::Keyboard(KeyCode::KeyG),
    InputKey::Keyboard(KeyCode::KeyH),
    InputKey::Keyboard(KeyCode::KeyI),
    InputKey::Keyboard(KeyCode::KeyJ),
    InputKey::Keyboard(KeyCode::KeyK),
    InputKey::Keyboard(KeyCode::KeyL),
    InputKey::Keyboard(KeyCode::KeyM),
    InputKey::Keyboard(KeyCode::KeyN),
    InputKey::Keyboard(KeyCode::KeyO),
    InputKey::Keyboard(KeyCode::KeyP),
    InputKey::Keyboard(KeyCode::KeyQ),
    InputKey::Keyboard(KeyCode::KeyR),
    InputKey::Keyboard(KeyCode::KeyS),
    InputKey::Keyboard(KeyCode::KeyT),
    InputKey::Keyboard(KeyCode::KeyU),
    InputKey::Keyboard(KeyCode::KeyV),
    InputKey::Keyboard(KeyCode::KeyW),
    InputKey::Keyboard(KeyCode::KeyX),
    InputKey::Keyboard(KeyCode::KeyY),
    InputKey::Keyboard(KeyCode::KeyZ),
    InputKey::Keyboard(KeyCode::Minus),
    InputKey::Keyboard(KeyCode::Period),
    InputKey::Keyboard(KeyCode::Quote),
    InputKey::Keyboard(KeyCode::Semicolon),
    InputKey::Keyboard(KeyCode::Slash),
    InputKey::Keyboard(KeyCode::AltLeft),
    InputKey::Keyboard(KeyCode::AltRight),
    InputKey::Keyboard(KeyCode::Backspace),
    InputKey::Keyboard(KeyCode::CapsLock),
    InputKey::Keyboard(KeyCode::ContextMenu),
    InputKey::Keyboard(KeyCode::ControlLeft),
    InputKey::Keyboard(KeyCode::ControlRight),
    InputKey::Keyboard(KeyCode::Enter),
    InputKey::Keyboard(KeyCode::SuperLeft),
    InputKey::Keyboard(KeyCode::SuperRight),
    InputKey::Keyboard(KeyCode::ShiftLeft),
    InputKey::Keyboard(KeyCode::ShiftRight),
    InputKey::Keyboard(KeyCode::Space),
    InputKey::Keyboard(KeyCode::Tab),
    InputKey::Keyboard(KeyCode::Convert),
    InputKey::Keyboard(KeyCode::KanaMode),
    InputKey::Keyboard(KeyCode::Lang1),
    InputKey::Keyboard(KeyCode::Lang2),
    InputKey::Keyboard(KeyCode::Lang3),
    InputKey::Keyboard(KeyCode::Lang4),
    InputKey::Keyboard(KeyCode::Lang5),
    InputKey::Keyboard(KeyCode::NonConvert),
    InputKey::Keyboard(KeyCode::Delete),
    InputKey::Keyboard(KeyCode::End),
    InputKey::Keyboard(KeyCode::Help),
    InputKey::Keyboard(KeyCode::Home),
    InputKey::Keyboard(KeyCode::Insert),
    InputKey::Keyboard(KeyCode::PageDown),
    InputKey::Keyboard(KeyCode::PageUp),
    InputKey::Keyboard(KeyCode::ArrowDown),
    InputKey::Keyboard(KeyCode::ArrowLeft),
    InputKey::Keyboard(KeyCode::ArrowRight),
    InputKey::Keyboard(KeyCode::ArrowUp),
    InputKey::Keyboard(KeyCode::NumLock),
    InputKey::Keyboard(KeyCode::Numpad0),
    InputKey::Keyboard(KeyCode::Numpad1),
    InputKey::Keyboard(KeyCode::Numpad2),
    InputKey::Keyboard(KeyCode::Numpad3),
    InputKey::Keyboard(KeyCode::Numpad4),
    InputKey::Keyboard(KeyCode::Numpad5),
    InputKey::Keyboard(KeyCode::Numpad6),
    InputKey::Keyboard(KeyCode::Numpad7),
    InputKey::Keyboard(KeyCode::Numpad8),
    InputKey::Keyboard(KeyCode::Numpad9),
    InputKey::Keyboard(KeyCode::NumpadAdd),
    InputKey::Keyboard(KeyCode::NumpadBackspace),
    InputKey::Keyboard(KeyCode::NumpadClear),
    InputKey::Keyboard(KeyCode::NumpadClearEntry),
    InputKey::Keyboard(KeyCode::NumpadComma),
    InputKey::Keyboard(KeyCode::NumpadDecimal),
    InputKey::Keyboard(KeyCode::NumpadDivide),
    InputKey::Keyboard(KeyCode::NumpadEnter),
    InputKey::Keyboard(KeyCode::NumpadEqual),
    InputKey::Keyboard(KeyCode::NumpadHash),
    InputKey::Keyboard(KeyCode::NumpadMemoryAdd),
    InputKey::Keyboard(KeyCode::NumpadMemoryClear),
    InputKey::Keyboard(KeyCode::NumpadMemoryRecall),
    InputKey::Keyboard(KeyCode::NumpadMemoryStore),
    InputKey::Keyboard(KeyCode::NumpadMemorySubtract),
    InputKey::Keyboard(KeyCode::NumpadMultiply),
    InputKey::Keyboard(KeyCode::NumpadParenLeft),
    InputKey::Keyboard(KeyCode::NumpadParenRight),
    InputKey::Keyboard(KeyCode::NumpadStar),
    InputKey::Keyboard(KeyCode::NumpadSubtract),
    InputKey::Keyboard(KeyCode::Escape),
    InputKey::Keyboard(KeyCode::Fn),
    InputKey::Keyboard(KeyCode::FnLock),
    InputKey::Keyboard(KeyCode::PrintScreen),
    InputKey::Keyboard(KeyCode::ScrollLock),
    InputKey::Keyboard(KeyCode::Pause),
    InputKey::Keyboard(KeyCode::BrowserBack),
    InputKey::Keyboard(KeyCode::BrowserFavorites),
    InputKey::Keyboard(KeyCode::BrowserForward),
    InputKey::Keyboard(KeyCode::BrowserHome),
    InputKey::Keyboard(KeyCode::BrowserRefresh),
    InputKey::Keyboard(KeyCode::BrowserSearch),
    InputKey::Keyboard(KeyCode::BrowserStop),
    InputKey::Keyboard(KeyCode::Eject),
    InputKey::Keyboard(KeyCode::LaunchApp1),
    InputKey::Keyboard(KeyCode::LaunchApp2),
    InputKey::Keyboard(KeyCode::LaunchMail),
    InputKey::Keyboard(KeyCode::MediaPlayPause),
    InputKey::Keyboard(KeyCode::MediaSelect),
    InputKey::Keyboard(KeyCode::MediaStop),
    InputKey::Keyboard(KeyCode::MediaTrackNext),
    InputKey::Keyboard(KeyCode::MediaTrackPrevious),
    InputKey::Keyboard(KeyCode::Power),
    InputKey::Keyboard(KeyCode::Sleep),
    InputKey::Keyboard(KeyCode::AudioVolumeDown),
    InputKey::Keyboard(KeyCode::AudioVolumeMute),
    InputKey::Keyboard(KeyCode::AudioVolumeUp),
    InputKey::Keyboard(KeyCode::WakeUp),
    InputKey::Keyboard(KeyCode::Meta),
    InputKey::Keyboard(KeyCode::Hyper),
    InputKey::Keyboard(KeyCode::Turbo),
    InputKey::Keyboard(KeyCode::Abort),
    InputKey::Keyboard(KeyCode::Resume),
    InputKey::Keyboard(KeyCode::Suspend),
    InputKey::Keyboard(KeyCode::Again),
    InputKey::Keyboard(KeyCode::Copy),
    InputKey::Keyboard(KeyCode::Cut),
    InputKey::Keyboard(KeyCode::Find),
    InputKey::Keyboard(KeyCode::Open),
    InputKey::Keyboard(KeyCode::Paste),
    InputKey::Keyboard(KeyCode::Props),
    InputKey::Keyboard(KeyCode::Select),
    InputKey::Keyboard(KeyCode::Undo),
    InputKey::Keyboard(KeyCode::Hiragana),
    InputKey::Keyboard(KeyCode::Katakana),
    InputKey::Keyboard(KeyCode::F1),
    InputKey::Keyboard(KeyCode::F2),
    InputKey::Keyboard(KeyCode::F3),
    InputKey::Keyboard(KeyCode::F4),
    InputKey::Keyboard(KeyCode::F5),
    InputKey::Keyboard(KeyCode::F6),
    InputKey::Keyboard(KeyCode::F7),
    InputKey::Keyboard(KeyCode::F8),
    InputKey::Keyboard(KeyCode::F9),
    InputKey::Keyboard(KeyCode::F10),
    InputKey::Keyboard(KeyCode::F11),
    InputKey::Keyboard(KeyCode::F12),
    InputKey::Keyboard(KeyCode::F13),
    InputKey::Keyboard(KeyCode::F14),
    InputKey::Keyboard(KeyCode::F15),
    InputKey::Keyboard(KeyCode::F16),
    InputKey::Keyboard(KeyCode::F17),
    InputKey::Keyboard(KeyCode::F18),
    InputKey::Keyboard(KeyCode::F19),
    InputKey::Keyboard(KeyCode::F20),
    InputKey::Keyboard(KeyCode::F21),
    InputKey::Keyboard(KeyCode::F22),
    InputKey::Keyboard(KeyCode::F23),
    InputKey::Keyboard(KeyCode::F24),
    InputKey::Keyboard(KeyCode::F25),
    InputKey::Keyboard(KeyCode::F26),
    InputKey::Keyboard(KeyCode::F27),
    InputKey::Keyboard(KeyCode::F28),
    InputKey::Keyboard(KeyCode::F29),
    InputKey::Keyboard(KeyCode::F30),
    InputKey::Keyboard(KeyCode::F31),
    InputKey::Keyboard(KeyCode::F32),
    InputKey::Keyboard(KeyCode::F33),
    InputKey::Keyboard(KeyCode::F34),
    InputKey::Keyboard(KeyCode::F35),
    InputKey::Mouse(MouseButton::Left),
    InputKey::Mouse(MouseButton::Right),
    InputKey::Mouse(MouseButton::Middle),
    InputKey::Mouse(MouseButton::Back),
    InputKey::Mouse(MouseButton::Forward),
]

fn custom_key_id(key: InputKey) -> String {
    key.input_id()
        .expect("custom key choices must have a stable input ID")
        .into_owned()
}

fn cycle_custom_key(input_id: &str, direction: i32) -> String {
    let current = InputKey::from_input_id(input_id).unwrap_or(InputKey::Keyboard(KeyCode::KeyR));
    let current_index = CUSTOM_INPUT_CHOICES
        .iter()
        .position(|&key| key == current)
        .unwrap_or(0) as i32;
    let next = (current_index + direction).rem_euclid(CUSTOM_INPUT_CHOICES.len() as i32) as usize;
    custom_key_id(CUSTOM_INPUT_CHOICES[next])
}

fn custom_key_label(input_id: &str) -> String {
    InputKey::from_input_id(input_id)
        .map(|key| key.display_name().into_owned())
        .unwrap_or_else(|_| "?".to_owned())
}

fn button_index(button: TouchButton) -> usize {
    match button {
        TouchButton::Jump => 0,
        TouchButton::Chat => 1,
        TouchButton::Inventory => 2,
        TouchButton::Pause => 3,
    }
}

impl MainMenu {
    pub(super) fn build_touch_hud_editor(
        &mut self,
        sw: f32,
        sh: f32,
        input: &MenuInput,
        key_bindings: &KeyBindings,
    ) -> MainMenuResult {
        if input.escape {
            self.touch_hud_draft = None;
            self.touch_hud_selected = None;
            self.touch_hud_stick_selected = false;
            self.touch_hud_custom_selected = None;
            self.touch_hud_dragging = false;
            self.set_screen(Screen::OptionsControls, key_bindings);
            return empty_result();
        }

        let mut draft = self
            .touch_hud_draft
            .clone()
            .unwrap_or_else(|| self.touch_hud.clone());
        let gs = self.gui_scale(sw, sh);
        let mut elements = vec![MenuElement::BlurBackground];
        let chrome = helpers::push_screen_chrome(&mut elements, sw, sh, gs, "Touch Controls");
        // Use nearly the full viewport for the editing canvas. The toolbar
        // remains below it, while normalized coordinates preserve saved layouts
        // across different device resolutions and aspect ratios.
        let preview_x = sw * 0.02;
        let preview_y = chrome.content_top + 4.0 * gs;
        let preview_w = sw * 0.96;
        let controls_h = 100.0 * gs;
        let available_h = (chrome.content_bottom - preview_y - controls_h).max(100.0 * gs);
        let preview_h = available_h;

        // A framed miniature of the gameplay screen. Coordinates are normalized
        // exactly like the live HUD, so the editor and hit-testing share a model.
        elements.push(MenuElement::Rect {
            x: preview_x,
            y: preview_y,
            w: preview_w,
            h: preview_h,
            corner_radius: 5.0 * gs,
            color: [0.025, 0.035, 0.06, 0.88],
        });
        elements.push(MenuElement::Rect {
            x: preview_x + 2.0 * gs,
            y: preview_y + 2.0 * gs,
            w: preview_w - 4.0 * gs,
            h: preview_h - 4.0 * gs,
            corner_radius: 3.0 * gs,
            color: [0.12, 0.16, 0.2, 0.25],
        });
        let fs = common::FONT_SIZE * gs;
        elements.push(MenuElement::Text {
            x: preview_x + 8.0 * gs,
            y: preview_y + 8.0 * gs,
            text: "Drag a control to move it".into(),
            scale: fs,
            color: COL_DIM,
            centered: false,
        });

        // The movement stick is shown for context but is not editable in this
        // first editor version; it remains anchored by the gameplay layout.
        let stick_radius = STICK_RADIUS * gs * draft.stick_scale;
        let stick_inset = 1.6 * stick_radius;
        let stick_x = preview_x + stick_inset;
        let stick_y = preview_y + preview_h - stick_inset;
        elements.push(MenuElement::Rect {
            x: stick_x - stick_radius,
            y: stick_y - stick_radius,
            w: 2.0 * stick_radius,
            h: 2.0 * stick_radius,
            corner_radius: stick_radius,
            color: [0.8, 0.85, 0.9, 0.13],
        });
        elements.push(MenuElement::Rect {
            x: stick_x - stick_radius * 0.72,
            y: stick_y - stick_radius * draft.sprint_threshold,
            w: stick_radius * 1.44,
            h: 2.0 * gs,
            corner_radius: gs,
            color: [0.96, 0.72, 0.18, 0.78],
        });
        elements.push(MenuElement::Text {
            x: stick_x,
            y: stick_y - fs / 2.0,
            text: "MOVE / SPRINT".into(),
            scale: fs * 0.8,
            color: COL_DIM,
            centered: true,
        });

        elements.push(MenuElement::Text {
            x: stick_x,
            y: stick_y - stick_radius - fs * 0.9,
            text: format!("SPRINT > {}%", (draft.sprint_threshold * 100.0).round() as u32),
            scale: fs * 0.62,
            color: COL_DIM,
            centered: true,
        });

        // A touch press becomes a pointer drag after the platform's gesture
        // threshold. Keep a generous selection radius so that transition does
        // not make a button impossible to grab on a phone.
        if input.clicked {
            let mut selected_builtin = false;
            let stick_dx = input.cursor.0 - stick_x;
            let stick_dy = input.cursor.1 - stick_y;
            if stick_dx * stick_dx + stick_dy * stick_dy <= (stick_radius * 2.0).powi(2) {
                self.touch_hud_selected = None;
                self.touch_hud_stick_selected = true;
                self.touch_hud_custom_selected = None;
                self.touch_hud_dragging = false;
                selected_builtin = true;
            }
            for button in TouchButton::ALL {
                if selected_builtin {
                    break;
                }
                let index = button_index(button);
                let [nx, ny] = draft.positions[index];
                let centre = (preview_x + nx * preview_w, preview_y + ny * preview_h);
                let radius = BUTTON_RADIUS * gs * draft.scales[index];
                let dx = input.cursor.0 - centre.0;
                let dy = input.cursor.1 - centre.1;
                if dx * dx + dy * dy <= (radius * 2.0).powi(2) {
                    self.touch_hud_selected = Some(button);
                    self.touch_hud_stick_selected = false;
                    self.touch_hud_custom_selected = None;
                    self.touch_hud_dragging = input.mouse_held;
                    self.touch_hud_drag_offset = (
                        nx - (input.cursor.0 - preview_x) / preview_w,
                        ny - (input.cursor.1 - preview_y) / preview_h,
                    );
                    selected_builtin = true;
                    break;
                }
            }
            if !selected_builtin {
                for (index, button) in draft.custom_buttons.iter().enumerate() {
                    let [nx, ny] = button.position;
                    let centre = (preview_x + nx * preview_w, preview_y + ny * preview_h);
                    let radius = BUTTON_RADIUS * gs * button.scale;
                    let dx = input.cursor.0 - centre.0;
                    let dy = input.cursor.1 - centre.1;
                    if dx * dx + dy * dy <= (radius * 2.0).powi(2) {
                        self.touch_hud_selected = None;
                        self.touch_hud_stick_selected = false;
                        self.touch_hud_custom_selected = Some(index);
                        self.touch_hud_dragging = input.mouse_held;
                        self.touch_hud_drag_offset = (
                            nx - (input.cursor.0 - preview_x) / preview_w,
                            ny - (input.cursor.1 - preview_y) / preview_h,
                        );
                        break;
                    }
                }
            }
        }

        if let Some(button) = self.touch_hud_selected {
            if self.touch_hud_dragging && input.mouse_held {
                let index = button_index(button);
                draft.positions[index] = [
                    ((input.cursor.0 - preview_x) / preview_w + self.touch_hud_drag_offset.0)
                        .clamp(0.015, 0.985),
                    ((input.cursor.1 - preview_y) / preview_h + self.touch_hud_drag_offset.1)
                        .clamp(0.015, 0.985),
                ];
            } else if !input.mouse_held {
                self.touch_hud_dragging = false;
            }
        } else if let Some(index) = self.touch_hud_custom_selected {
            if let Some(button) = draft.custom_buttons.get_mut(index) {
                if self.touch_hud_dragging && input.mouse_held {
                    button.position = [
                        ((input.cursor.0 - preview_x) / preview_w + self.touch_hud_drag_offset.0)
                            .clamp(0.015, 0.985),
                        ((input.cursor.1 - preview_y) / preview_h + self.touch_hud_drag_offset.1)
                            .clamp(0.015, 0.985),
                    ];
                } else if !input.mouse_held {
                    self.touch_hud_dragging = false;
                }
            } else {
                self.touch_hud_custom_selected = None;
                self.touch_hud_dragging = false;
            }
        }

        for button in TouchButton::ALL {
            let index = button_index(button);
            let [nx, ny] = draft.positions[index];
            let centre_x = preview_x + nx * preview_w;
            let centre_y = preview_y + ny * preview_h;
            let radius = BUTTON_RADIUS * gs * draft.scales[index];
            let selected = self.touch_hud_selected == Some(button);
            let visible = draft.visible[index];
            let color = if selected {
                [0.18, 0.72, 0.42, if visible { 0.9 } else { 0.38 }]
            } else if visible {
                [0.12, 0.52, 0.78, 0.78]
            } else {
                [0.35, 0.38, 0.42, 0.35]
            };
            elements.push(MenuElement::Rect {
                x: centre_x - radius,
                y: centre_y - radius,
                w: radius * 2.0,
                h: radius * 2.0,
                corner_radius: radius,
                color,
            });
            elements.push(MenuElement::Text {
                x: centre_x,
                y: centre_y - fs / 2.0,
                text: button.label().into(),
                scale: fs,
                color: WHITE,
                centered: true,
            });
            if !visible {
                elements.push(MenuElement::Text {
                    x: centre_x,
                    y: centre_y + radius + 2.0 * gs,
                    text: "HIDDEN".into(),
                    scale: fs * 0.7,
                    color: COL_DIM,
                    centered: true,
                });
            }
        }

        for (index, button) in draft.custom_buttons.iter().enumerate() {
            let [nx, ny] = button.position;
            let centre_x = preview_x + nx * preview_w;
            let centre_y = preview_y + ny * preview_h;
            let radius = BUTTON_RADIUS * gs * button.scale;
            let selected = self.touch_hud_custom_selected == Some(index);
            let color = if selected {
                [0.18, 0.72, 0.42, if button.visible { 0.9 } else { 0.38 }]
            } else if button.visible {
                [0.12, 0.52, 0.78, 0.78]
            } else {
                [0.35, 0.38, 0.42, 0.35]
            };
            elements.push(MenuElement::Rect {
                x: centre_x - radius,
                y: centre_y - radius,
                w: radius * 2.0,
                h: radius * 2.0,
                corner_radius: radius,
                color,
            });
            elements.push(MenuElement::Text {
                x: centre_x,
                y: centre_y - fs / 2.0,
                text: custom_key_label(&button.input_id),
                scale: fs,
                color: WHITE,
                centered: true,
            });
            if !button.visible {
                elements.push(MenuElement::Text {
                    x: centre_x,
                    y: centre_y + radius + 2.0 * gs,
                    text: "HIDDEN".into(),
                    scale: fs * 0.7,
                    color: COL_DIM,
                    centered: true,
                });
            }
        }

        let mut any_hovered = false;
        self.focus_advance(input);
        let mut ctx = self.make_focus_ctx(input);
        let row_y = preview_y + preview_h + 5.0 * gs;
        let row_h = common::BTN_H * gs;
        let gap = 4.0 * gs;
        let small_w = (sw.min(420.0 * gs) - 4.0 * gap) / 4.0;
        let row_x = (sw - (small_w * 4.0 + gap * 3.0)) / 2.0;
        let selected = self.touch_hud_selected;
        let labels = ["Jump", "Chat", "Inventory", "Pause"];
        for (i, button) in TouchButton::ALL.into_iter().enumerate() {
            let x = row_x + i as f32 * (small_w + gap);
            let active = selected == Some(button);
            if helpers::push_button_f(
                &mut elements,
                &mut ctx,
                &mut any_hovered,
                input.cursor,
                input.clicked,
                x,
                row_y,
                small_w,
                row_h,
                gs,
                labels[i],
                true,
            ) {
                self.touch_hud_selected = Some(button);
                self.touch_hud_stick_selected = false;
                self.touch_hud_custom_selected = None;
                self.touch_hud_dragging = false;
            }
            if active {
                elements.push(MenuElement::Rect {
                    x: x + 1.0 * gs,
                    y: row_y + row_h - 2.0 * gs,
                    w: small_w - 2.0 * gs,
                    h: 1.0 * gs,
                    corner_radius: 0.0,
                    color: [0.2, 0.85, 0.48, 1.0],
                });
            }
        }

        let custom_row_y = row_y + row_h + 3.0 * gs;
        let action_w = (sw.min(420.0 * gs) - 3.0 * gap) / 4.0;
        let action_x = (sw - (action_w * 4.0 + gap * 3.0)) / 2.0;
        let mut add_custom = false;
        let mut remove_custom = false;
        let mut key_previous = false;
        let mut key_next = false;
        add_custom |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x,
            custom_row_y,
            action_w,
            row_h,
            gs,
            "Add",
            draft.custom_buttons.len() < MAX_CUSTOM_TOUCH_BUTTONS,
        );
        remove_custom |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x + action_w + gap,
            custom_row_y,
            action_w,
            row_h,
            gs,
            "Remove",
            self.touch_hud_custom_selected.is_some(),
        );
        key_previous |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x + (action_w + gap) * 2.0,
            custom_row_y,
            action_w,
            row_h,
            gs,
            if self.touch_hud_stick_selected { "Sprint -" } else { "Key -" },
            self.touch_hud_custom_selected.is_some() || self.touch_hud_stick_selected,
        );
        key_next |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x + (action_w + gap) * 3.0,
            custom_row_y,
            action_w,
            row_h,
            gs,
            if self.touch_hud_stick_selected { "Sprint +" } else { "Key +" },
            self.touch_hud_custom_selected.is_some() || self.touch_hud_stick_selected,
        );

        if add_custom && draft.custom_buttons.len() < MAX_CUSTOM_TOUCH_BUTTONS {
            let index = draft.custom_buttons.len();
            let key = CUSTOM_INPUT_CHOICES[index % CUSTOM_INPUT_CHOICES.len()];
            let column = (index % 4) as f32;
            let row = (index / 4) as f32;
            draft.custom_buttons.push(CustomTouchButton {
                input_id: custom_key_id(key),
                position: [
                    (0.34 + column * 0.09).min(0.70),
                    (0.32 + row * 0.10).min(0.72),
                ],
                scale: 1.0,
                visible: true,
            });
            self.touch_hud_selected = None;
            self.touch_hud_stick_selected = false;
            self.touch_hud_custom_selected = Some(index);
            self.touch_hud_dragging = false;
        }
        if remove_custom {
            if let Some(index) = self.touch_hud_custom_selected.take()
                && index < draft.custom_buttons.len()
            {
                draft.custom_buttons.remove(index);
            }
            self.touch_hud_dragging = false;
        }
        if self.touch_hud_stick_selected {
            if key_previous {
                draft.sprint_threshold = (draft.sprint_threshold - 0.02).max(0.5);
            }
            if key_next {
                draft.sprint_threshold = (draft.sprint_threshold + 0.02).min(0.98);
            }
        } else if let Some(index) = self.touch_hud_custom_selected
            && let Some(button) = draft.custom_buttons.get_mut(index)
        {
            if key_previous {
                button.input_id = cycle_custom_key(&button.input_id, -1);
            }
            if key_next {
                button.input_id = cycle_custom_key(&button.input_id, 1);
            }
        }

        let action_y = custom_row_y + row_h + 3.0 * gs;
        let selected_index = selected.map(button_index);
        let custom_selected_index = self
            .touch_hud_custom_selected
            .filter(|&index| index < draft.custom_buttons.len());
        let has_selection = selected_index.is_some()
            || custom_selected_index.is_some()
            || self.touch_hud_stick_selected;
        let mut size_down = false;
        let mut size_up = false;
        let mut toggle_visibility = false;
        let mut reset = false;
        size_down |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x,
            action_y,
            action_w,
            row_h,
            gs,
            "Size -",
            has_selection,
        );
        size_up |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x + action_w + gap,
            action_y,
            action_w,
            row_h,
            gs,
            "Size +",
            has_selection,
        );
        toggle_visibility |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x + (action_w + gap) * 2.0,
            action_y,
            action_w,
            row_h,
            gs,
            "Show / Hide",
            has_selection,
        );
        reset |= helpers::push_button_f(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input.cursor,
            input.clicked,
            action_x + (action_w + gap) * 3.0,
            action_y,
            action_w,
            row_h,
            gs,
            "Reset",
            true,
        );

        if self.touch_hud_stick_selected {
            if size_down {
                draft.stick_scale = (draft.stick_scale - 0.1).max(0.6);
            }
            if size_up {
                draft.stick_scale = (draft.stick_scale + 0.1).min(1.8);
            }
        } else if let Some(index) = selected_index {
            if size_down {
                draft.scales[index] = (draft.scales[index] - 0.1).max(0.6);
            }
            if size_up {
                draft.scales[index] = (draft.scales[index] + 0.1).min(1.8);
            }
            if toggle_visibility {
                draft.visible[index] = !draft.visible[index];
            }
        } else if let Some(index) = custom_selected_index {
            let button = &mut draft.custom_buttons[index];
            if size_down {
                button.scale = (button.scale - 0.1).max(0.6);
            }
            if size_up {
                button.scale = (button.scale + 0.1).min(1.8);
            }
            if toggle_visibility {
                button.visible = !button.visible;
            }
        }

        if reset {
            draft = TouchHudLayout::default();
            self.touch_hud_selected = None;
            self.touch_hud_stick_selected = false;
            self.touch_hud_custom_selected = None;
            self.touch_hud_dragging = false;
        }

        let cx = sw / 2.0;
        let footer_w = helpers::FOOTER_PAIR_W * gs;
        let footer_gap = helpers::FOOTER_PAIR_GAP * gs;
        let cancel_x = cx - footer_w - footer_gap / 2.0;
        let cancel = helpers::push_footer_button(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input,
            &chrome,
            cancel_x,
            footer_w,
            gs,
            "Cancel",
            true,
        );
        let save = helpers::push_footer_button(
            &mut elements,
            &mut ctx,
            &mut any_hovered,
            input,
            &chrome,
            cancel_x + footer_w + footer_gap,
            footer_w,
            gs,
            "Save",
            true,
        );

        self.finish_focus(&ctx);
        if save {
            self.touch_hud = draft.sanitized();
            self.settings_dirty = true;
            self.touch_hud_draft = None;
            self.touch_hud_selected = None;
            self.touch_hud_stick_selected = false;
            self.touch_hud_custom_selected = None;
            self.touch_hud_dragging = false;
            self.set_screen(Screen::OptionsControls, key_bindings);
        } else if cancel {
            self.touch_hud_draft = None;
            self.touch_hud_selected = None;
            self.touch_hud_custom_selected = None;
            self.touch_hud_dragging = false;
            self.set_screen(Screen::OptionsControls, key_bindings);
        } else {
            self.touch_hud_draft = Some(draft.clone().sanitized());
        }

        MainMenuResult {
            elements,
            action: MenuAction::None,
            cursor_pointer: any_hovered,
            clicked_button: ctx.fired,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_key_choices_cycle_in_both_directions() {
        assert_eq!(
            InputKey::from_input_id(&cycle_custom_key("key.keyboard.r", 1)).unwrap(),
            InputKey::Keyboard(KeyCode::KeyS)
        );
        assert_eq!(
            InputKey::from_input_id(&cycle_custom_key("key.keyboard.r", -1)).unwrap(),
            InputKey::Keyboard(KeyCode::KeyQ)
        );
    }
}
