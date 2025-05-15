use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState };
use windows::Win32::Foundation::HWND;
use crate::window;
use crate::game::Game;

pub mod aimbot;
pub mod esp;

pub struct AimbotToggles {
    pub fov_toggle: bool,
    pub fov_slider: f32,
    pub smooth_toggle: bool,
    pub smooth_slider: f32,
}

pub struct EspToggles {
    pub boxes: bool,
    pub health_bars: bool,
}

pub struct Toggles {
    // aimbot bools
    pub aimbot: bool,
    pub aimbot_toggles: AimbotToggles,
    // menu
    pub menu: bool,
    // esp bools
    pub esp: bool,
    pub esp_toggles: EspToggles,
}

impl Toggles {
    pub fn new() -> Self {
        Self {
            aimbot: false,
            aimbot_toggles: AimbotToggles {
                fov_toggle: true,
                fov_slider: 0.0,
                smooth_toggle: true,
                smooth_slider: 50.0
            },
            menu: false,
            esp: true,
            esp_toggles: EspToggles {
                boxes: true,
                health_bars: true,
            },
        }
    }
    pub fn cache_toggles(&mut self, handle: &HWND) {
        // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
        // up arrow
        unsafe {
            if GetAsyncKeyState(0x26) & 0x01 > 0 {
                self.aimbot = !self.aimbot;
            }
            // insert
            if GetAsyncKeyState(0x2D) & 0x01 > 0 {
                self.menu = !self.menu;
                if self.menu {
                    window::make_window_non_click_through(*handle);
                } else {
                    window::make_window_click_through(*handle);
                }
            }
        }
    }
}
