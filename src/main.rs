// Prevailing note
// Take screenshots of bugs for a "bug montage" to What Is Love - Haddaway (https://youtube.com/watch?v=SxQdbtjGEsc)

pub mod assets;
pub mod interaction;
pub mod sector;
pub mod terrain;
pub mod utility;

use assets::TextureStore;
use interaction::{handle_inputs, Selection};
use log::{info, warn, LevelFilter};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Vector2,
    texture::Texture2D,
};
use sector::{Sector, Unit};
use simplelog::{ColorChoice, Config, TermLogger};
use terrain::TileType;
use utility::GridPosVec;

use crate::{assets::load_assets, terrain::generate_terrain};

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        simplelog::TerminalMode::Stdout,
        ColorChoice::Always,
    )
    .unwrap();

    info!("Starting Raylib");
    let (mut rl, thread) = raylib::init().size(1200, 720).title("WIP Game").build();
    let asset_store = load_assets(&mut rl, &thread);
    info!("Assets loaded");

    let mut sector = generate_terrain("Hello world sector".to_string(), GridPosVec::new(16, 16));
    let mut unit = Unit::new(GridPosVec::new(3, 6), Color::GOLD, 4., &sector).unwrap();
    sector.add_unit(unit);

    info!("Sector created and unit added");

    let mut selection = Selection::new();

    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        // Render tiles
        render(d, &sector, &selection, &asset_store);
        handle_inputs(&mut rl, &sector, &mut selection);
    }
}

fn render(mut d: RaylibDrawHandle, sector: &Sector, sel: &Selection, asset_store: &TextureStore) {
    let edge = usize::min(1200 / sector.width(), 720 / sector.height());
    for tile in sector.tiles() {
        let tag = match tile.tile_type() {
            TileType::GrassNone => "grass-none",
            TileType::GrassVar1 => "grass-var1",
            TileType::GrassVar2 => "grass-var2",
            TileType::PathBottom => "path-bottom",
            TileType::PathBottomRight => "path-bottom,right",
            TileType::PathBottomLeftRight => "path-bottom,left,right",
            TileType::PathTop => "path-top",
        };
        let origin = GridPosVec::new(edge * tile.pos().x(), edge * tile.pos().y());
        let size = GridPosVec::new(edge, edge);
        asset_store.render(tag.to_string(), origin, size, &mut d);
    }

    for u in sector.units() {
        let x = (u.pos().x() * edge) + (edge / 2);
        let y = (u.pos().y() * edge) + (edge / 2);
        d.draw_circle(x as i32, y as i32, edge as f32 * 0.4, u.color());
    }

    // Render selection
    // if let Some(pos) = sel.tile() {
    //     let sel_texture = asset_store.get("selector-icon").unwrap().texture();
    //     let pos = *pos;
    //     d.draw_texture_ex(
    //         sel_texture,
    //         Vector2::new((pos.x() * edge) as f32, (pos.y() * edge) as f32),
    //         0.,
    //         3.,
    //         Color::WHITE,
    //     );
    //     if let Some(unit) = sector.unit_at_tile(pos) {
    //         // if !unit.has_nav() {
    //         //     warn!("Attempting to render unit with no nav");
    //         // }

    //         // for x in 0..sector.width() {
    //         //     for y in 0..sector.height() {
    //         //         if let Some(nav) = unit.get_nav() {
    //         //             if let Some(dist) = nav.tile(x, y) {
    //         //                 if dist <= unit.movement() {
    //         //                     d.draw_rectangle(
    //         //                         (x * edge) as i32,
    //         //                         (y * edge) as i32,
    //         //                         edge as i32,
    //         //                         edge as i32,
    //         //                         Color::new(0, 0, 0, 128)
    //         //                     );
    //         //                 }
    //         //             }
    //         //         }
    //         //     }
    //         // }
    //     }
    // }

    d.clear_background(Color::WHITE);
    d.draw_text(&sector.name(), 12, 12, 20, Color::BLUE);
}
