mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new("out.ppm");
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    writer
        .write(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .unwrap();

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = ray.color();

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            writer
                .write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
