use serde::{Deserialize, Serialize};

use crate::combat;

const ENEMY_NAMES: [&str; 8] = [
    "Moss Slime",
    "Cave Beetle",
    "Scrap Rat",
    "Dust Imp",
    "Iron Crab",
    "Glow Moth",
    "Stone Toad",
    "Copper Wisp",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub gold_reward: u32,
    pub xp_reward: u32,
    pub is_boss: bool,
}

impl Enemy {
    pub fn for_stage(stage: u32) -> Self {
        let stage = stage.max(1);
        let is_boss = stage % 10 == 0;
        let normal_health = 8u32
            .saturating_add(stage.saturating_mul(4))
            .saturating_add(stage.saturating_mul(stage) / 2);
        let max_health = if is_boss {
            normal_health.saturating_mul(3)
        } else {
            normal_health
        };

        let reward_multiplier = if is_boss { 3 } else { 1 };
        let gold_reward = 3u32
            .saturating_add(stage.saturating_mul(2))
            .saturating_mul(reward_multiplier);
        let xp_reward = 4u32
            .saturating_add(stage.saturating_mul(2))
            .saturating_mul(reward_multiplier);

        let name = if is_boss {
            format!("Iron Warden {stage}")
        } else {
            let index = ((stage - 1) as usize) % ENEMY_NAMES.len();
            ENEMY_NAMES[index].to_string()
        };

        Self {
            name,
            health: max_health,
            max_health,
            gold_reward,
            xp_reward,
            is_boss,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = combat::health_after_hit(self.health, damage);
    }

    pub fn defeated(&self) -> bool {
        combat::is_defeated(self.health)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_enemies_are_stronger() {
        assert!(Enemy::for_stage(5).max_health > Enemy::for_stage(1).max_health);
    }

    #[test]
    fn every_tenth_enemy_is_a_boss() {
        let boss = Enemy::for_stage(10);
        assert!(boss.is_boss);
        assert!(boss.name.contains("Iron Warden"));
    }
}
