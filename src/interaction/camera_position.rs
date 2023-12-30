use juno::{ivec, vector::IVec2, vector::Vector};
use log::debug;

pub struct CameraPosition {
    position: IVec2,
    scale: f32,
    pan_sensitivity: f32,
    zoom_sensitivity: f32,
}

impl Default for CameraPosition {
    fn default() -> Self {
        Self {
            position: ivec!(0, 0),
            scale: 1.,
            pan_sensitivity: 1.,
            zoom_sensitivity: 1.,
        }
    }
}

impl CameraPosition {
    pub fn move_camera(
        &mut self,
        mouse_delta: IVec2,
        sector_px_dim: &IVec2,
        screen_dimensions: &IVec2,
    ) {
        let min_screen_dim = i32::min(*screen_dimensions.x(), *screen_dimensions.y());
        let min_x = 0.;
        let max_x = ((sector_px_dim.x() - min_screen_dim) as f32).max(0.);
        let min_y = 0.;
        let max_y = ((sector_px_dim.y() - min_screen_dim) as f32).max(0.);
        let new_position_x = (-*mouse_delta.x() as f32 * self.pan_sensitivity
            + *self.position().x() as f32)
            .clamp(min_x, max_x);
        let new_position_y = (-*mouse_delta.y() as f32 * self.pan_sensitivity
            + *self.position().y() as f32)
            .clamp(min_y, max_y);
        self.position = ivec!(new_position_x as i32, new_position_y as i32);
    }

    pub fn zoom_camera(&mut self, mouse_wheel_delta: f32) {
        let scale_delta = mouse_wheel_delta * self.zoom_sensitivity;
        let new_scale_raw = self.scale + scale_delta;
        let new_scale = new_scale_raw.clamp(1., 4.);
        self.scale = new_scale;
        // Adjust position after scaling to ensure a smooth experience. There are two seperate
        // behaviours that we want here:
        //  - Zoom in -> adjust to ensure that the cursor is hovering over the same position
        //  - Zoom out -> adjust to center the previous position
        // When zooming out by a factor of 1, the zoomed in area halves in size. Since position is
        // based off the top right corner, we want our previous camera position to be at (0.25 *
        // width, 0.25 * height).
        // TODO: That said, this is a *later* problem
    }

    pub fn position(&self) -> &IVec2 {
        &self.position
    }

    pub fn scale(&self) -> &f32 {
        &self.scale
    }
}
