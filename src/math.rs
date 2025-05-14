use std::ops::Sub;

static M_PI: f32 = 3.14159265358979323846;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    v: [f32; 3],
}

#[derive(Default)]
pub struct ViewAngles {
    pub pitch: f32,
    pub yaw: f32,
}

fn radians_to_degrees(angle: &f32) -> f32 {
    angle * (180.0 / M_PI)
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            v: [
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
