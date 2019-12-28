use crate::ray::{Hit, Ray};
use crate::vec3::Vec3;

use rand::random;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let point = 2.0 * Vec3(random(), random(), random()) - Vec3::one();
        if point.sqlen() < 1.0 {
            return point;
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &mut Hit,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &mut Hit,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        *scattered = Ray::new(hit.point, target - hit.point);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &mut Hit,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray.direction.norm(), hit.normal);
        *scattered = Ray::new(hit.point, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(hit.normal) > 0.0
    }
}
