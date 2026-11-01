use rustbound_idle::{Game, PurchaseOutcome};

fn main() {
    let mut game = Game::new();

    while game.stage <= 3 {
        let stage = game.stage;
        let level = game.player.level;

        game.click();

        if game.stage > stage {
            println!(
                "Stage {stage} cleared!  Gold: {}  XP: {}/{}",
                game.player.gold,
                game.player.xp,
                game.player.xp_needed()
            );

            if game.player.level > level {
                println!(
                    "Level up! Now level {}. Click damage: {}",
                    game.player.level,
                    game.player.click_damage()
                );
            }

            println!(
                "  Next up: {} with {} health.",
                game.enemy.name, game.enemy.health
            );
        }

        match game.buy_click_upgrade() {
            PurchaseOutcome::Purchased => println!(
                "Bought click upgrade {}! Click damage: {}  Gold left: {}",
                game.player.click_upgrade_level,
                game.player.click_damage(),
                game.player.gold
            ),
            PurchaseOutcome::NotEnoughGold => {}
        }
    }

    let save = game.save_json();
    println!("Saved: {save}");

    drop(game);

    let mut game = match Game::from_saved_json(&save) {
        Ok(game) => game,
        Err(error) => {
            println!("Could not load the save: {error:?}");
            return;
        }
    };

    println!("Loaded. Stage {}, level {}.", game.stage, game.player.level);

    let stage = game.stage;
    let level = game.player.level;
    let gold = game.player.gold;

    game.tick(3600);

    println!("One hour away.");
    println!(
        "Stage {} -> {}  Level {} -> {}  Gold {} -> {}",
        stage, game.stage, level, game.player.level, gold, game.player.gold
    );
}
