//! Combat logic for the clash phase.

use macroquad_toolkit::rng::choose;

use crate::data::types::GodStats;
use crate::engine::rules::calculate_hp;

#[derive(Debug, Clone)]
pub struct Combatant {
    pub id: usize,
    pub stats: GodStats,
    pub hp: i32,
}

impl Combatant {
    pub fn new(id: usize, stats: GodStats) -> Self {
        Self {
            id,
            stats,
            hp: calculate_hp(&stats),
        }
    }

    pub fn is_defeated(&self) -> bool {
        self.hp <= 0
    }
}

#[derive(Debug, Clone)]
pub struct ClashOutcome {
    pub left: Vec<Combatant>,
    pub right: Vec<Combatant>,
    pub log: Vec<CombatEvent>,
}

pub fn simulate_clash(left: &[GodStats], right: &[GodStats]) -> ClashOutcome {
    let mut left_team: Vec<Combatant> = left
        .iter()
        .enumerate()
        .map(|(id, stats)| Combatant::new(id, *stats))
        .collect();
    let mut right_team: Vec<Combatant> = right
        .iter()
        .enumerate()
        .map(|(id, stats)| Combatant::new(id, *stats))
        .collect();

    let mut log = Vec::new();

    for round_index in 0..3 {
        if team_defeated(&left_team) || team_defeated(&right_team) {
            break;
        }

        let mut turn_order: Vec<(bool, usize, i32)> = Vec::new();
        for (idx, combatant) in left_team.iter().enumerate() {
            if !combatant.is_defeated() {
                turn_order.push((true, idx, combatant.stats.speed));
            }
        }
        for (idx, combatant) in right_team.iter().enumerate() {
            if !combatant.is_defeated() {
                turn_order.push((false, idx, combatant.stats.speed));
            }
        }

        turn_order.sort_by(|a, b| b.2.cmp(&a.2));

        for (is_left, attacker_idx, _) in turn_order {
            if is_left {
                if left_team[attacker_idx].is_defeated() {
                    continue;
                }
                if team_defeated(&right_team) {
                    break;
                }
                if let Some(event) =
                    attack(&left_team, &mut right_team, attacker_idx, round_index, true)
                {
                    log.push(event);
                }
            } else {
                if right_team[attacker_idx].is_defeated() {
                    continue;
                }
                if team_defeated(&left_team) {
                    break;
                }
                if let Some(event) =
                    attack(&right_team, &mut left_team, attacker_idx, round_index, false)
                {
                    log.push(event);
                }
            }
        }
    }

    ClashOutcome {
        left: left_team,
        right: right_team,
        log,
    }
}

#[derive(Debug, Clone)]
pub struct CombatEvent {
    pub round: usize,
    pub attacker_id: usize,
    pub target_id: usize,
    pub damage: i32,
    pub target_defeated: bool,
    pub attacker_is_left: bool,
}

fn attack(
    attackers: &[Combatant],
    defenders: &mut [Combatant],
    attacker_idx: usize,
    round: usize,
    attacker_is_left: bool,
) -> Option<CombatEvent> {
    let attacker = &attackers[attacker_idx];
    let alive_indices: Vec<usize> = defenders
        .iter()
        .enumerate()
        .filter_map(|(idx, defender)| (!defender.is_defeated()).then_some(idx))
        .collect();

    let target_idx = match choose(&alive_indices) {
        Some(idx) => *idx,
        None => return None,
    };

    let defense = defenders[target_idx].stats.defense / 2;
    let mut damage = attacker.stats.attack - defense;
    if damage < 1 {
        damage = 1;
    }
    defenders[target_idx].hp -= damage;
    let defeated = defenders[target_idx].hp <= 0;
    Some(CombatEvent {
        round,
        attacker_id: attacker.id,
        target_id: defenders[target_idx].id,
        damage,
        target_defeated: defeated,
        attacker_is_left,
    })
}

fn team_defeated(team: &[Combatant]) -> bool {
    team.iter().all(|combatant| combatant.is_defeated())
}
