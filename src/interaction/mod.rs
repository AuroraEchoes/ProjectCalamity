pub mod camera_movement;

use juno::{
    ivec,
    vector::{IVec2, Vector},
};
use rust_raylib::{MouseButton, Raylib};

use crate::sector::Sector;

use self::camera_movement::move_camera;

pub struct Selection {
    selected_tile: Option<IVec2>,
    camera_position: IVec2,
    scale: f32,
    vim_bindings: bool,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            selected_tile: None,
            camera_position: ivec!(0, 0),
            scale: 1.,
            vim_bindings: false,
        }
    }
}

impl Selection {
    pub fn camera_position(&self) -> &IVec2 {
        &self.camera_position
    }

    pub fn camera_position_mut(&mut self) -> &mut IVec2 {
        &mut self.camera_position
    }
}

pub fn handle_inputs(raylib: &Raylib, sector: &Sector, selection: &mut Selection) {
    let edge = i32::min(1200 / sector.width(), 720 / sector.height());
    // Mouse
    if raylib.is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = IVec2::new(raylib.get_mouse_x(), raylib.get_mouse_y());

        // In bounds
        let size = sector.size().clone();
        if mouse_pos.x() < &(size.x() * edge) && mouse_pos.y() < &(size.y() * edge) {
            let pos = mouse_pos / edge;
            selection.selected_tile = match &selection.selected_tile {
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
    if raylib.is_mouse_button_pressed(MouseButton::Right) {
        let mouse_delta_rl = raylib.get_mouse_delta();
        let mouse_delta = ivec!(mouse_delta_rl.x as i32, mouse_delta_rl.y as i32);
        move_camera(selection, mouse_delta);
    }
}
