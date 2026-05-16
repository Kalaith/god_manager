//! Main game struct and loop coordination.

use macroquad::prelude::*;

use crate::state::{game_data::GameData, AppState, GameAction, ScreenCommand, StateTransition};

pub struct Game {
    pub state: AppState,
    pub data: GameData,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            data: GameData::new(),
        }
    }

    pub fn update(&mut self) {
        let command = match &mut self.state {
            AppState::MatchSetup(screen) => screen.update(&self.data),
            AppState::Cultivation(screen) => screen.update(&self.data),
            AppState::Clash(screen) => screen.update(&self.data),
            AppState::Fallout(screen) => screen.update(&self.data),
            AppState::End(screen) => screen.update(&self.data),
        };

        self.handle_command(command);
    }

    pub fn draw(&self) {
        match &self.state {
            AppState::MatchSetup(screen) => screen.draw(&self.data),
            AppState::Cultivation(screen) => screen.draw(&self.data),
            AppState::Clash(screen) => screen.draw(&self.data),
            AppState::Fallout(screen) => screen.draw(&self.data),
            AppState::End(screen) => screen.draw(&self.data),
        }
    }

    fn apply_transition(&mut self, transition: StateTransition) {
        self.state = match transition {
            StateTransition::ToMatchSetup => {
                AppState::MatchSetup(crate::screens::match_setup::MatchSetupScreen::new())
            }
            StateTransition::ToCultivation => {
                AppState::Cultivation(crate::screens::cultivation::CultivationScreen::new())
            }
            StateTransition::ToClash => AppState::Clash(crate::screens::clash::ClashScreen::new()),
            StateTransition::ToFallout => {
                AppState::Fallout(crate::screens::fallout::FalloutScreen::new())
            }
            StateTransition::ToEnd => AppState::End(crate::screens::end::EndScreen::new()),
        };
    }

    fn handle_command(&mut self, command: ScreenCommand) {
        let transition = match command {
            ScreenCommand::None => None,
            ScreenCommand::Action(action) => self.apply_action(action),
            ScreenCommand::ActionAndTransition(action, transition) => {
                let follow_up = self.apply_action(action);
                if follow_up.is_some() {
                    follow_up
                } else {
                    Some(transition)
                }
            }
        };

        if let Some(transition) = transition {
            self.apply_transition(transition);
        }
    }

    fn apply_action(&mut self, action: GameAction) -> Option<StateTransition> {
        match action {
            GameAction::StartMatch => {
                self.data.reset();
                Some(StateTransition::ToCultivation)
            }
            GameAction::ApplyCultivation(allocation) => {
                let _ = self.data.apply_cultivation(allocation);
                Some(StateTransition::ToClash)
            }
            GameAction::SetMode(mode) => {
                self.data.set_mode(mode);
                None
            }
            GameAction::RunClash(left_indices, right_indices) => {
                self.data
                    .run_clash_with_selection(&left_indices, &right_indices);
                Some(StateTransition::ToFallout)
            }
            GameAction::ApplyFallout => {
                self.data.apply_fallout();
                match self.data.last_outcome {
                    crate::engine::match_rules::MatchOutcome::Ongoing => {
                        Some(StateTransition::ToCultivation)
                    }
                    _ => Some(StateTransition::ToEnd),
                }
            }
            GameAction::ResetMatch => {
                self.data.reset();
                Some(StateTransition::ToMatchSetup)
            }
        }
    }
}

pub fn clear_frame() {
    clear_background(Color::from_rgba(15, 15, 18, 255));
}
