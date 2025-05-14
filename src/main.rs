use std::io::{Error, ErrorKind};
use std::{thread, time};
use crate::game::Game;
use glium::Surface;
use winit::window::{ WindowAttributes, WindowLevel };
use winit::raw_window_handle::{ Win32WindowHandle, RawWindowHandle, HasWindowHandle };
use winit::platform::windows::HWND;
use core::num::NonZeroIsize;

mod process;
mod offsets;
mod game;
mod math;
mod window;

// im kind of interested in building an external cheat, because i want to use rust
fn main() -> Result<(), Error> {

    let mut game = Game::new("Counter-Strike 2")?;

    //game.run_cheat_loop()?;

    //thread::sleep(time::Duration::from_millis(5));

    let window_attributes = WindowAttributes::new()
        .with_title("Hack Overlay")
        .with_transparent(true)
        .with_window_level(WindowLevel::AlwaysOnTop);

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);

    let window_handle = match _window.window_handle() {
        Ok(wh) => wh,
        Err(_e) => return Err(Error::new(ErrorKind::Other, "Window HWND is invald!")),
    };

    let raw_handle = window_handle.as_raw();

    let win32_handle: Win32WindowHandle = if let RawWindowHandle::Win32(hwnd) = raw_handle {
        hwnd
    } else {
        return Err(Error::new(ErrorKind::Other, "Wrong raw handle returned!"))
    };

    let handle = win32_handle.hwnd;

    window::make_window_click_through(handle.into());

    let mut frame = display.draw();

    frame.clear_color(0.0, 0.0, 0.0, 0.0);

    frame.finish().unwrap();

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();


    Ok(())
}
