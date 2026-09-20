/// Add the player's base damage and bonus damage.
///
/// This intentionally stays tiny: it is a good candidate for the first function
/// introduced in the accompanying beginner series.
fn click_damage(base: u32, bonus: u32) -> u32 {
    base.saturating_add(bonus)
}

/// Return the health remaining after a hit without ever underflowing below zero.
fn health_after_hit(health: u32, damage: u32) -> u32 {
    health.saturating_sub(damage)
}

/// Tell us whether something with this much health has been defeated.
fn is_defeated(health: u32) -> bool {
    health == 0
}

fn main() {
    println!("Click damage: {}", click_damage(3, 2));
    println!("Health after hit: {}", health_after_hit(12, 5));
    println!("Defeated: {}", is_defeated(0));
}
