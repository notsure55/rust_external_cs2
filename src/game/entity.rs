use crate::offsets;
use crate::game::Game;
use crate::math::Vec3;

use std::io::{Error, ErrorKind};

pub trait Log {
    fn log(&self);
}

pub trait Pos {
    fn pos(&self) -> Vec3;
}

pub struct Player {
    pub m_pawn: Pawn,
    pub m_controller: Controller,
}

pub struct Pawn {
    address: usize,
    health: i32,
    head: Vec3,
    feet: Vec3,
    yaw: f32,
    pitch: f32
}

pub struct Controller {
    pub address: usize,
    name: String
}

impl Log for Player {
    fn log(&self) {
        println!("Health: {}, Name: {}, Address: {:X}", self.m_pawn.health, self.m_controller.name, self.m_pawn.address);
    }
}

impl Pos for Player {
    fn pos(&self) -> Vec3 {
        self.m_pawn.head
    }
}

impl Controller {
    pub fn new(game: &Game ,entity_list_address: usize, i: usize) -> Result<Self, Error> {
        let entity_list_entry: usize = game.process.read(entity_list_address + 8 * (i >> 9) + 16)?;
        if entity_list_entry == 0 {
            return Err(Error::new(ErrorKind::Other, "Invalid Controller!"));
        }

        let base_entity: usize = game.process.read(120 * (i & 0x1FF) + entity_list_entry)?;
        if base_entity == 0 {
            return Err(Error::new(ErrorKind::Other, "Invalid Controller!"));
        }

        let name = game.process.read_buffer(base_entity + offsets::PLAYER_NAME_OFFSET, 128)?;

        let first_null_byte = name.iter().position(|&b| b == 0).unwrap_or(name.len());

        let s = String::from_utf8_lossy(&name[..first_null_byte]);

        Ok(Self {
            address: base_entity,
            name: String::from(s)
        })
    }
}

impl Pawn {
    pub fn pos(&self) -> (Vec3, Vec3){
        (self.head, self.feet)
    }
    pub fn new(game: &Game, entity_list_address: usize, controller_address: usize) -> Result<Self, Error> {

        let pawn_handle: usize = game.process.read(controller_address + offsets::PLAYER_PAWN_OFFSET)?;

        let pawn_entry: usize = game.process.read(entity_list_address + 8 * ((pawn_handle & 0x7FFF) >> 9) + 16)?;

        let player_pawn: usize = game.process.read(120 * (pawn_handle & 0x1FF) + pawn_entry)?;

        let health: i32 = game.process.read(player_pawn + offsets::HEALTH_OFFSET)?;

        if health > 100 || health <= 0 {
            return Err(Error::new(ErrorKind::Other, "Invalid Player!"));
        }

        let head: Vec3 = game.process.read(player_pawn + offsets::HEAD_OFFSET)?;
        let feet: Vec3 = game.process.read(player_pawn + offsets::FEET_OFFSET)?;

        let pitch: f32 = game.process.read(player_pawn + offsets::EYE_ANGLES)?;
        let yaw: f32 = game.process.read(player_pawn + offsets::EYE_ANGLES + 0x4)?;

        Ok(Self {
            address: player_pawn,
            health,
            head,
            feet,
            yaw,
            pitch
        })
    }
}

impl Player {
    pub fn new(game: &Game, address: usize, index: usize) -> Result<Self, Error> {
        let controller = Controller::new(game, address, index)?;
        let pawn = Pawn::new(game, address, controller.address)?;

        Ok(Self {
            m_pawn: pawn,
            m_controller: controller
        })
    }
}

pub enum Entity {
    Player(Player)
}

impl Log for Entity {
    fn log(&self) {
        match self {
            Entity::Player(player) => player.log(),
        }
    }
}

impl Pos for Entity {
    fn pos(&self) -> Vec3 {
        match self {
            Entity::Player(player) => player.pos(),
        }
    }
}
