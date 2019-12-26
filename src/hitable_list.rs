use crate::ray::{Hit, Hitable, Ray};

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in self.iter() {
            if hitable.hit(ray, t_min, closest_so_far, hit) {
                hit_anything = true;
                closest_so_far = hit.t;
            }
        }

        hit_anything
    }
}

pub type HitableList = Vec<Box<dyn Hitable>>;
