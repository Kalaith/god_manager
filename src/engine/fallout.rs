//! Fallout and stability rules.

use macroquad_toolkit::rng::choose_mut;

use crate::data::types::{God, TraitAlignment, UniverseStats};
use crate::engine::traits::apply_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FalloutEffect {
    DecreaseBelief,
    DecreaseConflict,
    DecreaseInnovation,
    RandomNegativeTrait,
    None,
    RandomPositiveTrait,
}

pub fn stability_loss(defeated_gods: i32) -> i32 {
    defeated_gods * 2
}

pub fn apply_stability_loss(universe: &UniverseStats, defeated_gods: i32) -> UniverseStats {
    UniverseStats {
        stability: universe.stability - stability_loss(defeated_gods),
        ..*universe
    }
}

pub fn fallout_effect(roll: u8) -> FalloutEffect {
    match roll {
        1 => FalloutEffect::DecreaseBelief,
        2 => FalloutEffect::DecreaseConflict,
        3 => FalloutEffect::DecreaseInnovation,
        4 => FalloutEffect::RandomNegativeTrait,
        5 => FalloutEffect::None,
        6 => FalloutEffect::RandomPositiveTrait,
        _ => FalloutEffect::None,
    }
}

pub fn apply_fallout(universe: &UniverseStats, roll: u8, gods: &mut [God]) -> UniverseStats {
    match fallout_effect(roll) {
        FalloutEffect::DecreaseBelief => UniverseStats {
            belief: universe.belief - 1,
            ..*universe
        },
        FalloutEffect::DecreaseConflict => UniverseStats {
            conflict: universe.conflict - 1,
            ..*universe
        },
        FalloutEffect::DecreaseInnovation => UniverseStats {
            innovation: universe.innovation - 1,
            ..*universe
        },
        FalloutEffect::RandomNegativeTrait => {
            if let Some(god) = choose_mut(gods) {
                apply_trait(god, TraitAlignment::Negative);
            }
            *universe
        }
        FalloutEffect::RandomPositiveTrait => {
            if let Some(god) = choose_mut(gods) {
                apply_trait(god, TraitAlignment::Positive);
            }
            *universe
        }
        FalloutEffect::None => *universe,
    }
}
