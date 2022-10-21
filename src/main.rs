mod game;
use game::Game;

fn main() {
    let game = Game::new();
    game.start();
}
