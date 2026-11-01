struct Player {
    gold: u32,
    xp: u32,
    level: u32,
    click_upgrade_level: u32,
}

struct Enemy {
    name: String,
    health: u32,
    gold_reward: u32,
    xp_reward: u32,
}

struct Game {
    stage: u32,
    player: Player,
    enemy: Enemy,
}

#[derive(Debug, PartialEq)]
enum PurchaseOutcome {
    Purchased,
    NotEnoughGold,
}

impl Player {
    fn new() -> Player {
        Player {
            gold: 0,
            xp: 0,
            level: 1,
            click_upgrade_level: 0,
        }
    }

    fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }

    fn add_xp(&mut self, amount: u32) {
        self.xp = self.xp.saturating_add(amount);

        while self.xp >= xp_to_next_level(self.level) {
            self.xp -= xp_to_next_level(self.level);
            self.level = self.level.saturating_add(1);
        }
    }

    fn base_damage(&self) -> u32 {
        base_damage_for_level(self.level)
    }

    fn bonus_damage(&self) -> u32 {
        click_upgrade_bonus(self.click_upgrade_level)
    }

    fn buy_click_upgrade(&mut self) -> PurchaseOutcome {
        let cost = click_upgrade_cost(self.click_upgrade_level);

        if self.gold < cost {
            return PurchaseOutcome::NotEnoughGold;
        }

        self.gold -= cost;
        self.click_upgrade_level = self.click_upgrade_level.saturating_add(1);

        PurchaseOutcome::Purchased
    }
}

impl Enemy {
    fn for_stage(stage: u32) -> Enemy {
        Enemy {
            name: String::from("Training Slime"),
            health: enemy_health_for_stage(stage),
            gold_reward: gold_reward_for_stage(stage),
            xp_reward: xp_reward_for_stage(stage),
        }
    }
}

impl Game {
    fn new() -> Game {
        Game {
            stage: 1,
            player: Player::new(),
            enemy: Enemy::for_stage(1),
        }
    }

    fn click(&mut self) {
        attack(&self.player, &mut self.enemy);

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

    fn buy_click_upgrade(&mut self) -> PurchaseOutcome {
        self.player.buy_click_upgrade()
    }
}

fn click_damage(base: u32, bonus: u32) -> u32 {
    base.saturating_add(bonus)
}

fn health_after_hit(health: u32, damage: u32) -> u32 {
    health.saturating_sub(damage)
}

fn is_defeated(health: u32) -> bool {
    health == 0
}

fn enemy_health_for_stage(stage: u32) -> u32 {
    stage.saturating_mul(10).saturating_add(10)
}

fn gold_reward_for_stage(stage: u32) -> u32 {
    stage.saturating_mul(2).saturating_add(3)
}

fn xp_reward_for_stage(stage: u32) -> u32 {
    stage.saturating_mul(2).saturating_add(4)
}

fn xp_to_next_level(level: u32) -> u32 {
    level
        .saturating_sub(1)
        .saturating_mul(15)
        .saturating_add(20)
}

fn base_damage_for_level(level: u32) -> u32 {
    level.saturating_add(4)
}

fn click_upgrade_bonus(upgrades: u32) -> u32 {
    upgrades.saturating_mul(2)
}

fn click_upgrade_cost(upgrades: u32) -> u32 {
    upgrades.saturating_add(1).saturating_mul(10)
}

fn attack(player: &Player, enemy: &mut Enemy) {
    let damage = click_damage(player.base_damage(), player.bonus_damage());

    enemy.health = health_after_hit(enemy.health, damage);
}

fn main() {
    let mut game = Game::new();

    while game.stage <= 8 {
        let stage = game.stage;
        let level = game.player.level;

        game.click();

        if game.stage > stage {
            println!(
                "Stage {stage} cleared!  Gold: {}  XP: {}/{}",
                game.player.gold,
                game.player.xp,
                xp_to_next_level(game.player.level)
            );

            if game.player.level > level {
                println!(
                    "Level up! Now level {}. Click damage: {}",
                    game.player.level,
                    click_damage(game.player.base_damage(), game.player.bonus_damage())
                );
            }

            println!(
                "  Next up: {} with {} health.",
                game.enemy.name, game.enemy.health
            );
        }

        match game.buy_click_upgrade() {
            PurchaseOutcome::Purchased => println!(
                "Bought click upgrade {}! Click damage: {}  Gold left: {}",
                game.player.click_upgrade_level,
                click_damage(game.player.base_damage(), game.player.bonus_damage()),
                game.player.gold
            ),
            PurchaseOutcome::NotEnoughGold => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn click_damage_adds_base_and_bonus() {
        assert_eq!(click_damage(3, 4), 7);
    }

    #[test]
    fn health_never_drops_below_zero() {
        assert_eq!(health_after_hit(3, 10), 0);
    }

    #[test]
    fn zero_health_is_defeated() {
        assert!(is_defeated(0));
        assert!(!is_defeated(1));
    }

    #[test]
    fn attack_reduces_enemy_health() {
        let player = Player::new();

        let mut enemy = Enemy::for_stage(1);
        enemy.health = 10;

        attack(&player, &mut enemy);

        assert_eq!(enemy.health, 5);
    }

    #[test]
    fn later_stages_have_tougher_enemies() {
        assert_eq!(enemy_health_for_stage(1), 20);
        assert_eq!(enemy_health_for_stage(2), 30);
    }

    #[test]
    fn later_stages_pay_better() {
        assert_eq!(gold_reward_for_stage(1), 5);
        assert_eq!(gold_reward_for_stage(2), 7);
        assert_eq!(xp_reward_for_stage(1), 6);
        assert_eq!(xp_reward_for_stage(2), 8);
    }

    #[test]
    fn levels_need_more_xp_as_they_go() {
        assert_eq!(xp_to_next_level(1), 20);
        assert_eq!(xp_to_next_level(2), 35);
    }

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
    fn levels_make_the_player_hit_harder() {
        assert_eq!(base_damage_for_level(1), 5);
        assert_eq!(base_damage_for_level(2), 6);
    }

    #[test]
    fn upgrades_cost_more_each_time() {
        assert_eq!(click_upgrade_cost(0), 10);
        assert_eq!(click_upgrade_cost(1), 20);
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
}
