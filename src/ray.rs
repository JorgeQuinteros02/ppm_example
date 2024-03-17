use crate::vec3::Vec3;

#[derive(Debug, Default, Clone)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        return Ray{origin, direction}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t:f64) -> Vec3 {
        return self.origin + (self.direction * t)
    }
}