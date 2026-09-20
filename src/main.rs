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

fn attack(health: u32, base_damage: u32, bonus_damage: u32) -> u32 {
    if is_defeated(health) {
        return 0;
    }

    let damage = click_damage(base_damage, bonus_damage);
    health_after_hit(health, damage)
}

fn main() {
    let remaining = attack(12, 3, 2);
    println!("Health after attack: {remaining}");
    println!("Defeated: {}", is_defeated(remaining));
    println!("Already defeated: {}", attack(0, 3, 2));
}
