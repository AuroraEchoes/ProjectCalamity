// Prevailing note
// Take screenshots of bugs for a "bug montage" to What Is Love - Haddaway (https://youtube.com/watch?v=SxQdbtjGEsc)

pub mod interaction;
pub mod sector;
pub mod utility;

use interaction::{handle_inputs, Selection};
use log::{info, warn, LevelFilter};
use raylib::{drawing::{RaylibDraw, RaylibDrawHandle}, color::Color, texture::Texture2D, math::Vector2};
use sector::{Sector, Unit};
use simplelog::{TermLogger, Config, ColorChoice};
use utility::GridPosVec;

fn main() {
    TermLogger::init(
        LevelFilter::Trace, 
        Config::default(), 
        simplelog::TerminalMode::Stdout, 
        ColorChoice::Always
    ).unwrap();

    info!("Starting Raylib");
    let (mut rl, thread) = raylib::init()
        .size(1200, 720)
        .title("WIP Game")
        .build();

    let selection_texture = rl.load_texture(&thread, "assets/selector.png").unwrap();

    let mut sector = Sector::random("Test sector", 16, 16);
    sector.generate_navs();
    sector.add_unit(Unit::new(GridPosVec::new(5, 9), Color::GOLD, 2.));

    let mut selection = Selection::new();

    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        
        // Render tiles
        render(d, &sector, &selection, &selection_texture);
        handle_inputs(&mut rl, &sector, &mut selection);
    }
}

fn render(mut d: RaylibDrawHandle, sector: &Sector, sel: &Selection, sel_texture: &Texture2D) {
    let edge = usize::min(1200 / sector.width(), 720 / sector.height());
    for x in 0..sector.width() {
        for y in 0..sector.height() {
            d.draw_rectangle((x * edge) as i32, (y * edge) as i32, edge as i32, edge as i32, sector.tile(x, y).unwrap().color());
        }
    }

    for u in sector.units() {
        let x = (u.pos().x() * edge) + (edge / 2);
        let y = (u.pos().y() * edge) + (edge / 2);
        d.draw_circle(x as i32, y as i32, edge as f32 * 0.4, u.color());
    }

    // Render selection
    if let Some(pos) = sel.tile() {
        let pos = *pos;
        d.draw_texture_ex(sel_texture, Vector2::new((pos.x() * edge) as f32, (pos.y() * edge) as f32), 0., 3., Color::WHITE);
        if let Some(unit) = sector.unit_at_tile(pos) {
            // if !unit.has_nav() {
            //     warn!("Attempting to render unit with no nav");
            // }



            // for x in 0..sector.width() {
            //     for y in 0..sector.height() {
            //         if let Some(nav) = unit.get_nav() {
            //             if let Some(dist) = nav.tile(x, y) {
            //                 if dist <= unit.movement() {
            //                     d.draw_rectangle(
            //                         (x * edge) as i32, 
            //                         (y * edge) as i32, 
            //                         edge as i32, 
            //                         edge as i32, 
            //                         Color::new(0, 0, 0, 128)
            //                     );
            //                 }
            //             }
            //         }
            //     }
            // }
        }
    }

    d.clear_background(Color::WHITE);
    d.draw_text(&sector.name(), 12, 12, 20, Color::BLUE);
}