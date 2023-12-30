pub mod camera_position;

use juno::{
    ivec,
    vector::{IVec2, Vector},
};
use rust_raylib::{MouseButton, Raylib};

use crate::sector::Sector;

use self::camera_position::CameraPosition;

pub struct GameData {
    camera_position: CameraPosition,
    screen_size: IVec2,
    loaded_sector: Sector,
    selected_tile: Option<IVec2>,
    vim_bindings: bool,
}

impl GameData {
    pub fn new_default(screen_size: IVec2, loaded_sector: Sector) -> Self {
        Self {
            camera_position: CameraPosition::default(),
            screen_size,
            loaded_sector,
            selected_tile: None,
            vim_bindings: false,
        }
    }

    pub fn camera_position(&self) -> &CameraPosition {
        &self.camera_position
    }

    pub fn camera_position_mut(&mut self) -> &mut CameraPosition {
        &mut self.camera_position
    }

    pub fn screen_size(&self) -> &IVec2 {
        &self.screen_size
    }

    pub fn sector(&self) -> &Sector {
        &self.loaded_sector
    }

    pub fn selected_tile(&self) -> &Option<IVec2> {
        &self.selected_tile
    }

    pub fn selected_tile_mut(&mut self) -> &mut Option<IVec2> {
        &mut self.selected_tile
    }

    pub fn tile_edge_len(&self) -> i32 {
        i32::min(
            ((self.screen_size().x() / self.sector().width()) as f32
                * self.camera_position().scale()) as i32,
            ((self.screen_size().y() / self.sector().height()) as f32
                * self.camera_position().scale()) as i32,
        )
    }
}

pub fn handle_inputs(raylib: &Raylib, data: &mut GameData) {
    let edge_length = data.tile_edge_len();
    let sector_px_dim = ivec!(
        *data.sector().width() * edge_length,
        *data.sector().height() * edge_length
    );
    let screen_size = data.screen_size().clone();

    // Mouse
    if raylib.is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = IVec2::new(raylib.get_mouse_x(), raylib.get_mouse_y());

        // In bounds
        let size = data.sector().size().clone();
        let edge = data.tile_edge_len();
        if mouse_pos.x() < &(size.x() * edge) && mouse_pos.y() < &(size.y() * edge) {
            let pos = mouse_pos / edge;
            *data.selected_tile_mut() = match &data.selected_tile() {
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
    // Panning
    if raylib.is_mouse_button_down(MouseButton::Right) {
        let mouse_delta_rl = raylib.get_mouse_delta();
        let mouse_delta = ivec!(mouse_delta_rl.x as i32, mouse_delta_rl.y as i32);
        data.camera_position_mut()
            .move_camera(mouse_delta, &sector_px_dim, &screen_size);
    }
    // Zooming
    let mouse_wheel_delta = raylib.get_mouse_wheel_move_vec().y;
    if mouse_wheel_delta != 0. {
        data.camera_position_mut().zoom_camera(mouse_wheel_delta);
    }
}
