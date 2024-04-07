use crate::camera::Camera;
use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::quad::Quad;
use crate::hittables::objects::sphere::Sphere;
use crate::hittables::rotate_y::RotateY;
use crate::hittables::translate::Translate;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::{DefaultMat, MatEnum};
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use std::sync::Arc;

pub struct ChatScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl ChatScene {
    pub fn blank() -> ChatScene {
        ChatScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for ChatScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();

        // Materials
        let purple = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.65, 0.05, 0.65,
        ))));
        let white = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.73, 0.73, 0.73,
        ))));
        let yellow = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.45, 0.45, 0.15,
        ))));
        let light = Arc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(
            Color::from(15.0, 15.0, 15.0),
        )));
        let aluminum = Arc::new(MatEnum::Metal(Metal::from(
            Color::from(0.8, 0.85, 0.88),
            0.0,
        )));
        let glass = Arc::new(MatEnum::Dielectric(Dielectric::from(1.5)));
        let default_mat = Arc::new(MatEnum::Default(DefaultMat {}));

        // Add objects to the world
        world.add(Arc::new(Quad::from(
            Point3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 555.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            yellow.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 0.0, 0.0),
            Vec3::from(0.0, 555.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            purple.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(343.0, 554.0, 332.0),
            Vec3::from(-130.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, -105.0),
            light.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 0.0, 0.0),
            Vec3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            white.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(555.0, 555.0, 555.0),
            Vec3::from(-555.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, -555.0),
            white.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 0.0, 555.0),
            Vec3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 555.0, 0.0),
            white.clone(),
        )));

        let mut metal_ball = Arc::new(Sphere::from(
            Point3::from(0.0, 0.0, 0.0),
            150.0,
            aluminum.clone(),
        )) as Arc<dyn Hittable + Send + Sync>;
        metal_ball = Arc::new(RotateY::from(metal_ball, 15.0));
        metal_ball = Arc::new(Translate::from(metal_ball, Vec3::from(265.0, 0.0, 295.0)));
        world.add(metal_ball);

        world.add(Arc::new(Sphere::from(
            Point3::from(190.0, 90.0, 190.0),
            90.0,
            glass.clone(),
        )));

        // Light Sources
        let mut lights = HittableList::blank();
        lights.add(Arc::new(Quad::from(
            Point3::from(343.0, 554.0, 332.0),
            Vec3::from(-130.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, -105.0),
            default_mat.clone(),
        )));
        lights.add(Arc::new(Sphere::from(
            Point3::from(190.0, 90.0, 190.0),
            90.0,
            default_mat,
        )));

        let aspect_ratio = 1.0;
        let background = Color::from(0.0, 0.0, 0.0);

        let vfov = 40.0;
        let lookfrom = Point3::from(278.0, 278.0, -1000.0);
        let lookat = Point3::from(278.0, 278.0, 0.0);
        let vup = Vec3::from(0.0, 1.0, 0.0);

        let defocus_angle = 0.0;

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

        let lights = HittableList::blank();
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
