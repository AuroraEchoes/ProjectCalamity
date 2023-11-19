pub mod sector;
pub mod utility;

use raylib::{drawing::{RaylibDraw, RaylibDrawHandle}, color::Color, RaylibHandle, ffi::MouseButton};
use sector::{Sector, Unit};
use utility::USizeVec2;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1200, 720)
        .title("WIP Game")
        .build();

    let mut sector = Sector::random("Test sector", 16, 16);
    sector.add_unit(Unit::new(USizeVec2::new(5, 9), Color::GOLD));

    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        
        // Render tiles
        render(d, &sector);
        handle_inputs(&mut rl, &sector);
    }
}

fn render(mut d: RaylibDrawHandle, sector: &Sector) {
    let edge = usize::min(1200 / sector.width(), 720 / sector.height());
    for x in 0..sector.width() {
        for y in 0..sector.height() {
            d.draw_rectangle((x * edge) as i32, (y * edge) as i32, edge as i32, edge as i32, sector.tile(x, y).unwrap());
        }
    }

    for u in sector.units() {
        let x = (u.pos().x() * edge) + (edge / 2);
        let y = (u.pos().y() * edge) + (edge / 2);
        d.draw_circle(x as i32, y as i32, edge as f32 * 0.4, u.color());
    }
    d.clear_background(Color::WHITE);
    d.draw_text(&sector.name(), 12, 12, 20, Color::BLUE);
}

fn handle_inputs(rl: &mut RaylibHandle, sector: &Sector) {
    let edge = usize::min(1200 / sector.width(), 720 / sector.height());
    // Mouse
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        let mouse_pos = USizeVec2::new(rl.get_mouse_x() as usize, rl.get_mouse_y() as usize);

        // In bounds
        let size = *sector.size();
        if mouse_pos < (size * edge) {
            println!("pos {:?}", mouse_pos / edge);
        }
    }
}