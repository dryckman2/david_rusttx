use std::ops::Deref;
use crate::hittables::hittable::{HitRecord, Hittable};
use crate::materials::material::Material;
use crate::materials::MatEnum;
use crate::math_structures::aabb::Aabb;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Rc<MatEnum>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64, mat: Rc<MatEnum>) -> Sphere {
        let rvec = Vec3::from(radius, radius, radius);
        let bbox = Aabb::from_points(&(&center - &rvec), &(&center + &rvec));
        Sphere {
            center1: center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::blank(),
            bbox,
        }
    }

    pub fn from_moving(center1: Point3, center2: Point3, radius: f64, mat: Rc<MatEnum>) -> Sphere {
        let rvec = Vec3::from(radius, radius, radius);
        let box1 = Aabb::from_points(&(&center1 - &rvec), &(&center1 + &rvec));
        let box2 = Aabb::from_points(&(&center2 - &rvec), &(&center2 + &rvec));
        let bbox = Aabb::from_aabbs(&box1, &box2);
        let center_vec = &center2 - &center1;
        Sphere {
            center1,
            radius,
            mat,
            is_moving: true,
            center_vec,
            bbox,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        // Linearly interpolate from center1 to center2 according to time, where t=0 yields
        // center1, and t=1 yields center2.
        return &self.center1 + &(time * &self.center_vec);
    }

    pub fn get_sphere_uv(&self, p: &Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.center(r.time())
        } else {
            self.center1
        };
        let oc = r.origin() - &center;

        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = &(&p - &self.center1) / self.radius;
        let (u, v) = self.get_sphere_uv(&outward_normal);
        let mut rec = HitRecord {
            t,
            p,
            normal: Vec3::blank(),
            front_face: false,
            u,
            mat: self.mat.deref().clone(),
            v,
        };
        rec.set_face_normal(&r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::from((*self).clone())
    }
}
