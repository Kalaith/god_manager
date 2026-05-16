//! Core data types for the game.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UniverseStats {
    pub belief: i32,
    pub conflict: i32,
    pub innovation: i32,
    pub stability: i32,
}

impl UniverseStats {
    pub fn new_default() -> Self {
        Self {
            belief: 5,
            conflict: 5,
            innovation: 5,
            stability: 20,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CultivationAllocation {
    pub belief: i32,
    pub conflict: i32,
    pub innovation: i32,
}

impl CultivationAllocation {
    pub fn total_points(&self) -> i32 {
        self.belief + self.conflict + self.innovation
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GodRole {
    Tank,
    Striker,
    Control,
    Support,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct God {
    pub name: String,
    pub role: GodRole,
    pub domain: String,
    pub traits: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraitAlignment {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceTag {
    Strong,
    Neutral,
    Weak,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GodStats {
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl GodStats {
    pub fn new(attack: i32, defense: i32, speed: i32) -> Self {
        Self {
            attack,
            defense,
            speed,
        }
    }
}
