use core::ffi::c_void;
use windows::{
    Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT},
    Win32::Foundation::HWND,
};

use winit::platform;

pub fn make_window_click_through(winit_handle: platform::windows::HWND) {
    let hwnd = HWND(winit_handle as *mut c_void);

    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32);
    }
}
