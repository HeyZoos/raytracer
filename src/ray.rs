use crate::hitable_list::HitableList;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn color(&self, world: &HitableList) -> Vec3 {
        let mut hit = Hit::new();

        if world.hit(self, 0.0, std::f64::MAX, &mut hit) {
            Vec3(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            ) * 0.5
        } else {
            let unit_direction = self.direction.norm();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::one() + t * Vec3(0.5, 0.7, 1.0)
        }
    }

    pub fn point_at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }
}

pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            t: 0.0,
            point: Vec3::zero(),
            normal: Vec3::zero(),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut Hit) -> bool;
}
