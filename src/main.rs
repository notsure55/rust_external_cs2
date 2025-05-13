use std::io;
use std::{thread, time};
use crate::game::Game;

mod process;
mod offsets;
mod game;

// im kind of interested in building an external cheat, because i want to use rust
fn main() -> Result<(), io::Error> {

    let mut game = Game::new("Counter-Strike 2")?;

    loop {
        game.cache_entites();
        game.print_entities();
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}
