use juno::vector::IVec2;

use super::Selection;

pub fn move_camera(selection: &mut Selection, mouse_delta: IVec2) {
    *selection.camera_position_mut() += mouse_delta;
}
