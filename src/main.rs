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

/// Convert damage-per-second and elapsed seconds into one deterministic damage pool.
fn idle_damage(damage_per_second: u32, seconds: u32) -> u64 {
    u64::from(damage_per_second).saturating_mul(u64::from(seconds))
}

fn xp_to_next(level: u32) -> u32 {
    20u32.saturating_add(level.saturating_sub(1).saturating_mul(15))
}

/// Apply XP and return how many levels were gained.
fn apply_xp(level: &mut u32, xp: &mut u32, amount: u32) -> u32 {
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

fn click_upgrade_cost(level: u32) -> u32 {
    scaled_cost(CLICK_BASE_COST, level, 3, 2)
}

fn idle_upgrade_cost(level: u32) -> u32 {
    scaled_cost(IDLE_BASE_COST, level, 8, 5)
}

fn click_upgrade_bonus(level: u32) -> u32 {
    level.saturating_mul(2)
}

fn idle_upgrade_dps(level: u32, player_level: u32) -> u32 {
    if level == 0 {
        return 0;
    }

    let level_multiplier = 1 + player_level.saturating_sub(1) / 3;
    level.saturating_mul(level_multiplier)
}

struct Player {
    level: u32,
    xp: u32,
    gold: u32,
    click_upgrade_level: u32,
    idle_upgrade_level: u32,
    total_kills: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            level: 1,
            xp: 0,
            gold: 0,
            click_upgrade_level: 0,
            idle_upgrade_level: 0,
            total_kills: 0,
        }
    }
}

impl Player {
    fn base_click_damage(&self) -> u32 {
        1u32.saturating_add(self.level.saturating_sub(1))
    }

    fn click_damage(&self) -> u32 {
        click_damage(
            self.base_click_damage(),
            click_upgrade_bonus(self.click_upgrade_level),
        )
    }

    fn idle_dps(&self) -> u32 {
        idle_upgrade_dps(self.idle_upgrade_level, self.level)
    }

    fn click_upgrade_cost(&self) -> u32 {
        click_upgrade_cost(self.click_upgrade_level)
    }

    fn idle_upgrade_cost(&self) -> u32 {
        idle_upgrade_cost(self.idle_upgrade_level)
    }

    fn can_afford(&self, cost: u32) -> bool {
        self.gold >= cost
    }

    fn spend_gold(&mut self, amount: u32) -> bool {
        if !self.can_afford(amount) {
            return false;
        }

        self.gold -= amount;
        true
    }

    fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }

    fn add_kill(&mut self) {
        self.total_kills = self.total_kills.saturating_add(1);
    }

    fn add_xp(&mut self, amount: u32) -> u32 {
        apply_xp(&mut self.level, &mut self.xp, amount)
    }

    fn buy_click_upgrade(&mut self) -> bool {
        let cost = self.click_upgrade_cost();
        if !self.spend_gold(cost) {
            return false;
        }

        self.click_upgrade_level = self.click_upgrade_level.saturating_add(1);
        true
    }

    fn buy_idle_upgrade(&mut self) -> bool {
        let cost = self.idle_upgrade_cost();
        if !self.spend_gold(cost) {
            return false;
        }

        self.idle_upgrade_level = self.idle_upgrade_level.saturating_add(1);
        true
    }
}

const ENEMY_NAMES: [&str; 8] = [
    "Moss Slime",
    "Cave Beetle",
    "Scrap Rat",
    "Dust Imp",
    "Iron Crab",
    "Glow Moth",
    "Stone Toad",
    "Copper Wisp",
];

struct Enemy {
    name: String,
    health: u32,
    max_health: u32,
    gold_reward: u32,
    xp_reward: u32,
    is_boss: bool,
}

impl Enemy {
    fn for_stage(stage: u32) -> Self {
        let stage = stage.max(1);
        let is_boss = stage % 10 == 0;
        let normal_health = 8u32
            .saturating_add(stage.saturating_mul(4))
            .saturating_add(stage.saturating_mul(stage) / 2);
        let max_health = if is_boss {
            normal_health.saturating_mul(3)
        } else {
            normal_health
        };

        let reward_multiplier = if is_boss { 3 } else { 1 };
        let gold_reward = 3u32
            .saturating_add(stage.saturating_mul(2))
            .saturating_mul(reward_multiplier);
        let xp_reward = 4u32
            .saturating_add(stage.saturating_mul(2))
            .saturating_mul(reward_multiplier);

        let name = if is_boss {
            format!("Iron Warden {stage}")
        } else {
            let index = ((stage - 1) as usize) % ENEMY_NAMES.len();
            ENEMY_NAMES[index].to_string()
        };

        Self {
            name,
            health: max_health,
            max_health,
            gold_reward,
            xp_reward,
            is_boss,
        }
    }

    fn take_damage(&mut self, damage: u32) {
        self.health = health_after_hit(self.health, damage);
    }

    fn defeated(&self) -> bool {
        is_defeated(self.health)
    }
}

struct Game {
    stage: u32,
    player: Player,
    enemy: Enemy,
}

impl Game {
    fn new() -> Self {
        let stage = 1;
        Self {
            stage,
            player: Player::new(),
            enemy: Enemy::for_stage(stage),
        }
    }
}

impl Game {
    fn click(&mut self) {
        let damage = self.player.click_damage();
        self.apply_damage_pool(u64::from(damage));
    }

    fn tick(&mut self, seconds: u32) {
        if seconds == 0 {
            return;
        }

        let total_damage = idle_damage(self.player.idle_dps(), seconds);
        self.apply_damage_pool(total_damage);
    }

    fn buy_click_upgrade(&mut self) -> bool {
        self.player.buy_click_upgrade()
    }

    fn buy_idle_upgrade(&mut self) -> bool {
        self.player.buy_idle_upgrade()
    }

    fn apply_damage_pool(&mut self, mut damage: u64) {
        while damage > 0 {
            let current_health = u64::from(self.enemy.health.max(1));

            if damage < current_health {
                self.enemy.take_damage(damage as u32);
                break;
            }

            damage -= current_health;
            self.enemy.health = 0;
            self.defeat_enemy();
        }
    }

    fn defeat_enemy(&mut self) {
        let gold = self.enemy.gold_reward;
        let xp = self.enemy.xp_reward;

        self.player.add_gold(gold);
        self.player.add_xp(xp);
        self.player.add_kill();

        self.stage = self.stage.saturating_add(1).max(1);
        self.enemy = Enemy::for_stage(self.stage);
    }
}

fn main() {
    let mut game = Game::new();
    for _ in 0..100 {
        game.click();
    }
    let bought_click = game.buy_click_upgrade();
    let bought_idle = game.buy_idle_upgrade();
    game.tick(10);
    println!(
        "Stage: {}; level: {}; gold: {}; kills: {}",
        game.stage, game.player.level, game.player.gold, game.player.total_kills
    );
    println!(
        "Click upgrade: {bought_click}; idle upgrade: {bought_idle}; idle DPS: {}",
        game.player.idle_dps()
    );
}
