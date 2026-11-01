use crate::rules::{
    enemy_health_for_stage,
    gold_reward_for_stage,
    xp_reward_for_stage,
};

pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub(crate) gold_reward: u32,
    pub(crate) xp_reward: u32,
}

impl Enemy {
    pub(crate) fn for_stage(stage: u32) -> Enemy {
        Enemy {
            name: String::from("Training Slime"),
            health: enemy_health_for_stage(stage),
            gold_reward: gold_reward_for_stage(stage),
            xp_reward: xp_reward_for_stage(stage),
        }
    }
}
