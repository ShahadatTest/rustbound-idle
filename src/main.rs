struct Player {
    base_damage: u32,
    bonus_damage: u32,
}

struct Enemy {
    name: String,
    health: u32,
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
    let player = Player {
        base_damage: 5,
        bonus_damage: 2,
    };

    let mut enemy = Enemy {
        name: String::from("Training Slime"),
        health: 20,
    };

    while !is_defeated(enemy.health) {
        attack(&player, &mut enemy);

        println!(
            "{} health: {}",
            enemy.name,
            enemy.health
        );
    }

    println!("{} defeated!", enemy.name);
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
}
