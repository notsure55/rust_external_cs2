use std::io::{Error, ErrorKind};
use crate::game::Game;
use winit::window::{ WindowAttributes, WindowLevel };
use winit::raw_window_handle::{ HasWindowHandle };
use winit::dpi::{ Position::Logical, LogicalSize, LogicalPosition };

mod process;
mod offsets;
mod game;
mod math;
mod window;

// im kind of interested in building an external cheat, because i want to use rust
fn main() -> Result<(), Error> {

    let mut game = Game::new("Counter-Strike 2")?;

    let window_size = window::grab_window_dimensions(game.process.hwnd);
    println!("{}, {}", window_size.left, window_size.right);
    let width = window_size.right - window_size.left - 15;
    let height = window_size.bottom - window_size.top - 40;

    //thread::sleep(time::Duration::from_millis(5));

    #[allow(deprecated)]
    let window_attributes = WindowAttributes::new()
        .with_title("Hack Overlay")
        .with_inner_size(LogicalSize::new(width as f32, height as f32))
        .with_position(Logical(LogicalPosition::new(window_size.left.into(), window_size.top.into())))
        .with_transparent(true)
        .with_window_level(WindowLevel::AlwaysOnTop);

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);

    let window_handle = match window.window_handle() {
        Ok(wh) => wh,
        Err(_e) => return Err(Error::new(ErrorKind::Other, "Raw Window handle is invald!")),
    };

    let handle = match window::grab_handle(window_handle) {
        Some(h) => h,
        None => return Err(Error::new(ErrorKind::Other, "HWND is invald!")),
    };

    window::make_window_click_through(handle.into());

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    game.run_cheat_loop(&display).unwrap();
                    window.request_redraw()
                },
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();

    Ok(())
}
