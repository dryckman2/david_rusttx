use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::Arc;

use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::materials::material::Material;
use crate::math_structures::color::foo::fmt_to_file;
use crate::math_structures::color::{write_color, Color};
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{random_in_unit_disk, Point3, Vec3};
use crate::multithreading::render_to_memory;
use crate::rtweekend::{degrees_to_radians, random_double, INFINITY};

#[derive(Clone)]
pub struct Camera {
    pub image_width: i64,
    pub image_height: i64,

    pub center: Point3,

    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,

    pub pixel00_loc: Vec3,

    pub samples_per_pixel: i64,
    pub max_depth: i64,

    // Variation angle of rays through each pixel
    pub defocus_angle: f64,

    // Defocus disk horizontal radius
    pub defocus_disk_u: Vec3,
    // Defocus disk vertical radius
    pub defocus_disk_v: Vec3,

    // Scene background color
    background: Color,
    pub(crate) sqrt_spp: f64,
    recip_sqrt_spp: f64,
}

impl Camera {
    #[allow(dead_code)] //Using in single thread render
    pub fn render(self, mut out_file: &mut File, world: &HittableList) {
        // Render
        fmt_to_file!(
            &mut out_file,
            "P3\n{} {}\n255\n",
            self.image_width,
            self.image_height
        );
        for j in 0..self.image_height {
            print!(
                "\rScan lines remaining: {}          ",
                (self.image_height - j) as f64
            );
            io::stdout().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = Color::blank();
                for _ in 0..self.samples_per_pixel {
                    for s_j in 0..self.sqrt_spp as i64 {
                        for s_i in 0..self.sqrt_spp as i64 {
                            let r = self.get_ray(i, j, s_i, s_j);
                            pixel_color += &self.ray_color(&r, self.max_depth, world);
                        }
                    }
                }
                write_color(&mut out_file, &pixel_color, self.samples_per_pixel);
            }
        }
        println!("\rDone.                        \n");
    }

    pub fn multi_threaded_render(&self, out_file: &mut File, world: &HittableList) {
        let c = Arc::new((*self).clone());
        let w = Arc::from((*world).clone());
        let x = render_to_memory(c, w);
        for y in x {
            out_file.write(y.as_bytes()).expect("TODO: panic message");
        }
    }

    pub fn initialize(
        aspect_ratio: f64,
        image_width: i64,
        samples_per_pixel: i64,
        max_depth: i64,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        background: Color,
    ) -> Camera {
        //Image Values
        let image_height = std::cmp::max(1, (image_width as f64 / aspect_ratio) as i64);

        let center = lookfrom.clone();

        // Camera
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = Vec3::unit_vector(&(&lookfrom - &lookat));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * &u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * &-&v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = &viewport_u / (image_width as f64);
        let pixel_delta_v = &viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            &(&(&center - &(focus_dist * &w)) - &(&viewport_u / 2.0)) - &(&viewport_v / 2.0);
        let pixel00_loc = &viewport_upper_left + &(0.5 * &(&pixel_delta_u + &pixel_delta_v));

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = &u * defocus_radius;
        let defocus_disk_v = &v * defocus_radius;

        let sqrt_spp = (samples_per_pixel as f64).sqrt();
        let recip_sqrt_spp = 1.0 / sqrt_spp;

        Camera {
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background,
            sqrt_spp,
            recip_sqrt_spp,
        }
    }
    //    ray get_ray(int i, int j, int s_i, int s_j) const {
    pub(crate) fn get_ray(&self, i: i64, j: i64, s_i: i64, s_j: i64) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        let pixel_center = &self.pixel00_loc
            + &(&(&self.pixel_delta_u * i as f64) + &(&self.pixel_delta_v * j as f64));
        let pixel_sample = &pixel_center + &self.pixel_sample_square(s_i, s_j);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = &pixel_sample - &ray_origin;
        let ray_time = random_double();

        return Ray::from_set_time(ray_origin, ray_direction, ray_time);
    }

    pub fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        &(&self.center + &(p[0] * &self.defocus_disk_u)) + &(p[1] * &self.defocus_disk_v)
    }
    fn pixel_sample_square(&self, s_i: i64, s_j: i64) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin, given
        // the two subpixel indices.
        let px = -0.5 + self.recip_sqrt_spp * (s_i as f64 + random_double());
        let py = -0.5 + self.recip_sqrt_spp * (s_j as f64 + random_double());
        &(px * &self.pixel_delta_u) + &(py * &self.pixel_delta_v)
    }

    pub fn ray_color(&self, r: &Ray, depth: i64, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::from(0.0, 0.0, 0.0);
        }

        let rec;
        // If the ray hits nothing, return the background color.
        match world.hit(r, &Interval::from(0.001, INFINITY)) {
            None => {
                return self.background;
            }
            Some(x) => {
                rec = x;
            }
        }

        let scattered;
        let attenuation;
        let mut pdf = 0.0;
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        match rec.mat.scatter(r, &rec,pdf) {
            None => {
                return color_from_emission;
            }
            Some(x) => {
                attenuation = x.0;
                scattered = x.1;
                pdf = x.2;
            }
        }
        //scatter(&self, r_in: &Ray, rec: &HitRecord,pdf:f64)\
        let scattering_pdf = rec.mat.scattering(r, &rec, scattered,pdf);
        let pdf = scattering_pdf;

        let color_from_scatter =
            (&(&attenuation * &scattering_pdf) * &self.ray_color(&scattered, depth - 1, world)) / pdf;

        &color_from_emission + &color_from_scatter
    }
}
