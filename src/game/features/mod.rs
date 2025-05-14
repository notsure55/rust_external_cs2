pub mod aimbot;
pub mod esp;

#[derive(Eq, Hash, PartialEq)]
pub enum Toggles {
    Aimbot,
    Esp,
}
