use std::io::Write;
use std::ops::Deref;
use std::process::exit;
use std::rc::Rc;
use crate::hittables::hittable::{HitRecord, Hittable};
use crate::materials::isotropical::Isotropic;
use crate::materials::MatEnum;
use crate::materials::material::Material;
use crate::math_structures::aabb::Aabb;
use crate::math_structures::color::Color;
use crate::math_structures::interval;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
use crate::rtweekend::{INFINITY, random_double};
use crate::textures::TexEnum;
use crate::textures::texture::Texture;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: MatEnum,
}

impl ConstantMedium {
    pub fn from_texture(b: Box<dyn Hittable>, d: f64, a: TexEnum) -> ConstantMedium {
        let phase_function = MatEnum::Isotropic(Isotropic::from_texture(a));
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function,
        }
    }

    pub fn from_color(b: Box<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        let phase_function = MatEnum::Isotropic(Isotropic::from_color(c));
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set ENABLE_DEBUG true.

        const ENABLE_DEBUG: bool = true;
        let debugging: bool = ENABLE_DEBUG && random_double() < 0.00001;

        let mut rec1;
        match self.boundary.hit(r, &interval::UNIVERSE) {
            None => { return None; }
            Some(x) => { rec1 = x }
        }

        let mut rec2;
        match self.boundary.hit(r, &Interval::from(rec1.t + 0.001, INFINITY)) {
            None => { return None; }
            Some(x) => { rec2 = x; }
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(random_double());

        if hit_distance > distance_inside_boundary {
            return None;
        }


        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        let rec = HitRecord {
            p,
            normal: Vec3::from(1.0, 0.0, 0.0),  // arbitrary
            t,
            mat: self.phase_function.clone(),
            front_face: true, // also arbitrary
            u: 0.0,
            v: 0.0,
        };
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}