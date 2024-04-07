use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::quad::Quad;
use crate::hittables::objects::sphere::Sphere;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::{DefaultMat, MatEnum};
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use crate::textures::image_texture::ImageTexture;
use crate::textures::TexEnum;
use std::sync::Arc;

pub struct EarthInABallScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl EarthInABallScene {
    pub fn blank() -> EarthInABallScene {
        EarthInABallScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for EarthInABallScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();

        let red = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.65, 0.05, 0.05,
        ))));
        let white = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.73, 0.73, 0.73,
        ))));
        let green = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.12, 0.45, 0.15,
        ))));
        let light = Arc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(
            Color::from(15.0, 10.0, 1.0),
        )));

        world.add(Arc::new(Quad::from(
            Point3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 555.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            green,
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 0.0, 0.0),
            Vec3::from(0.0, 555.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            red,
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

        // Glass Sphere
        let glass = Arc::new(MatEnum::Dielectric(Dielectric::from(1.5)));
        world.add(Arc::new(Sphere::from(
            Point3::from(250.0, 150.0, 190.0),
            200.0,
            glass,
        )));

        //Earth Sphere
        let earth_texture = Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
            TexEnum::ImageTexture(ImageTexture::from("earthmap.jpg")),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(250.0, 150.0, 190.0),
            60.0,
            earth_texture,
        )));

        let moon_texture = Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
            TexEnum::ImageTexture(ImageTexture::from("moonmap.jpeg")),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(340.0, 200.0, 190.0),
            15.0,
            moon_texture,
        )));

        // Light Sources
        let sun = Arc::new(Sphere::from(
            Point3::from(343.0, 554.0, 332.0),
            100.0,
            light,
        ));
        world.add(sun.clone());
        let mut lights = HittableList::blank();
        let m = Arc::new(MatEnum::Default(DefaultMat {}));
        lights.add(Arc::new(Sphere::from(
            Point3::from(343.0, 554.0, 332.0),
            100.0,
            m,
        )));

        // lights.add(Arc::new(Sphere::from(
        //     Point3::from(190.0, 90.0, 190.0),
        //     90.0,
        //     m,
        // )));

        let aspect_ratio = 1.0;
        let background = Color::from(0.0, 0.0, 0.0);

        let vfov = 40.0;
        let lookfrom = Point3::from(278.0, 278.0, -800.0);
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
