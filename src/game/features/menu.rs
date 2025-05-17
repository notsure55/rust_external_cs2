use glium::{ Surface, uniform, Frame };
use glium::backend::glutin::{Display};
use glutin::surface::{SurfaceTypeTrait, ResizeableSurface};
use std::io::{ Result, Error };

use crate::game::{ Game, features::Toggles };
use crate::game::features::esp::Vertex;
use crate::math::Vec4;

use crate::rusttype as glium_text;

pub struct Rect {
    pub top_left: Vertex,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(top_left: Vertex, width: f32, height: f32) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }

}

pub trait Draw {
    fn draw<T: SurfaceTypeTrait + ResizeableSurface + 'static>(&self, frame: &mut Frame, menu: &mut Menu<T>);
}

struct Box {
    rect: Rect,
    color: Vec4,
}

impl Box {
    pub fn new(rect: Rect, color: Vec4) -> Self {
        Self {
            rect,
            color,
        }
    }
}

struct CheckBox<'a> {
    top_left: Vertex,
    color: Vec4,
    toggle: &'a mut bool,
}

impl<'a> CheckBox<'a> {
    pub fn new(top_left: Vertex, color: Vec4, toggle: &'a mut bool) -> Self {
        Self {
            top_left,
            color,
            toggle
        }
    }
}

pub enum MenuObject<'a> {
    CheckBox(CheckBox<'a>),
    FilledBox(Box),
    BoxOutline(Box),
}

impl<'a> Draw for MenuObject<'a> {
    fn draw<T: SurfaceTypeTrait + ResizeableSurface + 'static>(&self, frame: &mut Frame, menu: &mut Menu<T>) {
        match self {
            MenuObject::CheckBox(check_box) => menu.draw_check_box(frame, check_box),
            MenuObject::FilledBox(filled_box) => menu.draw_filled_box(frame, filled_box),
            MenuObject::BoxOutline(outline_box) => menu.draw_box(frame, outline_box),
        }
    }
}

pub struct Menu<'a, T>
where T: SurfaceTypeTrait + ResizeableSurface + 'static
{
    pub display: Display<T>,
    pub window_size: (u32, u32),
    system: glium_text::TextSystem,
    font: glium_text::FontTexture,
    base: Vertex,
    objects: Vec<MenuObject<'a>>,
}

impl<'a, T: SurfaceTypeTrait + ResizeableSurface + 'static> Menu<'a, T> {
    pub fn new (
        display: Display<T>,
        window_size: (u32, u32),
        system: glium_text::TextSystem,
        font: glium_text::FontTexture,
        toggles: &'a mut Toggles
    ) -> Self
    {
        let mut objects = Menu::<T>::build_menu(toggles).unwrap();

        Self {
            display,
            window_size,
            system,
            font,
            base: Vertex { position: [ 100.0, 100.0 ] },
            objects,
        }
    }
    fn build_menu(toggles: &mut Toggles) -> Result<Vec<MenuObject>> {
        let mut objects = Vec::new();

        let base = Vertex { position: [ 100.0, 100.0 ] };

        let filled_box = Box::new(
            Rect::new(Vertex { position: [100.0, 100.0] }, 600.0, 450.0 ),
            Vec4::new(0.4, 0.4, 0.4, 1.0)
        );
        objects.push(MenuObject::FilledBox(filled_box));
        let check_box = CheckBox::new(
            Vertex { position: [base.position[0] + 15.0, base.position[1] + 15.0] },
            Vec4::new(1.0, 0.5, 0.0, 1.0),
            &mut toggles.esp
        );
        objects.push(MenuObject::CheckBox(check_box));
        Ok(objects)
    }
    fn is_clicked(
        &self,
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

    pub fn draw_box(
        &mut self,
        frame: &mut Frame,
        b: &Box,
    ) {

        let uniforms = uniform! {
            screen_size: [self.window_size.0 as f32, self.window_size.1 as f32],

        };

        let shape = vec![
            Vertex { position: [ b.rect.top_left.position[0], b.rect.top_left.position[1] ] },
            Vertex { position: [ b.rect.top_left.position[0] + b.rect.width, b.rect.top_left.position[1]] },
            Vertex { position: [ b.rect.top_left.position[0] + b.rect.width, b.rect.top_left.position[1] + b.rect.height] },
            Vertex { position: [ b.rect.top_left.position[0], b.rect.top_left.position[1] + b.rect.height] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
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

        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &params
        ).unwrap();
    }

    pub fn draw_text(
        &mut self,
        frame: &mut Frame,
        top_left: Vertex,
        text: &str,
        scale: f32,
        color: Vec4
    ) {
        let text = glium_text::TextDisplay::new(&self.system, &self.font, text);
        let text_width = text.get_width();

        let sx = scale / (self.window_size.0 as f32 / 2.0);
        let sy = scale / (self.window_size.1 as f32 / 2.0);

        let x_ndc = (top_left.position[0] / self.window_size.0 as f32) * 2.0 - 1.0;
        let y_ndc = -((top_left.position[1] / self.window_size.1 as f32) * 2.0 - 1.0);

        let matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
            sx,  0.0, 0.0, 0.0,
            0.0, sy,  0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x_ndc, y_ndc, 0.0, 1.0,
        ).into();

        glium_text::draw(
            &text,
            &self.system,
            frame,
            matrix,
            color.v
                .into()
        ).unwrap();
    }

    pub fn draw_filled_box(
        &mut self,
        frame: &mut Frame,
        b: &Box,
    ) {
        let uniforms = uniform! {
            screen_size: [self.window_size.0 as f32, self.window_size.1 as f32],
            color_input: [b.color.v[0], b.color.v[1], b.color.v[2], b.color.v[3]]
        };

        let shape = vec![
            Vertex { position: [ b.rect.top_left.position[0], b.rect.top_left.position[1] ] },
            Vertex { position: [ b.rect.top_left.position[0] + b.rect.width, b.rect.top_left.position[1]] },
            Vertex { position: [ b.rect.top_left.position[0] + b.rect.width, b.rect.top_left.position[1] + b.rect.height] },
            Vertex { position: [ b.rect.top_left.position[0], b.rect.top_left.position[1] + b.rect.height] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
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

        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();
    }

    /*fn is_hovering(
        &mut self,
        frame: &mut Frame,
        top_left: Vertex,
        width: f32,
        height: f32,
        mouse_pos: (f32, f32)
    ) {
        if mouse_pos.0 < top_left.position[0] + width && mouse_pos.0 > top_left.position[0]
            && mouse_pos.1 < top_left.position[1] + height && mouse_pos.1 > top_left.position[1] {
                let top_left = Vertex { position: [ top_left.position[0] - 2.0, top_left.position[1] - 2.0] };
                self.draw_box(frame, top_left, width + 4.0, height + 4.0);
            }
    }*/

    fn draw_check_box(
        &mut self,
        frame: &mut Frame,
        b: &CheckBox,
    ) {
        let width = 35.0;
        let height = 35.0;
        let uniforms = uniform! {
            screen_size: [self.window_size.0 as f32, self.window_size.1 as f32],
            color_input: [b.color.v[0], b.color.v[1], b.color.v[2], b.color.v[3]]
        };

        let shape = vec![
            Vertex { position: [ b.top_left.position[0], b.top_left.position[1] ] },
            Vertex { position: [ b.top_left.position[0] + width, b.top_left.position[1]] },
            Vertex { position: [ b.top_left.position[0] + width, b.top_left.position[1] + height] },
            Vertex { position: [ b.top_left.position[0], b.top_left.position[1] + height] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
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

        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();

        // if mouse is witin checkbox
        /*
        self.is_hovering(frame, top_left, width, height, mouse_pos);

        if self.is_clicked(mouse_pos, top_left, width, height, clicked) {
            *toggle = !*toggle;
        }
        if *toggle {
            self.draw_check(frame, top_left);
        }*/
    }

    fn draw_check(
        &mut self,
        frame: &mut Frame,
        top_left: Vertex,
    ) {
        let uniforms = uniform! {
            screen_size: [self.window_size.0 as f32, self.window_size.1 as f32]
        };

        let shape = vec![
            Vertex { position: [ top_left.position[0] + 2.0, top_left.position[1] + 20.0 ] },
            Vertex { position: [ top_left.position[0] + 15.0, top_left.position[1] + 30.0] },
            Vertex { position: [ top_left.position[0] + 33.0, top_left.position[1] + 5.0] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
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

        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &params
        ).unwrap();
    }
    fn is_dragging() {

    }
    fn calc_base(&mut self, game: &mut Game, width: f32, height: f32) -> Vertex {
        // if mouse_pos has changed
        if game.mouse_pos.0 != game.toggles.cached_mouse_pos.0 || game.mouse_pos.1 != game.toggles.cached_mouse_pos.1 {
            if self.is_clicked(game.mouse_pos, self.base, width, height, game.toggles.dragging) {
                self.base.position[0] += game.mouse_pos.0 - game.toggles.cached_mouse_pos.0;
                self.base.position[1] += game.mouse_pos.1 - game.toggles.cached_mouse_pos.1;
                game.toggles.cached_mouse_pos = game.mouse_pos;
            }
        }
        return self.base
    }

    pub fn render_menu(
        &mut self,
        game: &mut Game,
        frame: &mut Frame
    ) {
        /*const WIDTH: f32 = 600.0;
        const HEIGHT: f32 = 450.0;
        let base = self.calc_base(game, WIDTH, HEIGHT);
        // draw main box
        self.draw_filled_box(
            frame,
            base,
            WIDTH,
            HEIGHT,
            Vec4::new(0.2, 0.2, 0.2, 1.0)
        );

        // label for check_box
        self.draw_text(
            frame,
            Vertex { position: [base.position[0] + 12.0, base.position[1] + 22.0 ] },
            "Esp",
            30.0,
            Vec4::new(0.5, 1.0, 0.5, 1.0)
        );
        // draw esp checkbox
        self.draw_check_box(
            frame,
            Vertex { position: [ base.position[0] + 15.0, base.position[1] + 30.0 ] },
             &mut game.toggles.esp,
            game.toggles.clicked,
            game.mouse_pos,
        );*/
    }

}
