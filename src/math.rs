use std::ops::Sub;
use crate::game::Game;
use crate::offsets;

static M_PI: f32 = 3.14159265358979323846;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub v: [f32; 3],
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 {
    pub v: [f32; 2],
}

struct Matrix {
    pub v: [[f32; 4]; 4],
}

#[derive(Default)]
pub struct ViewAngles {
    pub pitch: f32,
    pub yaw: f32,
}

fn radians_to_degrees(angle: &f32) -> f32 {
    angle * (180.0 / M_PI)
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            v:
            [
                x,
                y
            ],
        }
    }
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            v:
            [
                x,
                y,
                z,
            ],
        }
    }
    pub fn calculate_angle(self, target: Vec3) -> ViewAngles {
        let mut angles = ViewAngles::default();

        let yaw_angle = (target.v[1] - self.v[1]).atan2(target.v[0] - self.v[0]);
        angles.yaw = radians_to_degrees(&yaw_angle);

        let distance = (target.v[2] - self.v[2]) / (self - target).calculate_length();
        let pitch_angle = -(distance.asin());
        angles.pitch = radians_to_degrees(&pitch_angle);

        angles
    }
    fn calculate_length(&self) -> f32 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]).sqrt()
    }
    pub fn wts(&self, game: &Game, screen_size: (u32, u32) ) -> Option<Vec2> {
        let view_matrix: Matrix = game.process.read(
            game.process.modules.get("client.dll")
                .expect("Failed to retrieve module base")
                .lpBaseOfDll as usize + offsets::VIEW_MATRIX
        ).expect("Failed to read matrix mem");

        let z = view_matrix.v[3][0] * self.v[0] + view_matrix.v[3][1] * self.v[1]
            + view_matrix.v[3][2] * self.v[2] + view_matrix.v[3][3];

        if z < 0.001 {
            return None
        }

        let mut out = Vec2::new(0.0, 0.0);
        out.v[0] = screen_size.0 as f32 * 0.5;
        out.v[1] = screen_size.1 as f32 * 0.5;

        out.v[0] *= 1.0 + (view_matrix.v[0][0] * self.v[0] + view_matrix.v[0][1] *
                            self.v[1] + view_matrix.v[0][2] * self.v[2] + view_matrix.v[0][3]) / z;
        out.v[1] *= 1.0 - (view_matrix.v[1][0] * self.v[0] + view_matrix.v[1][1] *
                           self.v[1] + view_matrix.v[1][2] * self.v[2] + view_matrix.v[1][3]) / z;

        Some(out)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}
