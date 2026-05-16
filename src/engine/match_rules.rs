//! Win conditions and match resolution.

use crate::data::types::{GodStats, UniverseStats};
use crate::engine::combat::simulate_clash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchOutcome {
    Ongoing,
    WinnerLeft,
    WinnerRight,
    SuddenDeath,
}

pub fn universe_collapsed(universe: &UniverseStats) -> bool {
    universe.stability <= 0
}

pub fn evaluate_match_end(
    season_index: u8,
    left: &UniverseStats,
    right: &UniverseStats,
) -> MatchOutcome {
    if universe_collapsed(left) && universe_collapsed(right) {
        return MatchOutcome::SuddenDeath;
    }
    if universe_collapsed(left) {
        return MatchOutcome::WinnerRight;
    }
    if universe_collapsed(right) {
        return MatchOutcome::WinnerLeft;
    }

    if season_index < 3 {
        return MatchOutcome::Ongoing;
    }

    if left.stability > right.stability {
        MatchOutcome::WinnerLeft
    } else if right.stability > left.stability {
        MatchOutcome::WinnerRight
    } else {
        MatchOutcome::SuddenDeath
    }
}

pub fn resolve_sudden_death(left: &[GodStats], right: &[GodStats]) -> MatchOutcome {
    let outcome = simulate_clash(left, right);
    let left_alive = outcome.left.iter().filter(|c| c.hp > 0).count();
    let right_alive = outcome.right.iter().filter(|c| c.hp > 0).count();

    if left_alive > right_alive {
        MatchOutcome::WinnerLeft
    } else if right_alive > left_alive {
        MatchOutcome::WinnerRight
    } else {
        MatchOutcome::SuddenDeath
    }
}
