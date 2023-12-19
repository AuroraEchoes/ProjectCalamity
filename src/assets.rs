use std::collections::HashMap;

use juno::{
    ivec,
    vector::{IVec2, Vector},
};
use log::warn;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

pub fn load_assets(mut rl: &mut RaylibHandle, thread: &RaylibThread) -> TextureStore {
    let mut textures = TextureStore::new();
    textures.load_atlas(
        "punyworld-tileset",
        "assets/punyworld-overworld-tileset.png",
        rl,
        thread,
    );
    textures.add_sprite(
        "grass-none",
        "punyworld-tileset",
        ivec!(0, 0),
        ivec!(16, 16),
    );
    textures.add_sprite(
        "grass-var1",
        "punyworld-tileset",
        ivec!(16, 0),
        ivec!(16, 16),
    );
    textures.add_sprite(
        "grass-var2",
        "punyworld-tileset",
        ivec!(32, 0),
        ivec!(16, 16),
    );
    textures.add_sprite(
        "path-bottom",
        "punyworld-tileset",
        ivec!(48, 0),
        ivec!(16, 16),
    );
    textures.add_sprite(
        "path-bottom,right",
        "punyworld-tileset",
        ivec!(64, 0),
        ivec!(16, 16),
    );
    textures.add_sprite(
        "path-bottom,left,right",
        "punyworld-tileset",
        ivec!(96, 0),
        ivec!(16, 16),
    );
    textures.add_sprite(
        "path-top",
        "punyworld-tileset",
        ivec!(48, 32),
        ivec!(16, 16),
    );

    return textures;
}

pub struct TextureStore {
    textures: HashMap<String, Texture2D>,
    tiles: HashMap<String, AtlasTile>,
}

impl TextureStore {
    fn new() -> Self {
        return TextureStore {
            textures: HashMap::new(),
            tiles: HashMap::new(),
        };
    }

    fn load_atlas(
        &mut self,
        name: &str,
        path: &str,
        mut rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        let texture = rl.load_texture(thread, path).unwrap();
        self.textures.insert(name.to_string(), texture);
    }

    fn add_sprite(&mut self, name: &str, parent: &str, origin: IVec2, size: IVec2) {
        self.tiles.insert(
            name.to_string(),
            AtlasTile::new(parent.to_string(), origin, size),
        );
    }

    pub fn render(&self, tag: String, origin: IVec2, size: IVec2, draw: &mut RaylibDrawHandle) {
        let sprite = self.tiles.get(&tag).unwrap();
        let texture = self.textures.get(&sprite.parent).unwrap();
        draw.draw_texture_pro(
            texture,
            Rectangle::new(
                *sprite.origin.x() as f32,
                *sprite.origin.y() as f32,
                *sprite.size.x() as f32,
                *sprite.size.y() as f32,
            ),
            Rectangle::new(
                *origin.x() as f32,
                *origin.y() as f32,
                *size.x() as f32,
                *size.y() as f32,
            ),
            Vector2::new(0., 0.),
            0.,
            Color::WHITE,
        )
    }
}

pub struct AtlasTile {
    // Name of parent texture
    parent: String,
    // Top left hand corner pos on parent texture
    origin: IVec2,
    // Texture size
    size: IVec2,
}

impl AtlasTile {
    pub fn new(parent: String, origin: IVec2, size: IVec2) -> Self {
        return Self {
            parent,
            origin,
            size,
        };
    }
}

pub struct Asset {
    texture: Texture2D,
}

impl Asset {
    fn load_texture(path: &str, mut rl: &mut RaylibHandle, thread: &RaylibThread) -> Option<Self> {
        if let Ok(texture) = rl.load_texture(thread, format!("assets/{path}").as_str()) {
            return Some(Asset { texture });
        } else {
            warn!("Could not load texture {path}");
        }
        return None;
    }

    pub fn texture(&self) -> &Texture2D {
        return &self.texture;
    }
}
