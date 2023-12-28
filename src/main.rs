// Prevailing note
// Take screenshots of bugs for a "bug montage" to What Is Love - Haddaway (https://youtube.com/watch?v=SxQdbtjGEsc)

pub mod assets;
pub mod interaction;
pub mod sector;
pub mod terrain;

use assets::TextureStore;
use interaction::{handle_inputs, GameData};
use juno::{
    ivec,
    vector::{IVec2, Vector},
};
use log::{info, LevelFilter};
use rust_raylib::{
    color::Color,
    drawing::{Draw, DrawHandle},
    math::Vector2,
    Raylib,
};
use simplelog::{ColorChoice, Config, TermLogger};

use crate::{assets::load_assets, terrain::test_gen};

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        simplelog::TerminalMode::Stdout,
        ColorChoice::Always,
    )
    .unwrap();
    let sector = test_gen("Test Sector".to_string(), ivec!(48, 48));
    let mut game_data = GameData::new_default(ivec!(1200, 720), sector);

    info!("Starting Raylib");
    let mut raylib = Raylib::init_window(1920, 1080, "Project Calamity").unwrap();
    let asset_store = load_assets();
    info!("Assets loaded");

    while !raylib.window_should_close() {
        handle_inputs(&raylib, &mut game_data);
        let d = raylib.begin_drawing();
        // Render tiles
        render(d, &game_data, &asset_store);
    }
}

fn render(mut d: DrawHandle, data: &GameData, asset_store: &TextureStore) {
    d.begin_texture_mode(asset_store.target());
    let edge = data.tile_edge_len();
    let offset = data.camera_position().position();
    let scale = data.camera_position().scale();
    for tile in data.sector().tiles() {
        let origin = IVec2::new(
            ((edge * tile.pos().x() - offset.x()) as f32 * scale) as i32,
            ((edge * tile.pos().y() - offset.y()) as f32 * scale) as i32,
        );
        let size = IVec2::new((edge as f32 * scale) as i32, (edge as f32 * scale) as i32);
        let source_pos = tile.contents().atlas_position().clone() * 16;
        asset_store.draw_tile(&source_pos, origin, size, &mut d);
    }

    d.draw_text(
        data.sector().name(),
        Vector2 { x: 12., y: 12. },
        20,
        Color::GOLD,
    );
    d.draw_fps(Vector2 { x: 12., y: 120. });
    d.clear_background(Color::WHITE);
    d.end_drawing();
}
