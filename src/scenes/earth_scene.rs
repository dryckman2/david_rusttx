use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::sphere::Sphere;
use crate::materials::lambertian::Lambertian;
use crate::materials::MatEnum;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use crate::textures::image_texture::ImageTexture;
use crate::textures::TexEnum;
use std::sync::Arc;

pub struct EarthScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl EarthScene {
    pub fn blank() -> EarthScene {
        EarthScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for EarthScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();

        let earth_texture = ImageTexture::from("earthmap.jpg");
        let earth_surface = Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
            TexEnum::ImageTexture(earth_texture),
        )));
        let globe = Arc::new(Sphere::from(
            Point3::from(0.0, 0.0, 0.0),
            2.0,
            earth_surface,
        ));
        world.add(globe);

        let aspect_ratio = 16.0 / 9.0;

        let vfov = 20.0;
        let lookfrom = Point3::from(0.0, 0.0, 12.0);
        let lookat = Point3::from(0.0, 0.0, 0.0);
        let vup = Vec3::from(0.0, 1.0, 0.0);

        let defocus_angle = 0.0;
        let background = Color::from(0.0, 0.0, 0.0);

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
