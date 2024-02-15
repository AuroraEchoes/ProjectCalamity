pub mod camera_position;

use std::collections::HashMap;

use cgmath::Vector2;
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::{sector::Sector, juno::InputState};

use self::camera_position::CameraPosition;

pub struct GameData {
    camera_position: CameraPosition,
    screen_size: Vector2<u32>,
    loaded_sector: Sector,
    selected_tile: Option<Vector2<u32>>,
}

impl GameData {
    pub fn new_default(screen_size: Vector2<u32>, loaded_sector: Sector) -> Self {
        Self {
            camera_position: CameraPosition::default(),
            screen_size,
            loaded_sector,
            selected_tile: None,
        }
    }

    pub fn camera_position(&self) -> &CameraPosition {
        &self.camera_position
    }

    pub fn camera_position_mut(&mut self) -> &mut CameraPosition {
        &mut self.camera_position
    }

    pub fn screen_size(&self) -> Vector2<u32> {
        self.screen_size
    }

    pub fn sector(&self) -> &Sector {
        &self.loaded_sector
    }

    pub fn selected_tile(&self) -> &Option<Vector2<u32>> {
        &self.selected_tile
    }

    pub fn selected_tile_mut(&mut self) -> &mut Option<Vector2<u32>> {
        &mut self.selected_tile
    }

    pub fn tile_edge_len(&self) -> u32 {
        u32::min(
            ((self.screen_size().x / self.sector().width()) as f32 * self.camera_position().scale())
                as u32,
            ((self.screen_size().y / self.sector().height()) as f32
                * self.camera_position().scale()) as u32,
        )
    }
}

pub fn handle_inputs(data: &GameData, inputs: InputState) {

}

#[derive(Debug)]
pub enum ButtonInput {
    Key(KeyCode),
    Mouse(MouseButton),
}

pub enum InputAction {
    SelectAdjacentTile(Vector2<i32>),
    SelectSpecificTile(Vector2<u32>),
    PanScreen(Vector2<f32>),
    ChangeZoom(f32),
}

pub struct KeyMap {
    keys: HashMap<KeyCode, InputAction>
}

impl KeyMap {
    pub fn actions(&self, inputs: &InputState) -> Vec<InputAction> {

    }
}
