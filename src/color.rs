use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color (pixel_color: Color) {
    println!("{} {} {}", 
             (pixel_color.x * 255.999) as i32,
             (pixel_color.y * 255.999) as i32,
             (pixel_color.z * 255.999) as i32
            )
}