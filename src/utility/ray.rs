use super::vec3::Vec3;
#[derive(Debug, Default, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new_timed(origin: Vec3, direction: Vec3, tm: f64) -> Self {
        return Ray{origin, direction, tm}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t:f64) -> Vec3 {
        return self.origin + (self.direction * t)
    }
}