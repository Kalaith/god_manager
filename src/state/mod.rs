//! Application state machine.

use crate::screens::{
    clash::ClashScreen, cultivation::CultivationScreen, end::EndScreen, fallout::FalloutScreen,
    match_setup::MatchSetupScreen,
};

pub mod game_data;

pub enum AppState {
    MatchSetup(MatchSetupScreen),
    Cultivation(CultivationScreen),
    Clash(ClashScreen),
    Fallout(FalloutScreen),
    End(EndScreen),
}

pub enum ScreenCommand {
    None,
    Action(GameAction),
    ActionAndTransition(GameAction, StateTransition),
}

pub enum GameAction {
    StartMatch,
    SetMode(crate::state::game_data::MatchMode),
    ApplyCultivation(crate::data::types::CultivationAllocation),
    RunClash(Vec<usize>, Vec<usize>),
    ApplyFallout,
    ResetMatch,
}

pub enum StateTransition {
    ToMatchSetup,
    ToCultivation,
    ToClash,
    ToFallout,
    ToEnd,
}

impl AppState {
    pub fn new() -> Self {
        Self::MatchSetup(MatchSetupScreen::new())
    }
}
