use crate::enemy::Enemy;
use crate::player::{Player, PurchaseOutcome};
use crate::rules::{
    health_after_hit,
    idle_damage_for_level,
    is_defeated,
};

pub struct Game {
    pub stage: u32,
    pub player: Player,
    pub enemy: Enemy,
}

impl Game {
    pub fn new() -> Game {
        Game {
            stage: 1,
            player: Player::new(),
            enemy: Enemy::for_stage(1),
        }
    }

    pub fn click(&mut self) {
        attack(&self.player, &mut self.enemy);
        self.resolve_hit();
    }

    pub fn tick(&mut self, seconds: u32) {
        for _ in 0..seconds {
            let damage =
                idle_damage_for_level(self.player.level);

            self.enemy.health =
                health_after_hit(
                    self.enemy.health,
                    damage
                );

            self.resolve_hit();
        }
    }

    fn resolve_hit(&mut self) {
        if is_defeated(self.enemy.health) {
            self.defeat_enemy();
        }
    }

    fn defeat_enemy(&mut self) {
        self.player.add_gold(self.enemy.gold_reward);
        self.player.add_xp(self.enemy.xp_reward);

        self.stage = self.stage.saturating_add(1);
        self.enemy = Enemy::for_stage(self.stage);
    }

    pub fn buy_click_upgrade(
        &mut self
    ) -> PurchaseOutcome {
        self.player.buy_click_upgrade()
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new()
    }
}

fn attack(player: &Player, enemy: &mut Enemy) {
    enemy.health =
        health_after_hit(
            enemy.health,
            player.click_damage()
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_reduces_enemy_health() {
        let player = Player::new();

        let mut enemy = Enemy::for_stage(1);
        enemy.health = 10;

        attack(&player, &mut enemy);

        assert_eq!(enemy.health, 5);
    }

    #[test]
    fn a_new_game_starts_at_stage_one() {
        let game = Game::new();

        assert_eq!(game.stage, 1);
        assert_eq!(game.enemy.health, 20);
        assert_eq!(game.player.gold, 0);
    }

    #[test]
    fn clicking_damages_the_enemy() {
        let mut game = Game::new();

        game.click();

        assert_eq!(game.enemy.health, 15);
    }

    #[test]
    fn defeating_an_enemy_pays_the_player() {
        let mut game = Game::new();
        game.enemy.health = 1;

        game.click();

        assert_eq!(game.stage, 2);
        assert_eq!(game.enemy.health, 30);
        assert_eq!(game.player.gold, 5);
        assert_eq!(game.player.xp, 6);
    }

    #[test]
    fn one_second_is_one_idle_hit() {
        let mut game = Game::new();

        game.tick(1);

        assert_eq!(game.enemy.health, 19);
    }

    #[test]
    fn zero_seconds_changes_nothing() {
        let mut game = Game::new();

        game.tick(0);

        assert_eq!(game.stage, 1);
        assert_eq!(game.enemy.health, 20);
    }

    #[test]
    fn enough_time_defeats_the_enemy() {
        let mut game = Game::new();

        game.tick(20);

        assert_eq!(game.stage, 2);
        assert_eq!(game.enemy.health, 30);
        assert_eq!(game.player.gold, 5);
        assert_eq!(game.player.xp, 6);
    }

    #[test]
    fn one_long_tick_matches_many_short_ones() {
        let mut long = Game::new();
        let mut short = Game::new();

        long.tick(45);

        for _ in 0..45 {
            short.tick(1);
        }

        assert_eq!(long.stage, short.stage);
        assert_eq!(
            long.enemy.health,
            short.enemy.health
        );
        assert_eq!(
            long.player.gold,
            short.player.gold
        );
        assert_eq!(
            long.player.xp,
            short.player.xp
        );
        assert_eq!(
            long.player.level,
            short.player.level
        );
    }
}
