use crate::game::{Game, entity::Pos};
use std::io::{Error, ErrorKind};

fn calculate_angle() {

}

pub fn find_closest_entity(game: &Game) -> Result<(), Error> {
    for entity in game.entities.iter() {
        let entity_pos = entity.pos();

        let local_entity = match &game.local_entity {
            Some(entity) => entity,
            None => return Err(Error::new(ErrorKind::Other, "Invalid Controller!"))
        };

        let local_entity_pos = local_entity.pos();
    }

    Ok(())
}
