use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new("out.ppm");
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    let width = 200;
    let height = 100;

    writer
        .write(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .unwrap();

    for y in (0..height - 1).rev() {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.2f32;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            writer
                .write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
