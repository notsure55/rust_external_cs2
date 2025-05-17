use glium::{ Surface, uniform, Frame };
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};

use crate::game::Game;
use crate::game::features::esp::Vertex;
use crate::math::Vec4;

use crate::rusttype as glium_text;

fn is_clicked(
    mouse_pos: (f32, f32),
    top_left: Vertex,
    width: f32,
    height: f32,
    clicked: bool
) -> bool {
    if mouse_pos.0 < top_left.position[0] + width && mouse_pos.0 > top_left.position[0]
    && mouse_pos.1 < top_left.position[1] + height && mouse_pos.1 > top_left.position[1] {
        if clicked {
            return true
        } else {
            return false
        }
    } else {
        return false
    }
}

pub fn draw_box<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
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

    let params = glium::DrawParameters {
        line_width: Some(2.0),
        .. Default::default()
    };

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
               &params).unwrap();
}

pub fn draw_text<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    window_size: (u32, u32),
    text: &str,
    game: &Game,
    system: &glium_text::TextSystem,
    font: &glium_text::FontTexture,
    scale: f32,
    color: Vec4
) {
    let text = glium_text::TextDisplay::new(system, font, text);
    let text_width = text.get_width();

    let sx = scale / (window_size.0 as f32 / 2.0);
    let sy = scale / (window_size.1 as f32 / 2.0);

    let x_ndc = (top_left.position[0] / window_size.0 as f32) * 2.0 - 1.0;
    let y_ndc = -((top_left.position[1] / window_size.1 as f32) * 2.0 - 1.0);

    let matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        sx,  0.0, 0.0, 0.0,
        0.0, sy,  0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x_ndc, y_ndc, 0.0, 1.0,
    ).into();

    glium_text::draw(&text, &system, frame, matrix, color.v.into()).unwrap();
}


pub fn draw_filled_box<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    width: f32,
    height: f32,
    window_size: (u32, u32),
    color_input: Vec4
) {
    let uniforms = uniform! {
        screen_size: [window_size.0 as f32, window_size.1 as f32],
        color_input: [color_input.v[0], color_input.v[1], color_input.v[2], color_input.v[3]]
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

        uniform vec4 color_input;
        out vec4 color;

        void main() {
        color = color_input;
        }
        "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
               &Default::default()).unwrap();
}

fn is_hovering<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    top_left: Vertex,
    width: f32,
    height: f32,
    window_size: (u32, u32),
    mouse_pos: (f32, f32)
) {
    if mouse_pos.0 < top_left.position[0] + width && mouse_pos.0 > top_left.position[0]
    && mouse_pos.1 < top_left.position[1] + height && mouse_pos.1 > top_left.position[1] {
        let top_left = Vertex { position: [ top_left.position[0] - 2.0, top_left.position[1] - 2.0] };
        draw_box(display, frame, top_left, width + 4.0, height + 4.0, window_size);
    }
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

    // if mouse is witin checkbox
    is_hovering(display, frame, top_left, width, height, window_size, mouse_pos);

    if is_clicked(mouse_pos, top_left, width, height, clicked) {
        *toggle = !*toggle;
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

static mut BASE: Vertex = Vertex { position: [100.0, 100.0] };

fn calc_base(game: &Game, width: f32, height: f32) -> Vertex {
    unsafe {
        if is_clicked(game.mouse_pos, BASE, width, height, game.toggles.dragging) {
            BASE.position[0] += game.mouse_pos.0 - BASE.position[0] - 100.0;
            BASE.position[1] += game.mouse_pos.1 - BASE.position[1] - 100.0;
        }
        return BASE
    }
}

pub fn render_menu<T: SurfaceTypeTrait + ResizeableSurface + 'static>(
    display: &Display<T>,
    frame: &mut Frame,
    window_size: (u32, u32),
    game: &mut Game,
    system: &glium_text::TextSystem,
    font: &glium_text::FontTexture
) {
    const WIDTH: f32 = 600.0;
    const HEIGHT: f32 = 450.0;
    let base = calc_base(game, WIDTH, HEIGHT);
    // draw main box
    draw_filled_box(display,
                    frame,
                    base,
                    WIDTH,
                    HEIGHT,
                    window_size,
                    Vec4::new(0.2, 0.2, 0.2, 1.0)
    );
    // draw esp checkbox
    draw_check_box(display,
                   frame,
                   Vertex { position: [ base.position[0] + 15.0, base.position[1] + 15.0 ] },
                   window_size,
                   &mut game.toggles.esp,
                   game.toggles.clicked,
                   game.mouse_pos,
    );
}
