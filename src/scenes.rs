use std::sync::Arc;

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
use crate::materials::MatEnum;
use crate::math_structures::bvh::BvhNode;
use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::{random_double, random_double_bounded};
use crate::textures::checker_texture::CheckerTexture;
use crate::textures::image_texture::ImageTexture;
use crate::textures::noise_texture::NoiseTexture;
use crate::textures::TexEnum;
use crate::volume::constant_medium::ConstantMedium;

pub fn quads_scene() -> (Arc<Camera>, Arc<HittableList>) {
    let mut world = HittableList::blank();

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
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
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
    (Arc::new(cam), Arc::new(world))
}

pub fn two_perlin_spheres_scene() -> (Arc<Camera>, Arc<HittableList>) {
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
        pertext_mat.clone(),
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
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
    (Arc::new(cam), Arc::new(world))
}

pub fn random_spheres_scene() -> (Arc<Camera>, Arc<HittableList>) {
    //World
    let mut world = HittableList::blank();

    // Ground
    let checker =
        CheckerTexture::from_color(0.32, Color::from(0.2, 0.3, 0.1), Color::from(0.9, 0.9, 0.9));
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
                    let center2 = &center + &Vec3::from(0.0, random_double_bounded(0.0, 0.5), 0.0);
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
    let image_width = 1920;
    let samples_per_pixel = 100;
    let max_depth = 50;
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
    (Arc::new(cam), Arc::new(world))
}

pub fn two_spheres_scene() -> (Arc<Camera>, Arc<HittableList>) {
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
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

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
    (Arc::new(cam), Arc::new(world))
}

pub fn earth_scene() -> (Arc<Camera>, Arc<HittableList>) {
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
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::from(0.0, 0.0, 12.0);
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
    (Arc::new(cam), Arc::new(world))
}

pub fn simple_list_scene() -> (Arc<Camera>, Arc<HittableList>) {
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
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
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

    (Arc::new(cam), Arc::new(world))
}

pub fn cornell_box_scene() -> (Arc<Camera>, Arc<HittableList>) {
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
        Color::from(15.0, 15.0, 15.0),
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
        Point3::from(343.0, 554.0, 332.0),
        Vec3::from(-130.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -105.0),
        light,
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

    //Boxes
    let mut box1 = Quad::make_box(
        &Point3::from(0.0, 0.0, 0.0),
        &Point3::from(165.0, 330.0, 165.0),
        white.clone(),
    ) as Arc<dyn Hittable + Send + Sync>;
    box1 = Arc::new(RotateY::from(box1, 15.0));
    box1 = Arc::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2 = Quad::make_box(
        &Point3::from(0.0, 0.0, 0.0),
        &Point3::from(165.0, 165.0, 165.0),
        white,
    ) as Arc<dyn Hittable + Send + Sync>;
    box2 = Arc::new(RotateY::from(box2, -18.0));
    box2 = Arc::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(box2);

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 100;
    let max_depth = 50;
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

    (Arc::new(cam), Arc::new(world))
}

pub fn cornell_smoke_scene() -> (Arc<Camera>, Arc<HittableList>) {
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

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;
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

    (Arc::new(cam), Arc::new(world))
}

pub fn final_scene(
    image_width: i64,
    samples_per_pixel: i64,
    max_depth: i64,
) -> (Arc<Camera>, Arc<HittableList>) {
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
        boxes2.add(Arc::new(Sphere::from(
            Point3::random_bounded(0.0, 165.0),
            10.0,
            white.clone(),
        )));
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

    (Arc::new(cam), Arc::new(world))
}
