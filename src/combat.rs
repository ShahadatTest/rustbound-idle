/// Add the player's base damage and bonus damage.
///
/// This intentionally stays tiny: it is a good candidate for the first function
/// introduced in the accompanying beginner series.
pub fn click_damage(base: u32, bonus: u32) -> u32 {
    base.saturating_add(bonus)
}

/// Return the health remaining after a hit without ever underflowing below zero.
pub fn health_after_hit(health: u32, damage: u32) -> u32 {
    health.saturating_sub(damage)
}

/// Tell us whether something with this much health has been defeated.
pub fn is_defeated(health: u32) -> bool {
    health == 0
}

/// Convert damage-per-second and elapsed seconds into one deterministic damage pool.
pub fn idle_damage(damage_per_second: u32, seconds: u32) -> u64 {
    u64::from(damage_per_second).saturating_mul(u64::from(seconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn click_damage_adds_base_and_bonus() {
        assert_eq!(click_damage(3, 4), 7);
    }

    #[test]
    fn health_never_underflows() {
        assert_eq!(health_after_hit(3, 10), 0);
    }

    #[test]
    fn zero_health_is_defeated() {
        assert!(is_defeated(0));
        assert!(!is_defeated(1));
    }

    #[test]
    fn idle_damage_uses_elapsed_time() {
        assert_eq!(idle_damage(5, 10), 50);
    }
}
