use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use crate::data::types::GodRole;
use crate::state::{
    game_data::{GameData, MatchMode},
    GameAction, ScreenCommand, StateTransition,
};

pub struct MatchSetupScreen;

impl MatchSetupScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _data: &GameData) -> ScreenCommand {
        if button(40.0, 120.0, 140.0, 36.0, "PVP Mode") {
            return ScreenCommand::Action(GameAction::SetMode(MatchMode::Pvp));
        }
        if button(200.0, 120.0, 140.0, 36.0, "VS AI") {
            return ScreenCommand::Action(GameAction::SetMode(MatchMode::VsAi));
        }
        if button(40.0, 170.0, 200.0, 44.0, "Start Match") {
            return ScreenCommand::ActionAndTransition(
                GameAction::StartMatch,
                StateTransition::ToCultivation,
            );
        }
        ScreenCommand::None
    }

    pub fn draw(&self, data: &GameData) {
        draw_text("Match Setup", 40.0, 60.0, 32.0, WHITE);
        draw_text(
            &format!("Season: {}", data.season),
            40.0,
            90.0,
            20.0,
            LIGHTGRAY,
        );
        let mode_text = match data.mode {
            MatchMode::Pvp => "Mode: PVP",
            MatchMode::VsAi => "Mode: VS AI",
        };
        draw_text(mode_text, 40.0, 110.0, 20.0, LIGHTGRAY);
        draw_text("Roster (P1)", 40.0, 160.0, 24.0, WHITE);

        let mut y = 190.0;
        for god in &data.left_gods {
            let role = role_label(god.role);
            draw_text(
                &format!("{} - {} ({})", god.name, role, god.domain),
                40.0,
                y,
                20.0,
                LIGHTGRAY,
            );
            y += 24.0;
        }

        draw_text("Roster (P2)", 380.0, 160.0, 24.0, WHITE);
        let mut y = 190.0;
        for god in &data.right_gods {
            let role = role_label(god.role);
            draw_text(
                &format!("{} - {} ({})", god.name, role, god.domain),
                380.0,
                y,
                20.0,
                LIGHTGRAY,
            );
            y += 24.0;
        }
    }
}

fn role_label(role: GodRole) -> &'static str {
    match role {
        GodRole::Tank => "Tank",
        GodRole::Striker => "Striker",
        GodRole::Control => "Control",
        GodRole::Support => "Support",
    }
}
