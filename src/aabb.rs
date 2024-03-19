use crate::{hittable::Hittable, rtweekend::*};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(ix: Interval, iy: Interval, iz:Interval) -> AABB {
        AABB{x:ix, y:iy, z:iz}
    }

    pub fn from_points(a: Vec3, b: Vec3) -> AABB {
        // Treat the two points a and b as extrema for the bonding box, so we don't require a
        // particular minimum/maximum coordinate order
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        AABB{x,y,z}
    }

    pub fn from_boxes(box0:AABB, box1:AABB) -> AABB {
        AABB{
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
            let invD = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * invD;
            let mut t1 = (self.axis(a).max - orig) * invD;
            
            if invD < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            ray_t.shrink(t0, t1);

            if ray_t.max <= ray_t.min {return false;}
        }
        true
    }
}