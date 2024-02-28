// Prevailing note
// Take screenshots of bugs for a "bug montage" to What Is Love - Haddaway (https://youtube.com/watch?v=SxQdbtjGEsc)
pub mod interaction;
pub mod juno;
pub mod sector;
pub mod terrain;
pub mod terrain_old;

use crate::juno::{renderer::quad::TexturedQuad, JunoApp};
use cgmath::Vector2;
use interaction::{handle_inputs, GameData};
use juno::renderer::{renderer::Renderer, testing::TextureAtlasHandle};
use simplelog::TermLogger;

fn main() {
    TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Always,
    )
    .unwrap();
    let sector =
        terrain::generate_terrain(Vector2::new(72, 72), "New terrain test sector".to_string());
    let screen_size = Vector2::new(1280, 720);
    let mut game_data = GameData::new_default(screen_size, sector);
    let mut app = JunoApp::new(screen_size.x, screen_size.y);
    let punyworld = app
        .load_texture_atlas("punyworld-overworld-tileset.png", Vector2::new(16, 16))
        .unwrap();
    let overlays = app
        .load_texture_atlas("overlays.png", Vector2::new(16, 16))
        .unwrap();

    while app.update() {
        handle_inputs(&mut game_data, app.input_state());
        render(app.renderer_mut(), &game_data, &punyworld, &overlays);
        app.render();
    }
}

fn render(
    renderer: &mut Renderer,
    game_data: &GameData,
    punyworld: &TextureAtlasHandle,
    overlays: &TextureAtlasHandle,
) {
    let edge_len = game_data.tile_edge_len();
    for tile in game_data.sector().tiles() {
        renderer.submit_textured_quad(TexturedQuad::new(
            Vector2::new(
                (tile.pos().x * edge_len) as i32,
                (tile.pos().y * edge_len) as i32,
            ),
            Vector2::new(edge_len as i32, edge_len as i32),
            punyworld.texture(
                tile.contents().atlas_position().x,
                tile.contents().atlas_position().y,
            ),
        ));
    }

    if let Some(sel) = game_data.selected_tile() {
        renderer.submit_textured_quad(TexturedQuad::new(
            Vector2::new((sel.x * edge_len) as i32, (sel.y * edge_len) as i32),
            Vector2::new(edge_len as i32, edge_len as i32),
            overlays.texture(0, 0),
        ));
    }
}
