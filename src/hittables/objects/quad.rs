use crate::hittables::hittable::{HitRecord, Hittable};
use crate::hittables::hittable_list::HittableList;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::materials::MatEnum;
use crate::math_structures::aabb::Aabb;
use crate::math_structures::color::Color;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<MatEnum>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn from(q: Point3, u: Vec3, v: Vec3, mat: Arc<MatEnum>) -> Quad {
        let mut this = Quad {
            q,
            u,
            v,
            mat,
            bbox: Aabb::blank(),
            normal: Vec3::blank(),
            d: 0.0,
            w: Vec3::blank(),
        };
        let n = Vec3::cross(&u, &v);
        this.normal = Vec3::unit_vector(&n);
        this.d = Vec3::dot(&this.normal, &this.q);
        this.w = &n / Vec3::dot(&n, &n);

        this.set_bounding_box();
        this
    }

    pub fn make_box(a: &Point3, b: &Point3, mat: Arc<MatEnum>) -> Arc<HittableList> {
        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
        let mut sides = HittableList::blank();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::from(
            f64::min(a.x(), b.x()),
            f64::min(a.y(), b.y()),
            f64::min(a.z(), b.z()),
        );
        let max = Point3::from(
            f64::max(a.x(), b.x()),
            f64::max(a.y(), b.y()),
            f64::max(a.z(), b.z()),
        );

        let dx = Vec3::from(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::from(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::from(0.0, 0.0, max.z() - min.z());

        sides.add(Arc::new(Quad::from(
            Point3::from(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.clone(),
        ))); // front
        sides.add(Arc::new(Quad::from(
            Point3::from(max.x(), min.y(), max.z()),
            -&dz,
            dy,
            mat.clone(),
        ))); // right
        sides.add(Arc::new(Quad::from(
            Point3::from(max.x(), min.y(), min.z()),
            -&dx,
            dy,
            mat.clone(),
        ))); // back
        sides.add(Arc::new(Quad::from(
            Point3::from(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
        ))); // left
        sides.add(Arc::new(Quad::from(
            Point3::from(min.x(), max.y(), max.z()),
            dx,
            -&dz,
            mat.clone(),
        ))); // top
        sides.add(Arc::new(Quad::from(
            Point3::from(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat.clone(),
        ))); // bottom

        Arc::new(sides)
    }

    pub fn set_bounding_box(&mut self) {
        self.bbox = Aabb::from_points(&self.q, &(&(&self.q + &self.u) + &self.v)).pad();
    }

    pub fn is_interior(a: f64, b: f64) -> Option<HitRecord> {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
            return None;
        }
        let mut rec = HitRecord {
            p: Vec3::blank(),
            normal: Vec3::blank(),
            t: 0.0,
            mat: MatEnum::Lambertian(Lambertian::from_color(Color::blank())),
            front_face: false,
            u: a,
            v: b,
        };
        rec.u = a;
        rec.v = b;
        return Some(rec);
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = Vec3::dot(&self.normal, &r.direction());

        // No hit if the ray is parallel to the plane.
        if f64::abs(denom) < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - Vec3::dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = &intersection - &self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        let inter_rec;
        match { Quad::is_interior(alpha, beta) } {
            Some(x) => {
                inter_rec = x;
            }
            None => {
                return None;
            }
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        let mut rec = HitRecord {
            p: intersection,
            normal: Vec3::blank(),
            t,
            mat: self.mat.deref().clone(),
            front_face: false,
            u: inter_rec.u,
            v: inter_rec.v,
        };
        rec.set_face_normal(r, &self.normal);

        return Some(rec);
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}
