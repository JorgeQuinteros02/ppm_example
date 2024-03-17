mod vec3;
mod color;
mod ray;

use indicatif::ProgressBar;
use color::Color;
use color::write_color;
use vec3::Vec3;

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;

    //Calculate image height and ensure it is at least 1.
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let bar = ProgressBar::new(image_height as u64 - 1);

    // Viewport widths less than one are ok since they are real valued.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    println!("P3"); // The colors are in ASCII
    println!("{image_width} {image_height}"); // specifying number of columns and rows
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                (i as f64)/((image_width - 1) as f64),
                (j as f64)/((image_height - 1) as f64),
                0.0
            );

            write_color(pixel_color)
        }
        bar.inc(1);
    }
}
