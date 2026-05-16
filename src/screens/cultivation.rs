use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use crate::data::types::CultivationAllocation;
use crate::engine::rules::{CULTIVATION_MAX_PER_STAT, CULTIVATION_POINTS};
use crate::state::{game_data::GameData, GameAction, ScreenCommand};

pub struct CultivationScreen {
    allocation: CultivationAllocation,
}

impl CultivationScreen {
    pub fn new() -> Self {
        Self {
            allocation: CultivationAllocation {
                belief: 0,
                conflict: 0,
                innovation: 0,
            },
        }
    }

    pub fn update(&mut self, _data: &GameData) -> ScreenCommand {
        let mut changed = false;
        if button(40.0, 140.0, 28.0, 28.0, "+") {
            if self.allocation.belief < CULTIVATION_MAX_PER_STAT {
                self.allocation.belief += 1;
                changed = true;
            }
        }
        if button(80.0, 140.0, 28.0, 28.0, "-") {
            if self.allocation.belief > 0 {
                self.allocation.belief -= 1;
                changed = true;
            }
        }

        if button(40.0, 200.0, 28.0, 28.0, "+") {
            if self.allocation.conflict < CULTIVATION_MAX_PER_STAT {
                self.allocation.conflict += 1;
                changed = true;
            }
        }
        if button(80.0, 200.0, 28.0, 28.0, "-") {
            if self.allocation.conflict > 0 {
                self.allocation.conflict -= 1;
                changed = true;
            }
        }

        if button(40.0, 260.0, 28.0, 28.0, "+") {
            if self.allocation.innovation < CULTIVATION_MAX_PER_STAT {
                self.allocation.innovation += 1;
                changed = true;
            }
        }
        if button(80.0, 260.0, 28.0, 28.0, "-") {
            if self.allocation.innovation > 0 {
                self.allocation.innovation -= 1;
                changed = true;
            }
        }

        if changed && self.allocation.total_points() > CULTIVATION_POINTS {
            self.allocation = CultivationAllocation {
                belief: 0,
                conflict: 0,
                innovation: 0,
            };
        }

        let can_confirm = self.allocation.total_points() == CULTIVATION_POINTS;
        if can_confirm && button(40.0, 320.0, 200.0, 44.0, "Confirm") {
            let allocation = self.allocation;
            self.allocation = CultivationAllocation {
                belief: 0,
                conflict: 0,
                innovation: 0,
            };
            return ScreenCommand::Action(GameAction::ApplyCultivation(allocation));
        }

        ScreenCommand::None
    }

    pub fn draw(&self, data: &GameData) {
        draw_text("Cultivation", 40.0, 60.0, 32.0, WHITE);
        draw_text(
            &format!(
                "Belief: {} (+{})",
                data.left_universe.belief, self.allocation.belief
            ),
            120.0,
            160.0,
            22.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "Conflict: {} (+{})",
                data.left_universe.conflict, self.allocation.conflict
            ),
            120.0,
            220.0,
            22.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "Innovation: {} (+{})",
                data.left_universe.innovation, self.allocation.innovation
            ),
            120.0,
            280.0,
            22.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "Points: {}/{}",
                self.allocation.total_points(),
                CULTIVATION_POINTS
            ),
            40.0,
            360.0,
            20.0,
            LIGHTGRAY,
        );
    }
}
