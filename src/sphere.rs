use crate::material::Material;
use crate::ray::{Hit, Hitable, Ray};
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut Hit) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                hit.t = temp;
                hit.point = ray.point_at(hit.t);
                hit.normal = (hit.point - self.center) / self.radius;
                hit.material = Some(self.material.clone());
                return true;
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                hit.t = temp;
                hit.point = ray.point_at(hit.t);
                hit.normal = (hit.point - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}
