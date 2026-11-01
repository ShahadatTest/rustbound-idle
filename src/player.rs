use crate::rules::{
    base_damage_for_level, click_damage, click_upgrade_bonus, click_upgrade_cost, xp_to_next_level,
};

pub struct Player {
    pub gold: u32,
    pub xp: u32,
    pub level: u32,
    pub click_upgrade_level: u32,
}

#[derive(Debug, PartialEq)]
pub enum PurchaseOutcome {
    Purchased,
    NotEnoughGold,
}

impl Player {
    pub(crate) fn new() -> Player {
        Player {
            gold: 0,
            xp: 0,
            level: 1,
            click_upgrade_level: 0,
        }
    }

    pub(crate) fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub(crate) fn add_xp(&mut self, amount: u32) {
        self.xp = self.xp.saturating_add(amount);

        while self.xp >= self.xp_needed() {
            self.xp -= self.xp_needed();
            self.level = self.level.saturating_add(1);
        }
    }

    pub fn xp_needed(&self) -> u32 {
        xp_to_next_level(self.level)
    }

    pub fn click_damage(&self) -> u32 {
        click_damage(self.base_damage(), self.bonus_damage())
    }

    pub fn click_upgrade_cost(&self) -> u32 {
        click_upgrade_cost(self.click_upgrade_level)
    }

    fn base_damage(&self) -> u32 {
        base_damage_for_level(self.level)
    }

    fn bonus_damage(&self) -> u32 {
        click_upgrade_bonus(self.click_upgrade_level)
    }

    pub(crate) fn buy_click_upgrade(&mut self) -> PurchaseOutcome {
        let cost = self.click_upgrade_cost();

        if self.gold < cost {
            return PurchaseOutcome::NotEnoughGold;
        }

        self.gold -= cost;
        self.click_upgrade_level = self.click_upgrade_level.saturating_add(1);

        PurchaseOutcome::Purchased
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leveling_up_keeps_leftover_xp() {
        let mut player = Player::new();

        player.add_xp(25);

        assert_eq!(player.level, 2);
        assert_eq!(player.xp, 5);
    }

    #[test]
    fn a_big_reward_can_skip_levels() {
        let mut player = Player::new();

        player.add_xp(60);

        assert_eq!(player.level, 3);
        assert_eq!(player.xp, 5);
    }

    #[test]
    fn buying_an_upgrade_needs_gold() {
        let mut player = Player::new();

        assert_eq!(player.buy_click_upgrade(), PurchaseOutcome::NotEnoughGold);
        assert_eq!(player.click_upgrade_level, 0);
        assert_eq!(player.bonus_damage(), 0);
    }

    #[test]
    fn buying_an_upgrade_spends_gold() {
        let mut player = Player::new();
        player.gold = 12;

        assert_eq!(player.buy_click_upgrade(), PurchaseOutcome::Purchased);
        assert_eq!(player.gold, 2);
        assert_eq!(player.click_upgrade_level, 1);
        assert_eq!(player.bonus_damage(), 2);
    }
}
