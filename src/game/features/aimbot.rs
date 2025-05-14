use crate::game::{Game, entity::Pos};
use std::io::{Error, ErrorKind};
use crate::math::{ViewAngles};
use crate::offsets;

fn find_closest_entity(game: &Game) -> Result<ViewAngles, Error> {
    let mut angle = ViewAngles::default();

    for entity in game.entities.iter() {
        let entity_pos = entity.pos();

        let local_entity = match &game.local_entity {
            Some(entity) => entity,
            None => return Err(Error::new(ErrorKind::Other, "Local player isnt cached!"))
        };

        let local_entity_pos = local_entity.pos();

        angle = local_entity_pos.calculate_angle(entity_pos);
    }

    Ok(angle)
}

pub fn do_aimbot(game: &Game) -> Result<(), Error> {
    let angle = find_closest_entity(game)?;

    game.process.write(
        (game.process.modules.get("client.dll").unwrap().lpBaseOfDll as usize)
         + offsets::LOCAL_PLAYER_VIEW_ANGLES, angle
    )?;

    Ok(())
}
