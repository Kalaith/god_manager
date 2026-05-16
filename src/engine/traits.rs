//! Trait helpers and mappings.

use macroquad_toolkit::rng::choose;

use crate::data::types::{God, PerformanceTag, TraitAlignment};

const POSITIVE_TRAITS: [&str; 2] = ["Battle-Hardened", "Inspired"];
const NEGATIVE_TRAITS: [&str; 2] = ["Overextended", "Fractured Faith"];

pub fn alignment_from_performance(tag: PerformanceTag) -> TraitAlignment {
    match tag {
        PerformanceTag::Strong => TraitAlignment::Positive,
        PerformanceTag::Weak => TraitAlignment::Negative,
        PerformanceTag::Neutral => TraitAlignment::Positive,
    }
}

pub fn pick_trait(alignment: TraitAlignment) -> &'static str {
    match alignment {
        TraitAlignment::Positive => choose(&POSITIVE_TRAITS).copied().unwrap_or("Inspired"),
        TraitAlignment::Negative => choose(&NEGATIVE_TRAITS).copied().unwrap_or("Overextended"),
    }
}

pub fn apply_trait(god: &mut God, alignment: TraitAlignment) {
    let trait_name = pick_trait(alignment);
    god.traits.push(trait_name.to_string());
}

pub fn apply_trait_for_performance(god: &mut God, tag: PerformanceTag) {
    let alignment = alignment_from_performance(tag);
    apply_trait(god, alignment);
}
