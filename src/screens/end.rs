use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use crate::engine::match_rules::MatchOutcome;
use crate::state::{game_data::GameData, GameAction, ScreenCommand};

pub struct EndScreen;

impl EndScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _data: &GameData) -> ScreenCommand {
        if button(40.0, 120.0, 200.0, 44.0, "Back to Setup") {
            return ScreenCommand::Action(GameAction::ResetMatch);
        }
        ScreenCommand::None
    }

    pub fn draw(&self, data: &GameData) {
        draw_text("End", 40.0, 60.0, 32.0, WHITE);
        let outcome = match data.last_outcome {
            MatchOutcome::WinnerLeft => "Left Wins",
            MatchOutcome::WinnerRight => "Right Wins",
            MatchOutcome::SuddenDeath => "Sudden Death",
            MatchOutcome::Ongoing => "Ongoing",
        };
        draw_text(outcome, 40.0, 100.0, 22.0, LIGHTGRAY);
        draw_text(
            &format!(
                "Final stability - Left: {}  Right: {}",
                data.left_universe.stability, data.right_universe.stability
            ),
            40.0,
            130.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Season: {}", data.season),
            40.0,
            160.0,
            20.0,
            LIGHTGRAY,
        );
    }
}
