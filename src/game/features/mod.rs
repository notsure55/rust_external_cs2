use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState, SendInput, INPUT, INPUT_TYPE, MOUSE_EVENT_FLAGS, INPUT_0 };
use windows::Win32::Foundation::HWND;
use crate::window;

pub mod aimbot;
pub mod esp;
pub mod menu;

#[allow(unused)]
pub struct AimbotToggles {
    pub fov_toggle: bool,
    pub fov_slider: f32,
    pub smooth_toggle: bool,
    pub smooth_slider: f32,
}

pub struct EspToggles {
    pub boxes: bool,
    pub health_bars: bool,
    pub names: bool,
}

#[allow(unused)]
pub struct Toggles {
    pub clicked: bool,
    // for dragging mouse
    pub dragging: bool,
    pub cached_mouse_pos: (f32, f32),
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
            clicked: false,
            dragging: false,
            cached_mouse_pos: (0.0, 0.0),
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
                names: true,
            },
        }
    }
    pub fn cache_toggles(&mut self, handle: &HWND, mouse_pos: (f32, f32)) {
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
                // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
                let mut inputs = [INPUT { r#type: INPUT_TYPE(0), Anonymous: INPUT_0::default() }; 2];

                inputs[0].r#type = INPUT_TYPE(0);
                inputs[0].Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0x0002);

                inputs[1].r#type = INPUT_TYPE(0);
                inputs[1].Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0x0004);

                SendInput(&inputs, std::mem::size_of_val::<INPUT>(&inputs[0]) as i32);
            }
            // reset for next loop if not clicked again
            if self.clicked {
                self.clicked = false;
            }
            if GetAsyncKeyState(0x01) & 0x01 > 0 {
                self.clicked = true;
            }
            if GetAsyncKeyState(0x02) < 0 {
                if !self.dragging {
                    self.dragging = true;
                    self.cached_mouse_pos = mouse_pos;
                }
            } else {
                self.dragging = false;
            }
        }
    }
}
