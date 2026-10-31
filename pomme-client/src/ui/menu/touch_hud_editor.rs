use winit::keyboard::KeyCode;

use super::*;
use crate::app::input::InputKey;
use crate::app::input::touch::{
    BUTTON_RADIUS, CustomTouchButton, MAX_CUSTOM_TOUCH_BUTTONS, STICK_RADIUS, TouchButton,
    TouchHudLayout,
};

const CUSTOM_KEY_CHOICES: &[KeyCode] = &[
    KeyCode::KeyA,
    KeyCode::KeyB,
    KeyCode::KeyC,
    KeyCode::KeyD,
    KeyCode::KeyE,
    KeyCode::KeyF,
    KeyCode::KeyG,
    KeyCode::KeyH,
    KeyCode::KeyI,
    KeyCode::KeyJ,
    KeyCode::KeyK,
    KeyCode::KeyL,
    KeyCode::KeyM,
    KeyCode::KeyN,
    KeyCode::KeyO,
    KeyCode::KeyP,
    KeyCode::KeyQ,
    KeyCode::KeyR,
    KeyCode::KeyS,
    KeyCode::KeyT,
    KeyCode::KeyU,
    KeyCode::KeyV,
    KeyCode::KeyW,
    KeyCode::KeyX,
    KeyCode::KeyY,
    KeyCode::KeyZ,
    KeyCode::Digit0,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
    KeyCode::Space,
    KeyCode::Tab,
    KeyCode::Enter,
    KeyCode::Slash,
    KeyCode::Comma,
    KeyCode::Period,
    KeyCode::Backspace,
    KeyCode::ArrowUp,
    KeyCode::ArrowDown,
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
];

fn custom_key_id(key: KeyCode) -> String {
    InputKey::Keyboard(key)
        .input_id()
        .expect("custom key choices must have a stable input ID")
        .into_owned()
}

fn cycle_custom_key(input_id: &str, direction: i32) -> String {
    let current = match InputKey::from_input_id(input_id) {
        Ok(InputKey::Keyboard(key)) => key,
        _ => KeyCode::KeyR,
    };
    let current_index = CUSTOM_KEY_CHOICES
        .iter()
        .position(|&key| key == current)
        .unwrap_or(0) as i32;
    let next = (current_index + direction).rem_euclid(CUSTOM_KEY_CHOICES.len() as i32) as usize;
    custom_key_id(CUSTOM_KEY_CHOICES[next])
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
            "Key -",
            self.touch_hud_custom_selected.is_some(),
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
            "Key +",
            self.touch_hud_custom_selected.is_some(),
        );

        if add_custom && draft.custom_buttons.len() < MAX_CUSTOM_TOUCH_BUTTONS {
            let index = draft.custom_buttons.len();
            let key = CUSTOM_KEY_CHOICES[index % CUSTOM_KEY_CHOICES.len()];
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
        if let Some(index) = self.touch_hud_custom_selected
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
