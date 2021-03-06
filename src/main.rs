mod camera;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::material::{Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use rand::random;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::rc::Rc;

fn main() {
    let path = Path::new("out.ppm");
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut world = HitableList::new();

    world.push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian {
            albedo: Vec3(0.8, 0.3, 0.3),
        }),
    )));

    world.push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian {
            albedo: Vec3(0.8, 0.8, 0.0),
        }),
    )));

    world.push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            albedo: Vec3(0.8, 0.6, 0.2),
            fuzz: 0.0,
        }),
    )));

    world.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            albedo: Vec3(0.8, 0.8, 0.8),
            fuzz: 1.0,
        }),
    )));

    let camera = Camera::new();

    writer
        .write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut color = Vec3::zero();

            for _ in 0..ns {
                let u = (x as f64 + random::<f64>()) / nx as f64;
                let v = (y as f64 + random::<f64>()) / ny as f64;
                let ray = camera.get_ray(u, v);
                let _point = ray.point_at(2.0);
                color += Ray::color(&ray, &world, 0);
            }

            color /= ns as f64;
            color = Vec3(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

            let ir = (255.99 * color.r()) as i64;
            let ig = (255.99 * color.g()) as i64;
            let ib = (255.99 * color.b()) as i64;

            writer
                .write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
