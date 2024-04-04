use crate::camera::Camera;
use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::objects::quad::Quad;
use crate::hittables::rotate_y::RotateY;
use crate::hittables::translate::Translate;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::{DefaultMat, MatEnum};
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::scenes::Scene;
use crate::volume::constant_medium::ConstantMedium;
use std::sync::Arc;

pub struct CornellSmokeScene {
    pub(crate) cam: Option<Arc<Camera>>,
    pub(crate) world: Option<Arc<HittableList>>,
    pub(crate) lights: Option<Arc<HittableList>>,
}

impl CornellSmokeScene {
    pub fn blank() -> CornellSmokeScene {
        CornellSmokeScene {
            cam: None,
            world: None,
            lights: None,
        }
    }
}

impl Scene for CornellSmokeScene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64) {
        let mut world = HittableList::blank();

        let red = Box::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.65, 0.05, 0.05,
        ))));
        let white = Box::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.73, 0.73, 0.73,
        ))));
        let green = Box::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(
            0.12, 0.45, 0.15,
        ))));
        let light = Box::new(MatEnum::DiffuseLight(DiffuseLight::from_color(
            Color::from(7.0, 7.0, 7.0),
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
            Point3::from(113.0, 554.0, 127.0),
            Vec3::from(330.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, 305.0),
            light,
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 555.0, 0.0),
            Vec3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            white.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 0.0, 0.0),
            Vec3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, 555.0),
            white.clone(),
        )));
        world.add(Arc::new(Quad::from(
            Point3::from(0.0, 0.0, 555.0),
            Vec3::from(555.0, 0.0, 0.0),
            Vec3::from(0.0, 555.0, 0.0),
            white.clone(),
        )));

        //Boxes
        let mut box1 = Quad::make_box(
            &Point3::from(0.0, 0.0, 0.0),
            &Point3::from(165.0, 330.0, 165.0),
            white.clone(),
        ) as Arc<dyn Hittable + Send + Sync>;
        box1 = Arc::new(RotateY::from(box1, 15.0));
        box1 = Arc::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
        world.add(Arc::new(ConstantMedium::from_color(
            box1,
            0.01,
            Color::from(0.0, 0.0, 0.0),
        )));

        let mut box2 = Quad::make_box(
            &Point3::from(0.0, 0.0, 0.0),
            &Point3::from(165.0, 165.0, 165.0),
            white.clone(),
        ) as Arc<dyn Hittable + Send + Sync>;
        box2 = Arc::new(RotateY::from(box2, -18.0));
        box2 = Arc::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
        world.add(Arc::new(ConstantMedium::from_color(
            box2,
            0.01,
            Color::from(1.0, 1.0, 1.0),
        )));

        // Light Sources
        let mut lights = HittableList::blank();
        let m = Box::new(MatEnum::Default(DefaultMat {}));
        lights.add(Arc::new(Quad::from(
            Point3::from(343.0, 554.0, 332.0),
            Vec3::from(-130.0, 0.0, 0.0),
            Vec3::from(0.0, 0.0, -105.0),
            m,
        )));

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
