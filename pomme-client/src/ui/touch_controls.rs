//! Draws the touch controls (`app::input::touch`) over the HUD: the move
//! stick, resting bottom-left until a press places it, and the buttons.

use glam::Vec2;

use crate::app::input::{InputKey, touch::{STICK_RADIUS, TouchButton, TouchControls, TouchLayout}};
use crate::renderer::pipelines::menu_overlay::MenuElement;
use crate::ui::common::{FONT_SIZE, WHITE};

const BASE: [f32; 4] = [0.0, 0.0, 0.0, 0.35];
const KNOB: [f32; 4] = [1.0, 1.0, 1.0, 0.45];

pub fn build_touch_controls(
    elements: &mut Vec<MenuElement>,
    layout: &TouchLayout,
    touch: &TouchControls,
) {
    let gs = layout.gs;
    let radius = STICK_RADIUS * gs;
    let base = touch.stick_origin().unwrap_or_else(|| layout.stick_home());
    push_circle(elements, base, radius, BASE);
    let deflection = touch.stick().map_or(Vec2::ZERO, |v| Vec2::new(v.x, -v.y));
    push_circle(elements, base + deflection * radius, radius * 0.45, KNOB);

    for (index, button) in layout.hud.custom_buttons.iter().enumerate() {
        if !button.visible {
            continue;
        }
        let centre = layout.custom_button_centre(index);
        push_circle(elements, centre, layout.custom_button_radius(index), BASE);
        let label = InputKey::from_input_id(&button.input_id)
            .map(|key| key.display_name().into_owned())
            .unwrap_or_else(|_| "?".to_owned());
        elements.push(MenuElement::Text {
            x: centre.x,
            y: centre.y - FONT_SIZE * gs / 2.0,
            text: label,
            scale: FONT_SIZE * gs,
            color: WHITE,
            centered: true,
        });
    }

    for button in TouchButton::ALL {
        if !layout.hud.is_visible(button) {
            continue;
        }
        let centre = layout.button_centre(button);
        push_circle(elements, centre, layout.button_radius(button), BASE);
        elements.push(MenuElement::Text {
            x: centre.x,
            y: centre.y - FONT_SIZE * gs / 2.0,
            text: button.label().to_owned(),
            scale: FONT_SIZE * gs,
            color: WHITE,
            centered: true,
        });
    }
}

fn push_circle(elements: &mut Vec<MenuElement>, centre: Vec2, radius: f32, color: [f32; 4]) {
    elements.push(MenuElement::Rect {
        x: centre.x - radius,
        y: centre.y - radius,
        w: 2.0 * radius,
        h: 2.0 * radius,
        corner_radius: radius,
        color,
    });
}
