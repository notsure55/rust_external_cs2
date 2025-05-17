use std::io::{Error, ErrorKind};
use crate::game::Game;
use winit::window::{ WindowAttributes, WindowLevel };
use winit::raw_window_handle::{ HasWindowHandle };
use winit::dpi::{ Position::Logical, LogicalSize, LogicalPosition };
use core::ffi::c_void;
use windows::Win32::Foundation::HWND;

use crate::game::features::menu::Menu;

use rusttype as glium_text;

mod process;
mod offsets;
mod game;
mod math;
mod window;
mod rusttype;

// im kind of interested in building an external cheat, because i want to use rust
fn main() -> Result<(), Error> {

    let mut game = Game::new("Counter-Strike 2")?;

    let window_size = window::grab_window_dimensions(game.process.hwnd);
    let width = window_size.right - window_size.left - 15;
    let height = window_size.bottom - window_size.top - 40;

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

    let system = glium_text::TextSystem::new(&display);

    let font = glium_text::FontTexture::new(
        &display,
        &include_bytes!("../fonts/arialbd.ttf")[..], 70,
        glium_text::FontTexture::ascii_character_list()
    ).unwrap();

    let window_size = display.get_framebuffer_dimensions();

    let mut menu = Menu::new(display, window_size, system, font, &mut game.toggles);

    let hwnd: winit::platform::windows::HWND = handle.into();

    game.overlay_handle = HWND(hwnd as *mut c_void);

    window::make_window_click_through(game.overlay_handle);

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                    game.mouse_pos = (position.x as f32, position.y as f32);
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    game.run_cheat_loop(&mut menu).unwrap();
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
