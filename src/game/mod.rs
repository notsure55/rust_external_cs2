use crate::process::Process;
use std::io::Error;
use crate::game::entity::{Entity, Log};
use crate::offsets;
use crate::game::features::{aimbot, esp, Toggles, menu};
// for esp
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};
use windows::Win32::Foundation::HWND;
use core::ffi::c_void;
use glium::Surface;

use crate::rusttype as glium_text;

mod entity;
mod sig;
mod sigscanner;
mod features;

pub struct Game {
    pub process: Process,
    pub overlay_handle: HWND,
    entities: Vec<Entity>,
    local_entity: Option<Entity>,
    toggles: Toggles,
    pub sig_scanner: sigscanner::SigScanner,
    pub mouse_pos: (f32, f32),
}

impl Game {
    pub fn new(p_name: &str) -> Result<Game, Error> {
        // init process
        let process = Process::new(p_name)?;
        let mut sig_scanner = sigscanner::SigScanner::new();
        sig_scanner.cache_sigs(&process);

        Ok(Self {
            process,
            overlay_handle: HWND(0 as *mut c_void),
            entities: Vec::new(),
            local_entity: None,
            toggles: Toggles::new(),
            sig_scanner,
            mouse_pos: (0.0, 0.0),
        })
    }
    pub fn cache_entites(&mut self) {
        let entity_list_address: usize = self.process.read(
        *self.sig_scanner.cached_sigs.get(
            "CGameEntitySystem"
        ).unwrap()).unwrap();

        self.entities = Vec::new();

        for i in 1..=64 {

            if i >> 9 > 63 {
                continue;
            }

            let player = match entity::Player::new(self, entity_list_address, i) {
                Ok(p) => p,
                Err(_) => continue
            };

            let player_is_local_player = self.process.read(
                player.m_controller.address + offsets::IS_LOCAL_PLAYER
            ).unwrap_or(0);

            if player_is_local_player == 1 {
                self.local_entity = Some(Entity::Player(player));
                continue
            }

            self.entities.push(Entity::Player(player));
        }
    }

    pub fn run_cheat_loop<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
        &mut self,
        display: &Display<T>,
        system: &glium_text::TextSystem,
        font: &glium_text::FontTexture
    ) -> Result<(), Error> {

        self.cache_entites();
        self.toggles.cache_toggles(&self.overlay_handle);

        if self.toggles.aimbot {
            aimbot::do_aimbot(&self)?;
        }

        self.draw_to_screen(display, system, font);

        Ok(())
    }

    fn draw_to_screen<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
        &mut self,
        display: &Display<T>,
        system: &glium_text::TextSystem,
        font: &glium_text::FontTexture
    ) {
        // for storing text glyphs for drawing

        let mut frame = display.draw();

        let window_size = display.get_framebuffer_dimensions();

        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        if self.toggles.esp {
            esp::render_esp(
                display,
                &mut frame,
                window_size,
                &self,
                system,
                font
            );
        }
        if self.toggles.menu {
            menu::render_menu(
                display,
                &mut frame,
                window_size,
                self,
                system,
                font
            );
        }

        frame.finish().unwrap();
    }
    #[allow(unused, dead_code)]
    pub fn print_entities(&self) {

        match &self.local_entity {
            Some(entity) => entity.log(),
            None => (),
        };

        for player in self.entities.iter() {
            player.log();
        }
    }
}
