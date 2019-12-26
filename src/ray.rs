use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn color(&self) -> Vec3 {
        let direction = self.direction.norm();
        let time = 0.5 * (direction.y() + 1.0);
        Vec3(1.0, 1.0, 1.0) * (1.0 - time) + Vec3(0.5, 0.7, 1.0) * time
    }

    pub fn point_at(&self, time: f32) -> Vec3 {
        self.origin + self.direction * time
    }
}
