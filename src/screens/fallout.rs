use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use crate::state::{game_data::GameData, GameAction, ScreenCommand};
use macroquad_toolkit::ui::draw_ui_text;

pub struct FalloutScreen;

impl FalloutScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _data: &GameData) -> ScreenCommand {
        if button(40.0, 120.0, 220.0, 44.0, "Apply Fallout") {
            return ScreenCommand::Action(GameAction::ApplyFallout);
        }
        ScreenCommand::None
    }

    pub fn draw(&self, data: &GameData) {
        draw_ui_text("Fallout", 40.0, 60.0, 32.0, WHITE);
        draw_ui_text(
            &format!(
                "Left stability: {}  Right stability: {}",
                data.left_universe.stability, data.right_universe.stability
            ),
            40.0,
            100.0,
            20.0,
            LIGHTGRAY,
        );
    }
}
