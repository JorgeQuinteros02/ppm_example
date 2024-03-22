use std::ops::Add;

use crate::utility::{interval::Interval, vec3::Vec3, ray::Ray};

#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn from_points(a: Vec3, b: Vec3) -> Aabb {
        // Treat the two points a and b as extrema for the bonding box, so we don't require a
        // particular minimum/maximum coordinate order
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        Aabb{x,y,z}
    }

    pub fn from_boxes(box0:Aabb, box1:Aabb) -> Aabb {
        Aabb{
            x:Interval::from_intervals(box0.x, box1.x),
            y:Interval::from_intervals(box0.y, box1.y),
            z:Interval::from_intervals(box0.z, box1.z),
    
        }
        }

    pub fn axis(&self, n: usize) -> Interval {
        if n==1 {self.y}
        else if n==2 {self.z}
        else {self.x}
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool{
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;
            
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            }

            ray_t.shrink(t0, t1);

            if ray_t.max <= ray_t.min {return false;}
        }
        true
    }

    pub fn pad(&self) -> Aabb {
        // Return an Aabb that has no side narrower than some delta, padding if necessary.
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {self.x} else {self.x.expand(delta)};
        let new_y = if self.y.size() >= delta {self.y} else {self.y.expand(delta)};
        let new_z = if self.z.size() >= delta {self.z} else {self.z.expand(delta)};
        
        Aabb{
            x:new_x,
            y:new_y,
            z:new_z,
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb {
            x:self.x + rhs.x,
            y:self.y + rhs.y,
            z:self.z + rhs.z,
        }
    }
}