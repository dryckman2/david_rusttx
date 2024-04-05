use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::quad::Quad;
use crate::materials::lambertian::Lambertian;
use crate::materials::MatEnum;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use std::sync::Arc;

pub struct QuadsScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl QuadsScene {
    pub fn blank() -> QuadsScene {
        QuadsScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for QuadsScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();
        let lights = HittableList::blank();

        // materials
        let left_red = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            1.0, 0.2, 0.2,
        ))));
        let back_green = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.2, 1., 0.2,
        ))));
        let right_blue = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.2, 0.2, 1.0,
        ))));
        let upper_orange = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            1.0, 0.5, 0.0,
        ))));
        let lower_teal = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.2, 0.8, 0.8,
        ))));

        // Quads
        world.add(Arc::new(Quad::from(
            Point3::from(-3.0, -2.0, 5.0),
            Vec3::from(0.0, 0.0, -4.0),
            Vec3::from(0.0, 4.0, 0.0),
            left_red,
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(-2.0, -2.0, 0.0),
            Vec3::from(4.0, 0.0, 0.0),
            Vec3::from(0.0, 4.0, 0.0),
            back_green,
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(3.0, -2.0, 1.0),
            Vec3::from(0.0, 0.0, 4.0),
            Vec3::from(0.0, 4.0, 0.0),
            right_blue,
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(-2.0, 3.0, 1.0),
            Vec3::from(4.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, 4.0),
            upper_orange,
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(-2.0, -3.0, 5.0),
            Vec3::from(4.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, -4.0),
            lower_teal,
        )));

        let aspect_ratio = 1.0;
        let vfov = 80.0;
        let lookfrom = Point3::from(0.0, 0.0, 9.0);
        let lookat = Point3::from(0.0, 0.0, 0.0);
        let vup = Vec3::from(0.0, 1.0, 0.0);
        let defocus_angle = 0.0;
        let background = Color::from(0.70, 0.80, 1.00);
        let cam = Camera::initialize(
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            10.0,
            background,
        );

        self.cam = Some(Arc::new(cam));
        self.world = Some(Arc::new(world));
        self.lights = Some(Arc::new(lights));
    }

    fn get_cam(&self) -> Arc<Camera> {
        <Option<Arc<Camera>> as Clone>::clone(&self.cam).unwrap()
    }

    fn get_world(&self) -> Arc<HittableList> {
        <Option<Arc<HittableList>> as Clone>::clone(&self.world).unwrap()
    }

    fn get_lights(&self) -> Arc<HittableList> {
        <Option<Arc<HittableList>> as Clone>::clone(&self.lights).unwrap()
    }
}
