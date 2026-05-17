use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use crate::state::{
    game_data::{GameData, MatchMode},
    GameAction, ScreenCommand,
};

pub struct ClashScreen {
    selected: Vec<usize>,
    selected_right: Vec<usize>,
}

impl ClashScreen {
    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
            selected_right: Vec::new(),
        }
    }

    pub fn update(&mut self, data: &GameData) -> ScreenCommand {
        let mut y = 140.0;
        for (idx, god) in data.left_gods.iter().enumerate() {
            let label = if self.selected.contains(&idx) {
                format!("Deselect {}", god.name)
            } else {
                format!("Select {}", god.name)
            };
            if button(40.0, y, 220.0, 30.0, &label) {
                toggle_selection(&mut self.selected, idx);
                if self.selected.len() > 3 {
                    self.selected.pop();
                }
            }
            y += 36.0;
        }

        draw_text(
            &format!("Left selected: {}/3", self.selected.len()),
            40.0,
            120.0,
            18.0,
            LIGHTGRAY,
        );

        let right_selection = match data.mode {
            MatchMode::VsAi => data.top_three_indices(false),
            MatchMode::Pvp => {
                let mut y = 140.0;
                for (idx, god) in data.right_gods.iter().enumerate() {
                    let label = if self.selected_right.contains(&idx) {
                        format!("Deselect {}", god.name)
                    } else {
                        format!("Select {}", god.name)
                    };
                    if button(320.0, y, 220.0, 30.0, &label) {
                        toggle_selection(&mut self.selected_right, idx);
                        if self.selected_right.len() > 3 {
                            self.selected_right.pop();
                        }
                    }
                    y += 36.0;
                }
                self.selected_right.clone()
            }
        };

        let right_count = match data.mode {
            MatchMode::VsAi => right_selection.len(),
            MatchMode::Pvp => self.selected_right.len(),
        };
        let right_label = match data.mode {
            MatchMode::VsAi => format!("Right selected (AI): {}/3", right_count),
            MatchMode::Pvp => format!("Right selected: {}/3", right_count),
        };
        draw_text(&right_label, 320.0, 120.0, 18.0, LIGHTGRAY);

        let can_resolve = self.selected.len() == 3 && right_selection.len() == 3;
        if can_resolve && button(40.0, 320.0, 200.0, 44.0, "Resolve Clash") {
            let left_selection = self.selected.clone();
            self.selected.clear();
            self.selected_right.clear();
            return ScreenCommand::Action(GameAction::RunClash(left_selection, right_selection));
        }
        ScreenCommand::None
    }

    pub fn draw(&self, data: &GameData) {
        draw_text("Clash", 40.0, 60.0, 32.0, WHITE);
        draw_text("Select 3 gods", 40.0, 100.0, 20.0, LIGHTGRAY);
        draw_text("Selected:", 40.0, 360.0, 18.0, LIGHTGRAY);
        let mut y = 385.0;
        for idx in &self.selected {
            if let Some(god) = data.left_gods.get(*idx) {
                draw_text(&god.name, 40.0, y, 18.0, LIGHTGRAY);
                y += 20.0;
            }
        }

        if data.mode == MatchMode::Pvp {
            draw_text("Selected (P2):", 320.0, 360.0, 18.0, LIGHTGRAY);
            let mut y = 385.0;
            for idx in &self.selected_right {
                if let Some(god) = data.right_gods.get(*idx) {
                    draw_text(&god.name, 320.0, y, 18.0, LIGHTGRAY);
                    y += 20.0;
                }
            }
        }

        match data.mode {
            MatchMode::VsAi => {
                let right_indices = data.top_three_indices(false);
                draw_text("AI picks:", 320.0, 100.0, 20.0, LIGHTGRAY);
                let mut y = 130.0;
                for idx in right_indices {
                    if let Some(god) = data.right_gods.get(idx) {
                        draw_text(&god.name, 320.0, y, 20.0, LIGHTGRAY);
                        y += 24.0;
                    }
                }
            }
            MatchMode::Pvp => {
                draw_text("Player 2 selects 3", 320.0, 100.0, 20.0, LIGHTGRAY);
            }
        }

        if let Some(outcome) = &data.last_clash {
            draw_text(
                &format!("Last clash events: {}", outcome.log.len()),
                40.0,
                520.0,
                20.0,
                LIGHTGRAY,
            );
            if let Some(event) = outcome.log.last() {
                let side = if event.attacker_is_left {
                    "Left"
                } else {
                    "Right"
                };
                draw_text(
                    &format!(
                        "Last hit: R{} {} {} dmg (target {})",
                        event.round + 1,
                        side,
                        event.damage,
                        if event.target_defeated {
                            "defeated"
                        } else {
                            "alive"
                        }
                    ),
                    40.0,
                    545.0,
                    18.0,
                    LIGHTGRAY,
                );
            }
        }
    }
}

fn toggle_selection(selected: &mut Vec<usize>, idx: usize) {
    if let Some(pos) = selected.iter().position(|&value| value == idx) {
        selected.remove(pos);
    } else {
        selected.push(idx);
    }
}
