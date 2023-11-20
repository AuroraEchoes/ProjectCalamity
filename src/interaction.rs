use raylib::{RaylibHandle, ffi::{MouseButton, GamepadButton, KeyboardKey}};

use crate::{utility::GridPosVec, sector::Sector};

pub struct Selection {
    tile: Option<GridPosVec>
}

impl Selection {
    pub fn new() -> Self {
        return Selection { tile: None }
    }

    pub fn tile(&self) -> &Option<GridPosVec> {
        return &self.tile;
    }
}

pub fn handle_inputs(rl: &mut RaylibHandle, sector: &Sector, selection: &mut Selection) {
    let edge = usize::min(1200 / sector.width(), 720 / sector.height());
    // Mouse
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        let mouse_pos = GridPosVec::new(rl.get_mouse_x() as usize, rl.get_mouse_y() as usize);

        // In bounds
        let size = *sector.size();
        if mouse_pos < (size * edge) {
            let pos = mouse_pos / edge;
            selection.tile = match selection.tile {
                Some(prev_pos) => if prev_pos != pos { Some(pos) } else { None },
                None => Some(pos),
            }
        }
    }

    // Keyboard
    if let Some(mut pos) = selection.tile {
        if rl.is_key_pressed( KeyboardKey::KEY_RIGHT) {
            if pos.x() + 1 < sector.width() {
                pos.add_x(1);
            }
        }
        if rl.is_key_pressed( KeyboardKey::KEY_LEFT) {
            if pos.x() > 0 {
                pos.subtract_x(1);
            }
        }
        if rl.is_key_pressed( KeyboardKey::KEY_DOWN) {
            if pos.y() + 1 < sector.height() {
                pos.add_y(1);
            }
        }
        if rl.is_key_pressed( KeyboardKey::KEY_UP) {
            if pos.y() > 0 {
                pos.subtract_y(1);
            }
        }
        selection.tile = Some(pos);
    }

    // Gamepad
    if let Some(mut pos) = selection.tile {
        if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
            if pos.x() + 1 < sector.width() {
                pos.add_x(1);
            }
        }
        if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
            if pos.x() > 0 {
                pos.subtract_x(1);
            }
        }
        if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
            if pos.y() + 1 < sector.height() {
                pos.add_y(1);
            }
        }
        if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP) {
            if pos.y() > 0 {
                pos.subtract_y(1);
            }
        }
        selection.tile = Some(pos);
    }
}