use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::quad::Quad;
use crate::hittables::objects::sphere::Sphere;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::MatEnum;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use crate::textures::noise_texture::NoiseTexture;
use crate::textures::TexEnum;
use std::sync::Arc;

pub struct SimpleListScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl SimpleListScene {
    pub fn blank() -> SimpleListScene {
        SimpleListScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for SimpleListScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();

        let pertext = NoiseTexture::new(4.0);
        let pertext_mat = Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
            TexEnum::NoiseTexture(pertext),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, -1000.0, 0.0),
            1000.0,
            pertext_mat.clone(),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, 2.0, 0.0),
            2.0,
            pertext_mat,
        )));

        let difflight = Arc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(
            Color::from(4.0, 4.0, 4.0),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, 7.0, 0.0),
            2.0,
            difflight.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(3.0, 1.0, -2.0),
            Vec3::from(2.0, 0.0, 0.0),
            Vec3::from(0.0, 2.0, 0.0),
            difflight,
        )));

        let aspect_ratio = 16.0 / 9.0;
        let background = Color::from(0.0, 0.0, 0.0);

        let vfov = 20.0;
        let lookfrom = Point3::from(26.0, 3.0, 6.0);
        let lookat = Point3::from(0.0, 2.0, 0.0);
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
