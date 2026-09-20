pub fn xp_to_next(level: u32) -> u32 {
    20u32.saturating_add(level.saturating_sub(1).saturating_mul(15))
}

/// Apply XP and return how many levels were gained.
pub fn apply_xp(level: &mut u32, xp: &mut u32, amount: u32) -> u32 {
    *xp = (*xp).saturating_add(amount);
    let mut levels_gained: u32 = 0;

    loop {
        let needed = xp_to_next(*level);
        if *xp < needed {
            break;
        }

        *xp -= needed;
        *level = (*level).saturating_add(1);
        levels_gained = levels_gained.saturating_add(1);
    }

    levels_gained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_grows_with_level() {
        assert_eq!(xp_to_next(1), 20);
        assert_eq!(xp_to_next(2), 35);
    }

    #[test]
    fn xp_can_gain_multiple_levels() {
        let mut level = 1;
        let mut xp = 0;
        let gained = apply_xp(&mut level, &mut xp, 60);

        assert_eq!(gained, 2);
        assert_eq!(level, 3);
        assert_eq!(xp, 5);
    }
}
