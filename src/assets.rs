use std::collections::HashMap;

use log::warn;
use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub fn load_assets(mut rl: &mut RaylibHandle, thread: &RaylibThread) -> AssetStore {
    let mut asset_store = AssetStore::new();
    asset_store.add(
        "selector-icon",
        Asset::load_texture("selector.png", rl, thread).unwrap(),
    );
    asset_store.add(
        "tile-ground",
        Asset::load_texture("tile-ground.png", rl, thread).unwrap(),
    );
    asset_store.add(
        "tile-water",
        Asset::load_texture("tile-water.png", rl, thread).unwrap(),
    );
    return asset_store;
}

pub struct AssetStore {
    assets: HashMap<String, Asset>,
}

impl AssetStore {
    fn new() -> Self {
        return AssetStore {
            assets: HashMap::new(),
        };
    }

    fn add(&mut self, tag: &str, asset: Asset) {
        self.assets.insert(tag.to_string(), asset);
    }

    pub fn get(&self, tag: &str) -> Option<&Asset> {
        let asset = self.assets.get(tag);
        if let None = asset {
            warn!("Attempted to load non-existent asset \"{tag}\"")
        };
        return asset;
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
