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

fn main() {
    let enemy_health = 5;

    let damage = click_damage(5, 2);

    let remaining_health =
        health_after_hit(enemy_health, damage);

    println!("You dealt {damage} damage!");
    println!("Enemy health: {remaining_health}");

    if is_defeated(remaining_health) {
        println!("Enemy defeated!");
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
}
