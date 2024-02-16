pub mod camera_position;

use std::collections::HashMap;

use cgmath::Vector2;
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::{
    juno::{
        directions::{self, i32_u32_cast, u32_i32_subtract},
        InputState,
    },
    sector::Sector,
};

use self::camera_position::CameraPosition;

pub struct GameData {
    camera_position: CameraPosition,
    screen_size: Vector2<u32>,
    loaded_sector: Sector,
    selected_tile: Option<Vector2<u32>>,
    key_map: KeyMap,
}

impl GameData {
    pub fn new_default(screen_size: Vector2<u32>, loaded_sector: Sector) -> Self {
        Self {
            camera_position: CameraPosition::default(),
            screen_size,
            loaded_sector,
            selected_tile: None,
            key_map: KeyMap::default(),
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

pub fn handle_inputs(data: &mut GameData, inputs: &InputState) {
    let actions = data.key_map.actions(inputs);
    for action in actions {
        match action {
            InputAction::SelectAdjacentTile(dir) => {
                if let Some(prev_sel) = data.selected_tile {
                    if let Some(new_pos) = u32_i32_subtract(prev_sel, *dir) {
                        data.selected_tile = Some(new_pos);
                    }
                } else {
                    // If no tile is selected, select a corner based on the input
                    let selected_position = {
                        let x = match dir.x {
                            -1 => data.sector().width() as i32 - 1,
                            _ => dir.x,
                        };
                        let y = match dir.y {
                            -1 => data.sector().height() as i32 - 1,
                            _ => dir.y,
                        };
                        Vector2::new(x, y)
                    };
                    data.selected_tile = i32_u32_cast(selected_position);
                }
            }
            InputAction::SelectSpecificTile(_) => todo!(),
            InputAction::PanScreen(_) => todo!(),
            InputAction::ChangeZoom(_) => todo!(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ButtonInput {
    Key(KeyCode),
    Mouse(MouseButton),
}

#[derive(Debug)]
pub enum InputAction {
    SelectAdjacentTile(Vector2<i32>),
    SelectSpecificTile(Vector2<u32>),
    PanScreen(Vector2<f32>),
    ChangeZoom(f32),
}

pub struct KeyMap {
    keys: HashMap<ButtonInput, InputAction>,
}

impl KeyMap {
    pub fn actions(&self, inputs: &InputState) -> Vec<&InputAction> {
        // Buttons
        let actions = inputs
            .button_presses()
            .iter()
            .map(|input| self.keys.get(input))
            .filter_map(|map| map)
            .collect::<Vec<_>>();
        actions
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut keys = HashMap::new();
        keys.insert(
            ButtonInput::Key(KeyCode::KeyH),
            InputAction::SelectAdjacentTile(directions::LEFT),
        );
        keys.insert(
            ButtonInput::Key(KeyCode::KeyJ),
            InputAction::SelectAdjacentTile(directions::DOWN),
        );
        keys.insert(
            ButtonInput::Key(KeyCode::KeyK),
            InputAction::SelectAdjacentTile(directions::UP),
        );
        keys.insert(
            ButtonInput::Key(KeyCode::KeyL),
            InputAction::SelectAdjacentTile(directions::RIGHT),
        );
        Self { keys }
    }
}
