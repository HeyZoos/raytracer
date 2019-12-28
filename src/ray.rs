use crate::hitable_list::HitableList;
use crate::material::Material;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn color(ray: &Ray, world: &HitableList, depth: i64) -> Vec3 {
        let mut hit = Hit::new();

        if world.hit(ray, 0.001, std::f64::MAX, &mut hit) {
            let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
            let mut attenuation = Vec3::zero();
            let mut clone = hit.clone();

            if let Some(material) = hit.material {
                if depth < 50 && material.scatter(ray, &mut clone, &mut attenuation, &mut scattered)
                {
                    attenuation * Ray::color(&scattered, world, depth + 1)
                } else {
                    Vec3::zero()
                }
            } else {
                Vec3::zero()
            }
        } else {
            let unit_direction = ray.direction.norm();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::one() + t * Vec3(0.5, 0.7, 1.0)
        }
    }

    pub fn point_at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }
}

#[derive(Clone)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            t: 0.0,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            material: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut Hit) -> bool;
}
