use glium::{ Surface, implement_vertex, uniform, Frame };
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};
use crate::game::Game;
use crate::game::features::esp::Vertex;

fn draw_filled_box<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    width: f32,
    height: f32,
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
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

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
        color = vec4(0.2, 0.2, 0.2, 1.0);
        }
        "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
               &Default::default()).unwrap();
}

fn draw_check_box<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    window_size: (u32, u32),
    toggle: &mut bool,
    clicked: bool,
    mouse_pos: (f32, f32)
    ) {
    let width = 35.0;
    let height = 35.0;
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
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

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
        color = vec4(0.4, 0.4, 0.4, 1.0);
        }
        "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
               &Default::default()).unwrap();

    if mouse_pos.0 > top_left.position[0] + width || mouse_pos.0 < top_left.position[0]
    || mouse_pos.1 > top_left.position[1] + height || mouse_pos.1 < top_left.position[1] {
    } else {
        if clicked {
            *toggle = !*toggle;
        }
    }
    if *toggle {
        draw_check(display, frame, top_left, window_size);
    }
}

fn draw_check<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    window_size: (u32, u32)
    ) {
    let uniforms = uniform! {
        screen_size: [window_size.0 as f32, window_size.1 as f32]
    };

    let shape = vec![
        Vertex { position: [ top_left.position[0] + 2.0, top_left.position[1] + 20.0 ] },
        Vertex { position: [ top_left.position[0] + 15.0, top_left.position[1] + 30.0] },
        Vertex { position: [ top_left.position[0] + 33.0, top_left.position[1] + 5.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);

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

    let params = glium::DrawParameters {
        line_width: Some(4.0),
        .. Default::default()
    };

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
               &params).unwrap();
}

pub fn render_menu<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    window_size: (u32, u32),
    game: &mut Game,
) {
    // draw main box
    draw_filled_box(display,
                    frame,
                    Vertex { position: [ 100.0, 100.0 ] },
                    600.0,
                    450.0,
                    window_size
    );
    draw_check_box(display,
                   frame,
                   Vertex { position: [ 115.0, 115.0 ] },
                   window_size,
                   &mut game.toggles.esp,
                   game.toggles.clicked,
                   game.mouse_pos,
    );
}
