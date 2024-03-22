use crate::utility::{rand, vec3::{self, Vec3}};
 



pub struct Perlin {
    ranvec: Box<[Vec3]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    const POINT_COUNT:usize = 256;

    pub fn new() -> Self {
        let mut ranvec = Box::from_iter([Vec3::default();Self::POINT_COUNT]);
        for i in 0..Self::POINT_COUNT {
            ranvec[i] = vec3::unit_vector(vec3::random_vector_range(-1.0, 1.0))
        }

        Perlin {
            ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p:Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = (p.x.floor()) as i32;
        let j = (p.y.floor()) as i32;
        let k = (p.z.floor()) as i32;

        let mut c  = [[[Vec3::default();2];2];2];
        for (di, vec2d) in c.clone().iter_mut().enumerate() {
            for (dj ,vec1d) in vec2d.iter_mut().enumerate() {
                for (dk, _) in vec1d.iter_mut().enumerate() {
                    c[di][dj][dk] = self.ranvec [
                        
                            self.perm_x[((i+di as i32)  & 255) as usize] ^
                            self.perm_y[((j+dj as i32)  & 255) as usize] ^
                            self.perm_z[((k+dk as i32)  & 255) as usize]
                        
                    ];
                }
            }
        }

        Perlin::perlin_interp(&c, u, v, w)
    }

    pub fn perlin_generate_perm() -> Box<[usize]> {
        let mut p = Box::from_iter(0..Self::POINT_COUNT);
        Perlin::permute(&mut p, Self::POINT_COUNT);
        p
    }

    fn permute(p: &mut Box<[usize]>, n:usize) {
        for i in (1..n).rev() {
            let target = rand::random_int_range(0, i as i32) as usize;
            p.swap(target, i);
        }
    }

    /* fn trilinear_interp(c:&[[[f64;2];2];2], u:f64, v:f64, w:f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    
                    accum +=
                        ((i as f64 *u) as f64*u + (1.0-i as f64)*(1.0-u as f64))*
                        ((j as f64 *v) as f64*v + (1.0-j as f64)*(1.0-v as f64))*
                        ((k as f64 *w) as f64*w + (1.0-k as f64)*(1.0-w as f64))* c[i][j][k];

                }
            }
        }
        accum
    } */

    fn perlin_interp(c:&[[[Vec3;2];2];2], u:f64, v:f64, w:f64) -> f64 {
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;
        
        for i in [0.0,1.0] {
            for j in [0.0,1.0] {
                for k in [0.0,1.0] {
                    let weight_v = Vec3::new(u-i, v-j, w-k);
                    accum +=
                        ((i*uu)  + (1.0-i)*(1.0-u ))*
                        ((j *vv) + (1.0-j)*(1.0-v ))*
                        ((k *ww) + (1.0-k)*(1.0-w ))* c[i as usize][j as usize][k as usize].dot(weight_v);

                }
            }
        }
        accum
    }

    pub fn turb(&self, p: Vec3, depth:i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight*self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Perlin::new()
    }
}