use glium::{ implement_vertex, Frame };
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};

use crate::game::{Game, entity::Entity };
use crate::math::Vec4;

use super::menu::Menu;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub fn render_esp<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    menu: &mut Menu<T>,
    frame: &mut Frame,
    game: &Game
) {
    for entity in game.entities.iter() {
        match entity {
            Entity::Player(ent) => {
                let (head_pos, feet_pos) = ent.m_pawn.pos();
                let head_2d = match head_pos.wts(game, menu.window_size) {
                    Some(head) => head,
                    None => continue,
                };
                let feet_2d = match feet_pos.wts(game, menu.window_size) {
                    Some(feet) => feet,
                    None => continue,
                };

                let scalar = feet_2d.v[1] - head_2d.v[1];
                let height = scalar * 1.20;
                let width = scalar * 0.70;
                let top_left = Vertex{ position: [head_2d.v[0] - scalar * 0.30, head_2d.v[1] - scalar * 0.10] };

                /*if game.toggles.esp_toggles.boxes {
                    menu.draw_box(
                        frame,
                        top_left,
                        width,
                        height,
                    );
                }

                if game.toggles.esp_toggles.names {
                    let text_top_left = Vertex { position: [top_left.position[0],
                                                            top_left.position[1] - scalar * 0.30] };
                    menu.draw_text(
                        frame,
                        top_left,
                        &ent.m_controller.name,
                        scalar * 0.25,
                        Vec4::new(0.0, 1.0, 1.0, 1.0)
                    );
                }

                if game.toggles.esp_toggles.health_bars {
                    draw_health_bars(
                        menu,
                        frame,
                        top_left,
                        ent.m_pawn.health,
                        height,
                        scalar
                    );
                }*/
            },
        }
    }
}

fn draw_health_bars<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    menu: &mut Menu<T>,
    frame: &mut Frame,
    top_left: Vertex,
    health: i32,
    height: f32,
    scalar: f32
) {
    let health_scalar = -(health as f32 - 100.0) * 0.01;

    let health_top_left = Vertex { position: [ top_left.position[0] - scalar * 0.10,
                                               top_left.position[1] + scalar * health_scalar ] };

    let health_width = top_left.position[0] - health_top_left.position[0] - scalar * 0.02;

    let health_height = height - scalar * health_scalar;

    /*menu.draw_filled_box(
        frame,
        health_top_left,
        health_width,
        health_height,
        Vec4::new(0.0 + health_scalar, 1.0 - health_scalar, 0.0, 1.0)
    );*/
}
