use std::io;
use std::{thread, time};
use crate::game::Game;

mod process;
mod offsets;
mod game;
mod math;

// im kind of interested in building an external cheat, because i want to use rust
fn main() -> Result<(), io::Error> {

    let mut game = Game::new("Counter-Strike 2")?;

    loop {
        game.run_cheat_loop()?;
        thread::sleep(time::Duration::from_millis(5));
    }

    Ok(())
}
