use crate::process::Process;
use std::io::Error;
use crate::game::entity::{Entity, Log};
use crate::offsets;
use crate::game::features::{aimbot, esp, Toggles};
// for esp
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};
use std::collections::HashMap;

// for toggles
use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState };

mod entity;
mod sig;
mod sigscanner;
mod features;

pub struct Game {
    pub process: Process,
    entities: Vec<Entity>,
    local_entity: Option<Entity>,
    toggles: HashMap<Toggles, bool>,
    pub sig_scanner: sigscanner::SigScanner,
}

impl Game {
    pub fn new(p_name: &str) -> Result<Game, Error> {
        // init process
        let process = Process::new(p_name)?;
        let mut sig_scanner = sigscanner::SigScanner::new();
        sig_scanner.cache_sigs(&process);

        Ok(Self {
            process,
            entities: Vec::new(),
            local_entity: None,
            toggles: HashMap::from([
                (Toggles::Aimbot, false),
                (Toggles::Esp, false),
            ]),
            sig_scanner,
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
        display: &Display<T>
    ) -> Result<(), Error> {

        self.cache_entites();
        self.cache_toggles();
        if *self.toggles.get(&Toggles::Aimbot).unwrap_or(&false) == true {
            aimbot::do_aimbot(&self)?;
        }
        if *self.toggles.get(&Toggles::Esp).unwrap_or(&false) == true {
            esp::draw_to_screen(display, &self);
        }
        Ok(())
    }

    fn cache_toggles(&mut self) {
        // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
        // up arrow
        unsafe {
            if GetAsyncKeyState(0x26) & 0x01 > 0 {
                self.toggles.insert(Toggles::Aimbot, !self.toggles.get(&Toggles::Aimbot).unwrap_or(&false));
            }
            // insert
            if GetAsyncKeyState(0x2D) & 0x01 > 0 {
                self.toggles.insert(Toggles::Esp, !self.toggles.get(&Toggles::Esp).unwrap_or(&false));
            }
        }
    }

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
