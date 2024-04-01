use crate::hittables::hittable::HitRecord;
use crate::materials::material::Material;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{reflect, Point3, Vec3};
use crate::rtweekend::random_double;

#[derive(Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn from(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());
        let cos_theta = f64::min(Vec3::dot(&-&unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        Some((
            Color::from(1.0, 1.0, 1.0),
            Ray::from_set_time(rec.p, direction, r_in.time()),
        ))
    }
}
