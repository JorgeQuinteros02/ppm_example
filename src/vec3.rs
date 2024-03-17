use vector3d::Vector3d;
use std::ops;

pub type Vec3 = Vector3d<f64>;

pub trait Mul {
    fn mul(self, rhs: Self) -> Self;
}

impl<T:ops::Mul<Output = T  >> Mul for Vector3d<T> {
    fn mul(self, rhs: Self) -> Self {
        let result = Vector3d::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
        return result

    }
}
