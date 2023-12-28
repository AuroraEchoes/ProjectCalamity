// Prevailing note
// Take screenshots of bugs for a "bug montage" to What Is Love - Haddaway (https://youtube.com/watch?v=SxQdbtjGEsc)

pub mod assets;
pub mod interaction;
pub mod sector;
pub mod terrain;

use assets::TextureStore;
use interaction::{handle_inputs, Selection};
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
use sector::Sector;
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

    info!("Starting Raylib");
    let mut raylib = Raylib::init_window(1200, 720, "Project Calamity").unwrap();
    let asset_store = load_assets();
    info!("Assets loaded");

    let mut selection = Selection::default();
    let sector = test_gen("Test Sector".to_string(), ivec!(48, 48));

    while !raylib.window_should_close() {
        handle_inputs(&raylib, &sector, &mut selection);
        let d = raylib.begin_drawing();
        // Render tiles
        render(d, &sector, &selection, &asset_store);
    }
}

fn render(mut d: DrawHandle, sector: &Sector, sel: &Selection, asset_store: &TextureStore) {
    d.begin_texture_mode(asset_store.target());
    let edge = i32::min(1200 / sector.width(), 720 / sector.height());
    let tile_texture_origin = sel.camera_position();
    for tile in sector.tiles() {
        let origin = IVec2::new(edge * tile.pos().x(), edge * tile.pos().y());
        let size = IVec2::new(edge, edge);
        let source_pos = tile.contents().atlas_position().clone() * 16;
        asset_store.draw_tile(
            &source_pos,
            origin,
            tile_texture_origin.clone(),
            size,
            &mut d,
        );
    }

    d.draw_text(sector.name(), Vector2 { x: 12., y: 12. }, 20, Color::GOLD);
    d.draw_fps(Vector2 { x: 12., y: 120. });
    d.clear_background(Color::WHITE);
    d.end_drawing();
}
