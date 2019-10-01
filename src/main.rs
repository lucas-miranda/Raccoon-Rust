mod game;
mod input;
mod math;
mod rendering;
mod window;

use game::{
    Game
};

fn main() {
    let mut game = match Game::new() {
        Ok(g) => g,
        Err(e) => panic!(e)
    };

    game.start();
}
