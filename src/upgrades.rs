const CLICK_BASE_COST: u32 = 10;
const IDLE_BASE_COST: u32 = 25;

/// Scale a price using integer math so gameplay stays deterministic.
fn scaled_cost(base: u32, level: u32, numerator: u32, denominator: u32) -> u32 {
    let mut cost = u64::from(base);

    for _ in 0..level {
        cost = cost
            .saturating_mul(u64::from(numerator))
            .saturating_add(u64::from(denominator - 1))
            / u64::from(denominator);
    }

    cost.min(u64::from(u32::MAX)) as u32
}

pub fn click_upgrade_cost(level: u32) -> u32 {
    scaled_cost(CLICK_BASE_COST, level, 3, 2)
}

pub fn idle_upgrade_cost(level: u32) -> u32 {
    scaled_cost(IDLE_BASE_COST, level, 8, 5)
}

pub fn click_upgrade_bonus(level: u32) -> u32 {
    level.saturating_mul(2)
}

pub fn idle_upgrade_dps(level: u32, player_level: u32) -> u32 {
    if level == 0 {
        return 0;
    }

    let level_multiplier = 1 + player_level.saturating_sub(1) / 3;
    level.saturating_mul(level_multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn costs_increase_with_level() {
        assert!(click_upgrade_cost(2) > click_upgrade_cost(1));
        assert!(idle_upgrade_cost(2) > idle_upgrade_cost(1));
    }

    #[test]
    fn idle_upgrade_starts_at_zero_dps() {
        assert_eq!(idle_upgrade_dps(0, 10), 0);
    }
}
