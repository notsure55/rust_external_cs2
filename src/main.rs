use std::io;
use std::{thread, time};
use crate::game::Game;
use glfw::{Action, Context, Key};

mod process;
mod offsets;
mod game;
mod math;
mod window;

// im kind of interested in building an external cheat, because i want to use rust
fn main() -> Result<(), io::Error> {

    let mut game = Game::new("Counter-Strike 2")?;

    let (mut glfw, mut window) = window::init_window();

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();

        game.run_cheat_loop()?;

        window.swap_buffers();

        thread::sleep(time::Duration::from_millis(5));
    }

    Ok(())
}
