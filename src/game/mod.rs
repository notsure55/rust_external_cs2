use crate::process::Process;
use std::io::Error;
use crate::game::entity::{Entity, Log};
use crate::offsets;
use crate::game::features::{aimbot, esp};
// for esp
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};

mod entity;
mod sig;
mod sigscanner;
mod features;

pub struct Game {
    pub process: Process,
    entities: Vec<Entity>,
    local_entity: Option<Entity>,
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
        aimbot::do_aimbot(&self)?;
        esp::draw_to_screen(display, &self);
        Ok(())
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
