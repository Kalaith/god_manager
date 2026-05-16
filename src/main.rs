use macroquad::prelude::*;

mod data;
mod engine;
mod game;
mod screens;
mod state;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "God Manager".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = game::Game::new();

    loop {
        game::clear_frame();
        game.update();
        game.draw();
        next_frame().await;
    }
}
