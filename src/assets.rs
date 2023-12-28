use std::collections::HashMap;

use juno::vector::{IVec2, Vector};
use rust_raylib::{
    drawing::{Draw, DrawHandle, DrawTextureParams},
    math::{Rectangle, Vector2},
    texture::{RenderTexture, Texture},
    Raylib,
};

use crate::interaction::camera_position::{self, CameraPosition};

pub fn load_assets() -> TextureStore {
    let mut textures = TextureStore::new();
    textures.load_atlas(
        "punyworld-tileset",
        "assets/punyworld-overworld-tileset.png",
    );
    let tile_size = 16;
    return textures;
}

pub struct TextureStore {
    textures: HashMap<String, Texture>,
    target: RenderTexture,
}

impl TextureStore {
    fn new() -> Self {
        TextureStore {
            textures: HashMap::new(),
            target: RenderTexture::new(1200, 720).unwrap(),
        }
    }

    pub fn target(&self) -> &RenderTexture {
        &self.target
    }

    fn load_atlas(&mut self, name: &str, path: &str) {
        let texture = Texture::from_file(path).unwrap();
        self.textures.insert(name.to_string(), texture);
    }

    pub fn draw_tile(&self, source: &IVec2, origin: IVec2, size: IVec2, draw: &mut DrawHandle) {
        let tileset = self.textures.get("punyworld-tileset").unwrap();
        draw.draw_texture(
            tileset,
            Vector2 {
                x: *origin.x() as f32,
                y: *origin.y() as f32,
            },
            DrawTextureParams {
                source: Some(Rectangle::new(
                    *source.x() as f32,
                    *source.y() as f32,
                    16.,
                    16.,
                )),
                scale: Vector2 {
                    x: *size.x() as f32 / 16.,
                    y: *size.y() as f32 / 16.,
                },
                ..Default::default()
            },
        );
    }
}
