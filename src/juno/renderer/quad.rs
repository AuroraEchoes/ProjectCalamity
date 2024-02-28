use cgmath::Vector2;

use crate::juno::Vertex;

use super::testing::TextureSection;

#[derive(Debug)]
pub struct TexturedQuad {
    position: Vector2<i32>,
    size: Vector2<i32>,
    texture_section: TextureSection,
}

impl TexturedQuad {
    pub fn new(
        position: Vector2<i32>,
        size: Vector2<i32>,
        texture_section: TextureSection,
    ) -> Self {
        Self {
            position,
            size,
            texture_section,
        }
    }

    pub fn verticies(&self, window_dimensions: &winit::dpi::PhysicalSize<u32>) -> [Vertex; 4] {
        let v0 = Vertex::new(
            screen_to_clip(&self.position, window_dimensions),
            [
                self.texture_section.x(),
                self.texture_section.y() + self.texture_section.height(),
            ],
        );
        let v1 = Vertex::new(
            screen_to_clip(
                &(self.position + Vector2::new(self.size.x, 0)),
                window_dimensions,
            ),
            [
                self.texture_section.x() + self.texture_section.width(),
                self.texture_section.y() + self.texture_section.height(),
            ],
        );
        let v2 = Vertex::new(
            screen_to_clip(&(self.position + self.size), window_dimensions),
            [
                self.texture_section.x() + self.texture_section.width(),
                self.texture_section.y(),
            ],
        );
        let v3 = Vertex::new(
            screen_to_clip(
                &(self.position + Vector2::new(0, self.size.y)),
                window_dimensions,
            ),
            [self.texture_section.x(), self.texture_section.y()],
        );
        [v0, v1, v2, v3]
    }

    pub fn source(&self) -> u32 {
        self.texture_section.source()
    }
}

fn screen_to_clip(screen: &Vector2<i32>, size: &winit::dpi::PhysicalSize<u32>) -> [f32; 3] {
    let half_width = size.width as i32 / 2;
    let half_height = size.height as i32 / 2;
    let centered_x = screen.x - half_width;
    let centered_y = screen.y - half_height;
    let screen_pos = [
        centered_x as f32 / half_width as f32,
        centered_y as f32 / half_height as f32,
        0.,
    ];
    screen_pos
}
