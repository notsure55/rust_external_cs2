use glfw::{Action, Context, Key};
use core::ffi::c_void;
use windows::{
    Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT},
    Win32::Foundation::HWND,
};

pub fn init_window() -> (glfw::Glfw, glfw::PWindow) {

    let mut glfw = glfw::init_no_callbacks().unwrap();

    unsafe {
        // sets window transpareny to true
        // https://www.glfw.org/docs/3.3/group__window.html#ga60a0578c3b9449027d683a9c6abb9f14
        glfw::ffi::glfwWindowHint(0x0002000A, 1);
        // set window always on top
        glfw::ffi::glfwWindowHint(0x00020007, 1)
    }

    let (mut window, events) = glfw
        .create_window(1000, 1000, "Window Test", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    make_window_click_through(&window);

    window.set_key_polling(true);
    window.make_current();
    window.set_decorated(false);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    (glfw, window)
}

fn make_window_click_through(window: &glfw::Window) {
    let hwnd = HWND(window.get_win32_window() as *mut c_void);

    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32);
    }
}
