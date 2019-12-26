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
        let t = self.hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5);

        if t > 0.0 {
            let normal = (self.point_at(t) - Vec3(0.0, 0.0, -1.0)).norm();
            Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5
        } else {
            let direction = self.direction.norm();
            let time = 0.5 * (direction.y() + 1.0);
            Vec3(1.0, 1.0, 1.0) * (1.0 - time) + Vec3(0.5, 0.7, 1.0) * time
        }
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> f32 {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = oc.dot(self.direction) * 2.0;
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }

    pub fn point_at(&self, time: f32) -> Vec3 {
        self.origin + self.direction * time
    }
}
