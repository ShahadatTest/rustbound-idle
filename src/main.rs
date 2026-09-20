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

struct Player {
    base_damage: u32,
    bonus_damage: u32,
    gold: u32,
    total_kills: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            base_damage: 3,
            bonus_damage: 2,
            gold: 0,
            total_kills: 0,
        }
    }

    fn click_damage(&self) -> u32 {
        click_damage(self.base_damage, self.bonus_damage)
    }
}

struct Enemy {
    health: u32,
}

impl Enemy {
    fn new(health: u32) -> Self {
        Self { health }
    }

    fn take_damage(&mut self, damage: u32) {
        self.health = health_after_hit(self.health, damage);
    }

    fn defeated(&self) -> bool {
        is_defeated(self.health)
    }
}

fn attack(player: &Player, enemy: &mut Enemy) {
    if enemy.defeated() {
        return;
    }

    enemy.take_damage(player.click_damage());
}

struct Game {
    stage: u32,
    player: Player,
    enemy: Enemy,
}

impl Game {
    fn new() -> Self {
        Self {
            stage: 1,
            player: Player::new(),
            enemy: Enemy::new(12),
        }
    }

    fn click(&mut self) {
        attack(&self.player, &mut self.enemy);
        if self.enemy.defeated() {
            self.defeat_enemy();
        }
    }

    fn defeat_enemy(&mut self) {
        self.player.gold = self.player.gold.saturating_add(5);
        self.player.total_kills = self.player.total_kills.saturating_add(1);
        self.stage = self.stage.saturating_add(1);
        self.enemy = Enemy::new(12);
    }
}

fn main() {
    let mut game = Game::new();
    game.click();
    game.click();
    game.click();
    println!(
        "Stage: {}; gold: {}; kills: {}; next enemy HP: {}",
        game.stage, game.player.gold, game.player.total_kills, game.enemy.health
    );
}
