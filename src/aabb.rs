use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv = 1.0 / ray.direction()[a];
            let mut t0 = (self.minimum[a] - ray.origin()[a]) * inv;
            let mut t1 = (self.maximum[a] - ray.origin()[a]) * inv;
            if inv < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let small = Point3::new(
            f64::min(self.minimum.x(), other.minimum.x()),
            f64::min(self.minimum.y(), other.minimum.y()),
            f64::min(self.minimum.z(), other.minimum.z()),
        );
        let big = Point3::new(
            f64::max(self.maximum.x(), other.maximum.x()),
            f64::max(self.maximum.y(), other.maximum.y()),
            f64::max(self.maximum.z(), other.maximum.z()),
        );
        Self::new(small, big)
    }

    pub fn minimum(&self) -> Point3 {
        self.minimum
    }

    pub fn maximum(&self) -> Point3 {
        self.maximum
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            minimum: Point3::default(),
            maximum: Point3::default(),
        }
    }
}
