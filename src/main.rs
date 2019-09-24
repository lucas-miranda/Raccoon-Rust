mod core;
mod input;
mod window;

use crate::core::{
    Game
};

fn main() {
    let mut game = match Game::new() {
        Ok(g) => g,
        Err(e) => panic!(e)
    };

    game.start();
}
