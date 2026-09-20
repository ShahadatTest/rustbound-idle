use rustbound_idle::Game;

fn main() {
    let mut game = Game::default();
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
    let saved = game.save_json().expect("game should serialize");
    let restored = Game::from_saved_json(&saved).expect("save should restore");
    assert_eq!(restored, game);
    println!("Save restored successfully.");
}
