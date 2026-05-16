//! Core calculation rules.

use macroquad_toolkit::rng::gen_range;

use crate::data::types::{CultivationAllocation, GodRole, GodStats, UniverseStats};

pub const CULTIVATION_POINTS: i32 = 6;
pub const CULTIVATION_MAX_PER_STAT: i32 = 4;

pub fn calculate_god_stats(universe: &UniverseStats, role: GodRole) -> GodStats {
    match role {
        GodRole::Tank => GodStats::new(
            universe.conflict,
            universe.belief * 2,
            universe.innovation,
        ),
        GodRole::Striker => GodStats::new(
            universe.conflict * 2,
            universe.belief,
            universe.innovation,
        ),
        GodRole::Control => GodStats::new(
            universe.conflict,
            universe.belief,
            universe.innovation * 2,
        ),
        GodRole::Support => GodStats::new(
            universe.conflict,
            universe.belief + universe.innovation,
            universe.innovation,
        ),
    }
}

pub fn calculate_hp(stats: &GodStats) -> i32 {
    10 + stats.defense
}

pub fn roll_fallout() -> u8 {
    gen_range(1, 7) as u8
}

pub fn validate_allocation(allocation: &CultivationAllocation) -> Result<(), &'static str> {
    if allocation.belief < 0 || allocation.conflict < 0 || allocation.innovation < 0 {
        return Err("Allocation values must be non-negative.");
    }
    if allocation.belief > CULTIVATION_MAX_PER_STAT
        || allocation.conflict > CULTIVATION_MAX_PER_STAT
        || allocation.innovation > CULTIVATION_MAX_PER_STAT
    {
        return Err("Allocation exceeds per-stat cap.");
    }
    if allocation.total_points() != CULTIVATION_POINTS {
        return Err("Allocation must use all points.");
    }
    Ok(())
}

pub fn apply_cultivation(
    universe: &UniverseStats,
    allocation: &CultivationAllocation,
) -> Result<UniverseStats, &'static str> {
    validate_allocation(allocation)?;
    Ok(UniverseStats {
        belief: universe.belief + allocation.belief,
        conflict: universe.conflict + allocation.conflict,
        innovation: universe.innovation + allocation.innovation,
        stability: universe.stability,
    })
}
