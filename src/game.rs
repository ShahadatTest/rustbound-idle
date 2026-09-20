use crate::{combat, enemy::Enemy, player::Player};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub stage: u32,
    pub player: Player,
    pub enemy: Enemy,
}

impl Default for Game {
    fn default() -> Self {
        let stage = 1;
        Self {
            stage,
            player: Player::default(),
            enemy: Enemy::for_stage(stage),
        }
    }
}

impl Game {
    pub fn click(&mut self) {
        let damage = self.player.click_damage();
        self.apply_damage_pool(u64::from(damage));
    }

    pub fn tick(&mut self, seconds: u32) {
        if seconds == 0 {
            return;
        }

        let total_damage = combat::idle_damage(self.player.idle_dps(), seconds);
        self.apply_damage_pool(total_damage);
    }

    pub fn buy_click_upgrade(&mut self) -> bool {
        self.player.buy_click_upgrade()
    }

    pub fn buy_idle_upgrade(&mut self) -> bool {
        self.player.buy_idle_upgrade()
    }

    fn apply_damage_pool(&mut self, mut damage: u64) {
        while damage > 0 {
            let current_health = u64::from(self.enemy.health.max(1));

            if damage < current_health {
                self.enemy.take_damage(damage as u32);
                break;
            }

            damage -= current_health;
            self.enemy.health = 0;
            self.defeat_enemy();
        }
    }

    fn defeat_enemy(&mut self) {
        let gold = self.enemy.gold_reward;
        let xp = self.enemy.xp_reward;

        self.player.add_gold(gold);
        self.player.add_xp(xp);
        self.player.add_kill();

        self.stage = self.stage.saturating_add(1).max(1);
        self.enemy = Enemy::for_stage(self.stage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_game_starts_at_stage_one() {
        let game = Game::default();
        assert_eq!(game.stage, 1);
        assert_eq!(game.player.level, 1);
    }

    #[test]
    fn clicking_damages_the_enemy() {
        let mut game = Game::default();
        let before = game.enemy.health;
        game.click();
        assert!(game.enemy.health < before);
    }

    #[test]
    fn defeating_an_enemy_advances_the_stage_and_rewards_player() {
        let mut game = Game::default();
        let reward = game.enemy.gold_reward;
        game.enemy.health = 1;
        game.click();

        assert_eq!(game.stage, 2);
        assert_eq!(game.player.gold, reward);
        assert_eq!(game.player.total_kills, 1);
    }

    #[test]
    fn idle_damage_can_cross_multiple_enemies() {
        let mut game = Game::default();
        game.player.idle_upgrade_level = 20;
        game.tick(10);
        assert!(game.stage > 1);
    }

    #[test]
    fn upgrades_spend_gold_and_raise_level() {
        let mut game = Game::default();
        game.player.gold = 100;
        assert!(game.buy_click_upgrade());
        assert_eq!(game.player.click_upgrade_level, 1);
        assert!(game.player.gold < 100);
    }
}
