use rustbound_idle::Game;

#[test]
fn a_restored_game_plays_the_same_hour() {
    let mut original = Game::new();
    original.tick(600);

    let mut restored = Game::from_saved_json(&original.save_json()).expect("the save should load");

    original.tick(3600);
    restored.tick(3600);

    assert_eq!(restored.stage, original.stage);
    assert_eq!(restored.enemy.health, original.enemy.health);
    assert_eq!(restored.player.gold, original.player.gold);
    assert_eq!(restored.player.xp, original.player.xp);
    assert_eq!(restored.player.level, original.player.level);
}
