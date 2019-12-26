use crate::ray::{Hit, Hitable, Ray};
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction) * 2.0;
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                hit.t = temp;
                hit.point = ray.point_at(hit.t);
                hit.normal = (hit.point - self.center) / self.radius;
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
