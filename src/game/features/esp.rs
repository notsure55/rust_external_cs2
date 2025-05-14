use glium::{ Surface, implement_vertex, uniform, Frame };
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};

use crate::game::Game;
use crate::game::entity::Entity;
use crate::game::Toggles;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn draw_box<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    height: f32,
    width: f32,
    window_size: (u32, u32)
    ) {

    let uniforms = uniform! {
        screen_size: [window_size.0 as f32, window_size.1 as f32]
    };

    let shape = vec![
        Vertex { position: [ top_left.position[0], top_left.position[1] ] },
        Vertex { position: [ top_left.position[0] + width, top_left.position[1]] },
        Vertex { position: [ top_left.position[0] + width, top_left.position[1] + height] },
        Vertex { position: [ top_left.position[0], top_left.position[1] + height] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform vec2 screen_size;

        void main() {
        vec2 zero_to_one = position / screen_size;
        vec2 zero_to_two = zero_to_one * 2.0;
        vec2 clip_space = zero_to_two - 1.0;
        clip_space.y = -clip_space.y;

        gl_Position = vec4(clip_space, 0.0, 1.0);
        }
        "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
               &Default::default()).unwrap();
}

pub fn draw_to_screen<T: SurfaceTypeTrait + ResizeableSurface + 'static>(display: &Display<T>, game: &Game) {

    let mut frame = display.draw();

    let window_size = display.get_framebuffer_dimensions();

    frame.clear_color(0.0, 0.0, 0.0, 0.0);

    if game.toggles.esp {
        for entity in game.entities.iter() {
            match entity {
                Entity::Player(ent) => {
                    let (head_pos, feet_pos) = ent.m_pawn.pos();
                    let head_2d = match head_pos.wts(game, window_size) {
                        Some(head) => head,
                        None => continue,
                    };
                    let feet_2d = match feet_pos.wts(game, window_size) {
                        Some(feet) => feet,
                        None => continue,
                    };

                    let scalar = feet_2d.v[1] - head_2d.v[1];
                    let height = scalar * 1.20;
                    let width = scalar * 0.70;
                    let top_left = Vertex{ position: [head_2d.v[0] - scalar * 0.30, head_2d.v[1] - scalar * 0.10] };

                    draw_box(display, &mut frame, top_left, height, width, window_size);
                },
            }
        }
    }

    frame.finish().unwrap();
}
