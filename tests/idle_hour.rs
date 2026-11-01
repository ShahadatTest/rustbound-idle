use rustbound_idle::Game;

#[test]
fn an_hour_away_moves_the_game_forward() {
    let mut game = Game::new();

    game.tick(3600);

    assert_eq!(game.stage, 128);
    assert_eq!(game.player.level, 47);
}
