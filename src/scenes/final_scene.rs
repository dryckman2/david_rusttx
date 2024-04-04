use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::quad::Quad;
use crate::hittables::objects::sphere::Sphere;
use crate::hittables::rotate_y::RotateY;
use crate::hittables::translate::Translate;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::MatEnum;
use crate::math_structures::bvh::BvhNode;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::random_double_bounded;
use crate::scenes::Scene;
use crate::textures::image_texture::ImageTexture;
use crate::textures::noise_texture::NoiseTexture;
use crate::textures::TexEnum;
use crate::volume::constant_medium::ConstantMedium;
use std::sync::Arc;

pub struct FinalScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl FinalScene {
    pub fn blank() -> FinalScene {
        FinalScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for FinalScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut boxes1 = HittableList::blank();
        let ground = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.48, 0.83, 0.53,
        ))));
        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_double_bounded(1.0, 101.0);
                let z1 = z0 + w;

                boxes1.add(Quad::make_box(
                    &Point3::from(x0, y0, z0),
                    &Point3::from(x1, y1, z1),
                    ground.clone(),
                ));
            }
        }

        let mut world = HittableList::blank();
        world.add(Arc::new(boxes1));

        let light = Arc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(
            Color::from(7.0, 7.0, 7.0),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(123.0, 554.0, 147.0),
            Point3::from(300.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, 265.0),
            light,
        )));

        let center1 = Point3::from(400.0, 400.0, 200.0);
        let center2 = &center1 + &Vec3::from(30.0, 0.0, 0.0);
        let sphere_material = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.7, 0.3, 0.1,
        ))));
        world.add(Arc::new(Sphere::from_moving(
            center1,
            center2,
            50.0,
            sphere_material,
        )));

        world.add(Arc::new(Sphere::from(
            Point3::from(260.0, 150.0, 45.0),
            50.0,
            Arc::new(MatEnum::Dielectric(Dielectric::from(1.5))),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, 150.0, 145.0),
            50.0,
            Arc::new(MatEnum::Metal(Metal::from(Color::from(0.8, 0.8, 0.9), 1.0))),
        )));

        let boundary = Arc::new(Sphere::from(
            Point3::from(360.0, 150.0, 145.0),
            70.0,
            Arc::new(MatEnum::Dielectric(Dielectric::from(1.5))),
        ));
        world.add(boundary.clone());
        world.add(Arc::new(ConstantMedium::from_color(
            boundary,
            0.2,
            Color::from(0.2, 0.4, 0.9),
        )));
        let boundary = Arc::new(Sphere::from(
            Point3::from(0.0, 0.0, 0.0),
            5000.0,
            Arc::new(MatEnum::Dielectric(Dielectric::from(1.5))),
        ));
        world.add(Arc::new(ConstantMedium::from_color(
            boundary,
            0.0001,
            Color::from(1.0, 1.0, 1.0),
        )));

        let emat = Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
            TexEnum::ImageTexture(ImageTexture::from("earthmap.jpg")),
        )));
        world.add(Arc::new(Sphere::from(
            Point3::from(400.0, 200.0, 400.0),
            100.0,
            emat,
        )));

        let pertext = TexEnum::NoiseTexture(NoiseTexture::new(0.1));
        world.add(Arc::new(Sphere::from(
            Point3::from(220.0, 280.0, 300.0),
            80.0,
            Arc::new(MatEnum::Lambertian(Lambertian::from_texture(pertext))),
        )));

        let mut boxes2 = HittableList::blank();
        let white = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.73, 0.73, 0.73,
        ))));
        let ns = 1000;
        for _ in 0..ns {
            let cir = Arc::new(Sphere::from(
                Point3::random_bounded(0.0, 165.0),
                10.0,
                white.clone(),
            ));
            boxes2.add(cir);
        }
        world.add(Arc::new(Translate::from(
            Arc::new(RotateY::from(Arc::new(BvhNode::from_list(&boxes2)), 15.0)),
            Vec3::from(-100.0, 270.0, 395.0),
        )));

        let aspect_ratio = 1.0;
        let background = Color::from(0.0, 0.0, 0.0);

        let vfov = 40.0;
        let lookfrom = Point3::from(478.0, 278.0, -600.0);
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
