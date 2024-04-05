use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::sphere::Sphere;
use crate::materials::lambertian::Lambertian;
use crate::materials::MatEnum;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use crate::textures::checker_texture::CheckerTexture;
use crate::textures::TexEnum;
use std::sync::Arc;

pub struct TwoSpheresScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl TwoSpheresScene {
    pub fn blank() -> TwoSpheresScene {
        TwoSpheresScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for TwoSpheresScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();

        let checker =
            CheckerTexture::from_color(0.8, Color::from(0.2, 0.3, 0.1), Color::from(0.9, 0.9, 0.9));
        let checker_rc = Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
            TexEnum::CheckerTexture(checker),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, -10.0, 0.0),
            10.0,
            checker_rc.clone(),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, 10.0, 0.0),
            10.0,
            checker_rc,
        )));

        let aspect_ratio = 16.0 / 9.0;

        let vfov = 20.0;
        let lookfrom = Point3::from(13.0, 2.0, 3.0);
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
