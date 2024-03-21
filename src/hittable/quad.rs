use std::rc::Rc;

use crate::{material::{Mat, Material}, utility::{interval::Interval, ray::Ray, vec3::{self, Vec3}}};

use super::{aabb::AABB, hittable_list::HittableList, HitRecord, Hittable};


pub struct Quad {
    q:Vec3,
    u:Vec3,
    v:Vec3,
    mat:Mat,
    bbox:AABB,
    normal:Vec3,
    d:f64,
    w:Vec3,
}

impl Quad {
    pub fn new(q:Vec3, u: Vec3, v:Vec3, mat:Mat) -> Quad {
        let n = u.cross(v);
        let normal = vec3::unit_vector(n);
        let d = normal.dot(q);
        let w = n / n.norm2();
        let mut q1 = Quad {q, u, v, mat, bbox:AABB::default(), normal, d, w};
        q1.set_bounding_box();
        q1
    }

    pub fn set_bounding_box(&mut self) {
        self.bbox = AABB::from_points(self.q, self.q + self.u + self.v).pad();
    }

    pub fn is_interior(a:f64, b:f64, rec:&mut HitRecord) -> bool {
        // Given the hit point in the plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
            return false
        }
        rec.u = a;
        rec.v = b;
        return true
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(r.direction());

        // No hit if the ray is paralllel to the plane.
        if denom.abs() < 1e-8 {return false}

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {return false}

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        rec.t = t;
        rec.p = intersection;
        rec.mat = Option::Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);

        true
    }


}

pub fn _box(a:Vec3, b:Vec3, mat:Rc<dyn Material>) -> Rc<HittableList> {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.

    let mut sides = HittableList::default();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Rc::new(Quad::new(Vec3::new(min.x, min.y, max.z),  dx,  dy, mat.clone())));
    sides.add(Rc::new(Quad::new(Vec3::new(max.x, min.y, max.z), -dz,  dy, mat.clone())));
    sides.add(Rc::new(Quad::new(Vec3::new(max.x, min.y, min.z), -dx,  dy, mat.clone())));
    sides.add(Rc::new(Quad::new(Vec3::new(min.x, min.y, min.z),  dz,  dy, mat.clone())));
    sides.add(Rc::new(Quad::new(Vec3::new(min.x, max.y, max.z),  dx, -dz, mat.clone())));
    sides.add(Rc::new(Quad::new(Vec3::new(min.x, min.y, min.z),  dx,  dz, mat)));

    Rc::new(sides)
}