use serde::{Deserialize, Serialize};

use crate::{combat, progression, upgrades};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub level: u32,
    pub xp: u32,
    pub gold: u32,
    pub click_upgrade_level: u32,
    pub idle_upgrade_level: u32,
    pub total_kills: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            xp: 0,
            gold: 0,
            click_upgrade_level: 0,
            idle_upgrade_level: 0,
            total_kills: 0,
        }
    }
}

impl Player {
    pub fn base_click_damage(&self) -> u32 {
        1u32.saturating_add(self.level.saturating_sub(1))
    }

    pub fn click_damage(&self) -> u32 {
        combat::click_damage(
            self.base_click_damage(),
            upgrades::click_upgrade_bonus(self.click_upgrade_level),
        )
    }

    pub fn idle_dps(&self) -> u32 {
        upgrades::idle_upgrade_dps(self.idle_upgrade_level, self.level)
    }

    pub fn click_upgrade_cost(&self) -> u32 {
        upgrades::click_upgrade_cost(self.click_upgrade_level)
    }

    pub fn idle_upgrade_cost(&self) -> u32 {
        upgrades::idle_upgrade_cost(self.idle_upgrade_level)
    }

    pub fn can_afford(&self, cost: u32) -> bool {
        self.gold >= cost
    }

    pub fn spend_gold(&mut self, amount: u32) -> bool {
        if !self.can_afford(amount) {
            return false;
        }

        self.gold -= amount;
        true
    }

    pub fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub fn add_kill(&mut self) {
        self.total_kills = self.total_kills.saturating_add(1);
    }

    pub fn add_xp(&mut self, amount: u32) -> u32 {
        progression::apply_xp(&mut self.level, &mut self.xp, amount)
    }

    pub fn buy_click_upgrade(&mut self) -> bool {
        let cost = self.click_upgrade_cost();
        if !self.spend_gold(cost) {
            return false;
        }

        self.click_upgrade_level = self.click_upgrade_level.saturating_add(1);
        true
    }

    pub fn buy_idle_upgrade(&mut self) -> bool {
        let cost = self.idle_upgrade_cost();
        if !self.spend_gold(cost) {
            return false;
        }

        self.idle_upgrade_level = self.idle_upgrade_level.saturating_add(1);
        true
    }
}
