use super::{Material, HitRecord, utility::{rand, vec3, ray::Ray, color::Color}};


pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric { ir:index_of_refraction}
    }

    pub fn reflectance(&self, cosine:f64, ref_idx:f64) -> f64 {
        //Use Schlick's approximation for reflectance.
        let mut  r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0)*((1.0-cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation: &mut Color, scattered:&mut Ray) -> bool {
        *attenuation =  Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0 / self.ir} else {self.ir};

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = rec.normal.dot(-unit_direction).min(1.0);
        let sin_theta = ((1.0 - cos_theta*cos_theta) as f64).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rand::random_double() {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        
        *scattered = Ray::new_timed(rec.p, direction, r_in.time());
        true
    }
}