//! Match data and progression.

use crate::data::types::{
    CultivationAllocation,
    God,
    GodRole,
    GodStats,
    PerformanceTag,
    UniverseStats,
};
use crate::engine::combat::{ClashOutcome, Combatant, simulate_clash};
use crate::engine::fallout::{apply_fallout, apply_stability_loss};
use crate::engine::match_rules::{evaluate_match_end, resolve_sudden_death, MatchOutcome};
use crate::engine::rules::{apply_cultivation, calculate_god_stats, roll_fallout};
use crate::engine::traits::apply_trait_for_performance;

pub struct GameData {
    pub season: u8,
    pub mode: MatchMode,
    pub left_universe: UniverseStats,
    pub right_universe: UniverseStats,
    pub left_gods: Vec<God>,
    pub right_gods: Vec<God>,
    pub left_stats: Vec<GodStats>,
    pub right_stats: Vec<GodStats>,
    pub last_clash: Option<ClashOutcome>,
    pub last_outcome: MatchOutcome,
    pub last_roll_left: Option<u8>,
    pub last_roll_right: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchMode {
    Pvp,
    VsAi,
}

impl GameData {
    pub fn new() -> Self {
        let mut data = Self {
            season: 1,
            mode: MatchMode::VsAi,
            left_universe: UniverseStats::new_default(),
            right_universe: UniverseStats::new_default(),
            left_gods: default_gods("A"),
            right_gods: default_gods("B"),
            left_stats: Vec::new(),
            right_stats: Vec::new(),
            last_clash: None,
            last_outcome: MatchOutcome::Ongoing,
            last_roll_left: None,
            last_roll_right: None,
        };
        data.recalculate_stats();
        data
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn set_mode(&mut self, mode: MatchMode) {
        self.mode = mode;
    }

    pub fn apply_cultivation(&mut self, allocation: CultivationAllocation) -> Result<(), &'static str> {
        self.left_universe = apply_cultivation(&self.left_universe, &allocation)?;
        self.right_universe = apply_cultivation(&self.right_universe, &allocation)?;
        self.recalculate_stats();
        Ok(())
    }

    pub fn run_clash_with_selection(&mut self, left_indices: &[usize], right_indices: &[usize]) {
        self.recalculate_stats();
        let left_stats = select_stats(&self.left_stats, left_indices);
        let right_stats = select_stats(&self.right_stats, right_indices);
        self.last_clash = Some(simulate_clash(&left_stats, &right_stats));
    }

    pub fn top_three_indices(&self, is_left: bool) -> Vec<usize> {
        let stats = if is_left { &self.left_stats } else { &self.right_stats };
        let mut indices: Vec<usize> = (0..stats.len()).collect();
        indices.sort_by_key(|&idx| -(stats[idx].attack + stats[idx].defense + stats[idx].speed));
        indices.truncate(3);
        indices
    }

    pub fn apply_fallout(&mut self) {
        if let Some(outcome) = &self.last_clash {
            let left_defeated = count_defeated(&outcome.left);
            let right_defeated = count_defeated(&outcome.right);

            self.left_universe = apply_stability_loss(&self.left_universe, left_defeated);
            self.right_universe = apply_stability_loss(&self.right_universe, right_defeated);

            let roll_left = roll_fallout();
            let roll_right = roll_fallout();
            self.last_roll_left = Some(roll_left);
            self.last_roll_right = Some(roll_right);

            self.left_universe = apply_fallout(&self.left_universe, roll_left, &mut self.left_gods);
            self.right_universe = apply_fallout(&self.right_universe, roll_right, &mut self.right_gods);

            apply_performance_traits(&mut self.left_gods, &outcome.left);
            apply_performance_traits(&mut self.right_gods, &outcome.right);
        } else {
            for god in &mut self.left_gods {
                apply_trait_for_performance(god, PerformanceTag::Neutral);
            }
            for god in &mut self.right_gods {
                apply_trait_for_performance(god, PerformanceTag::Neutral);
            }
        }

        self.last_outcome = evaluate_match_end(self.season, &self.left_universe, &self.right_universe);
        if self.last_outcome == MatchOutcome::SuddenDeath {
            let sudden = resolve_sudden_death(&self.left_stats, &self.right_stats);
            if sudden != MatchOutcome::SuddenDeath {
                self.last_outcome = sudden;
            }
        }

        if self.season < 3 {
            self.season += 1;
        }
    }

    fn recalculate_stats(&mut self) {
        self.left_stats = self
            .left_gods
            .iter()
            .map(|god| calculate_god_stats(&self.left_universe, god.role))
            .collect();
        self.right_stats = self
            .right_gods
            .iter()
            .map(|god| calculate_god_stats(&self.right_universe, god.role))
            .collect();
    }
}

fn default_gods(suffix: &str) -> Vec<God> {
    vec![
        God {
            name: format!("Gogus {suffix}"),
            role: GodRole::Tank,
            domain: "War".to_string(),
            traits: Vec::new(),
        },
        God {
            name: format!("Lumina {suffix}"),
            role: GodRole::Striker,
            domain: "Fire".to_string(),
            traits: Vec::new(),
        },
        God {
            name: format!("Vexis {suffix}"),
            role: GodRole::Control,
            domain: "Time".to_string(),
            traits: Vec::new(),
        },
        God {
            name: format!("Nyra {suffix}"),
            role: GodRole::Support,
            domain: "Hope".to_string(),
            traits: Vec::new(),
        },
    ]
}

fn count_defeated(team: &[Combatant]) -> i32 {
    team.iter().filter(|combatant| combatant.hp <= 0).count() as i32
}

fn apply_performance_traits(gods: &mut [God], combatants: &[Combatant]) {
    for (idx, god) in gods.iter_mut().enumerate() {
        if let Some(combatant) = combatants.get(idx) {
            let tag = if combatant.hp <= 0 {
                PerformanceTag::Weak
            } else {
                PerformanceTag::Strong
            };
            apply_trait_for_performance(god, tag);
        } else {
            apply_trait_for_performance(god, PerformanceTag::Neutral);
        }
    }
}

fn select_stats(all_stats: &[GodStats], indices: &[usize]) -> Vec<GodStats> {
    indices
        .iter()
        .filter_map(|&idx| all_stats.get(idx).copied())
        .collect()
}
