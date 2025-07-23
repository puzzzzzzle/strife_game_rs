use strife_game_core::{Game, GameConfig};

#[test]
fn test_run_game() {
    let config = GameConfig {};
    let game = Game::new(config).unwrap();
}
