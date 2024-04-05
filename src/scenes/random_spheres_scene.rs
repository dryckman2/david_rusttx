use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::sphere::Sphere;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::MatEnum;
use crate::math_structures::bvh::BvhNode;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::{random_double, random_double_bounded};
use crate::scenes::Scene;
use crate::textures::checker_texture::CheckerTexture;
use crate::textures::TexEnum;
use std::sync::Arc;

pub struct RandomSpheresScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl RandomSpheresScene {
    pub fn blank() -> RandomSpheresScene {
        RandomSpheresScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for RandomSpheresScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        //World
        let mut world = HittableList::blank();

        // Ground
        let checker = CheckerTexture::from_color(
            0.32,
            Color::from(0.2, 0.3, 0.1),
            Color::from(0.9, 0.9, 0.9),
        );
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(MatEnum::Lambertian(Lambertian::from_texture(
                TexEnum::CheckerTexture(checker),
            ))),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_double();
                let center = Point3::from(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );
                if (&center - &Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = &Color::random() * &Color::random();
                        let center2 =
                            &center + &Vec3::from(0.0, random_double_bounded(0.0, 0.5), 0.0);
                        let sphere_material =
                            Arc::new(MatEnum::Lambertian(Lambertian::from_color(albedo)));
                        world.add(Arc::new(Sphere::from_moving(
                            center,
                            center2,
                            0.2,
                            sphere_material,
                        )));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_bounded(0.5, 1.0);
                        let fuzz = random_double_bounded(0.0, 0.5);
                        let sphere_material = Arc::new(MatEnum::Metal(Metal::from(albedo, fuzz)));
                        world.add(Arc::new(Sphere::from(center, 0.2, sphere_material)));
                    } else {
                        // glass
                        let sphere_material = Arc::new(MatEnum::Dielectric(Dielectric::from(1.5)));
                        world.add(Arc::new(Sphere::from(center, 0.2, sphere_material)));
                    }
                }
            }
        }

        // Big Balls
        let material1 = Arc::new(MatEnum::Dielectric(Dielectric::from(1.5)));
        world.add(Arc::new(Sphere::from(
            Point3::from(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));

        let material2 = Arc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.4, 0.2, 0.1,
        ))));
        world.add(Arc::new(Sphere::from(
            Point3::from(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = Arc::new(MatEnum::Metal(Metal::from(Color::from(0.7, 0.6, 0.5), 0.0)));
        world.add(Arc::new(Sphere::from(
            Point3::from(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));

        let _ = BvhNode::from_list(&world);

        //Camera Variables
        let aspect_ratio = 16.0 / 9.0;
        let vfov = 20.0;
        let lookfrom = Point3::from(13.0, 2.0, 3.0);
        let lookat = Point3::from(0.0, 0.0, 0.0);
        let vup = Vec3::from(0.0, 1.0, 0.0);

        let defocus_angle = 0.6;
        let focus_dist = 10.0;
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
            focus_dist,
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
