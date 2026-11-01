struct Player {
    base_damage: u32,
    bonus_damage: u32,
}

struct Enemy {
    name: String,
    health: u32,
}

struct Game {
    stage: u32,
    player: Player,
    enemy: Enemy,
}

impl Enemy {
    fn for_stage(stage: u32) -> Enemy {
        Enemy {
            name: String::from("Training Slime"),
            health: enemy_health_for_stage(stage),
        }
    }
}

impl Game {
    fn new() -> Game {
        Game {
            stage: 1,
            player: Player {
                base_damage: 5,
                bonus_damage: 2,
            },
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
        self.stage = self.stage.saturating_add(1);
        self.enemy = Enemy::for_stage(self.stage);
    }
}

fn click_damage(base: u32, bonus: u32) -> u32 {
    base.saturating_add(bonus)
}

fn health_after_hit(
    health: u32,
    damage: u32,
) -> u32 {
    health.saturating_sub(damage)
}

fn is_defeated(health: u32) -> bool {
    health == 0
}

fn enemy_health_for_stage(stage: u32) -> u32 {
    stage.saturating_mul(10).saturating_add(10)
}

fn attack(
    player: &Player,
    enemy: &mut Enemy,
) {
    let damage = click_damage(
        player.base_damage,
        player.bonus_damage,
    );

    enemy.health =
        health_after_hit(enemy.health, damage);
}

fn main() {
    let mut game = Game::new();

    while game.stage <= 3 {
        let stage = game.stage;

        game.click();

        if game.stage > stage {
            println!("Stage {stage} cleared!");
        } else {
            println!(
                "{} health: {}",
                game.enemy.name,
                game.enemy.health
            );
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
        assert_eq!(
            health_after_hit(3, 10),
            0
        );
    }

    #[test]
    fn zero_health_is_defeated() {
        assert!(is_defeated(0));
        assert!(!is_defeated(1));
    }

    #[test]
    fn attack_reduces_enemy_health() {
        let player = Player {
            base_damage: 5,
            bonus_damage: 2,
        };

        let mut enemy = Enemy {
            name: String::from("Training Slime"),
            health: 10,
        };

        attack(&player, &mut enemy);

        assert_eq!(enemy.health, 3);
    }

    #[test]
    fn later_stages_have_tougher_enemies() {
        assert_eq!(enemy_health_for_stage(1), 20);
        assert_eq!(enemy_health_for_stage(2), 30);
    }

    #[test]
    fn a_new_game_starts_at_stage_one() {
        let game = Game::new();

        assert_eq!(game.stage, 1);
        assert_eq!(game.enemy.health, 20);
    }

    #[test]
    fn clicking_damages_the_enemy() {
        let mut game = Game::new();

        game.click();

        assert_eq!(game.enemy.health, 13);
    }

    #[test]
    fn defeating_an_enemy_advances_the_stage() {
        let mut game = Game::new();
        game.enemy.health = 1;

        game.click();

        assert_eq!(game.stage, 2);
        assert_eq!(game.enemy.health, 30);
    }
}
