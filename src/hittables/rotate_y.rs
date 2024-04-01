use crate::hittables::hittable::{HitRecord, Hittable};
use crate::math_structures::aabb::Aabb;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::{degrees_to_radians, INFINITY};

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn from(p: Box<dyn Hittable>, angle: f64) -> RotateY {
        let mut this = RotateY {
            object: p,
            sin_theta: 0.0,
            cos_theta: 0.0,
            bbox: Aabb::blank(),
        };

        let radians = degrees_to_radians(angle);
        this.sin_theta = f64::sin(radians);
        this.cos_theta = f64::cos(radians);
        this.bbox = this.object.bounding_box();
        let mut min = Point3::from(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::from(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * this.bbox.x.max + (1 - i) as f64 * this.bbox.x.min;
                    let y = j as f64 * this.bbox.y.max + (1 - j) as f64 * this.bbox.y.min;
                    let z = k as f64 * this.bbox.z.max + (1 - k) as f64 * this.bbox.z.min;

                    let newx = this.cos_theta * x + this.sin_theta * z;
                    let newz = -this.sin_theta * x + this.cos_theta * z;

                    let tester = Vec3::from(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        this.bbox = Aabb::from_points(&min, &max);

        this
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Change the ray from world space to object space
        let mut origin = r.origin().clone();
        let mut direction = r.direction().clone();
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::from_set_time(origin.clone(), direction.clone(), r.time());

        // Determine where (if any) an intersection occurs in object space
        let mut rec;
        match { self.object.hit(&rotated_r, ray_t) } {
            None => { return None; }
            Some(x) => { rec = x }
        };


        // Change the intersection point from object space to world space
        let mut p = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        // Change the normal from object space to world space
        let mut normal = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        todo!()
    }
}
