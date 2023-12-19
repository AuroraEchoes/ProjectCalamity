use juno::vector::{IVec2, Vector};
use raylib::{
    ffi::{GamepadButton, KeyboardKey, MouseButton},
    RaylibHandle,
};

use crate::sector::Sector;

pub struct Selection {
    tile: Option<IVec2>,
}

impl Selection {
    pub fn new() -> Self {
        return Selection { tile: None };
    }

    pub fn tile(&self) -> &Option<IVec2> {
        return &self.tile;
    }
}

pub fn handle_inputs(rl: &mut RaylibHandle, sector: &Sector, selection: &mut Selection) {
    let edge = i32::min(1200 / sector.width(), 720 / sector.height());
    // Mouse
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        let mouse_pos = IVec2::new(rl.get_mouse_x(), rl.get_mouse_y());

        // In bounds
        let size = sector.size().clone();
        if mouse_pos.x() < &(size.x() * edge) && mouse_pos.y() < &(size.y() * edge) {
            let pos = mouse_pos / edge;
            selection.tile = match &selection.tile {
                Some(prev_pos) => {
                    if prev_pos != &pos {
                        Some(pos)
                    } else {
                        None
                    }
                }
                None => Some(pos),
            }
        }
    }
}
