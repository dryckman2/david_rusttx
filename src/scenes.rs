use std::rc::Rc;
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

pub fn quads_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    // materials
    let left_red = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(1.0, 0.2, 0.2))));
    let back_green = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.2, 1., 0.2))));
    let right_blue = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.2, 0.2, 1.0))));
    let upper_orange = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(1.0, 0.5, 0.0))));
    let lower_teal = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.2, 0.8, 0.8))));

    // Quads
    world.add(Box::new(Quad::from(
        Point3::from(-3.0, -2.0, 5.0),
        Vec3::from(0.0, 0.0, -4.0),
        Vec3::from(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(-2.0, -2.0, 0.0),
        Vec3::from(4.0, 0.0, 0.0),
        Vec3::from(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(3.0, -2.0, 1.0),
        Vec3::from(0.0, 0.0, 4.0),
        Vec3::from(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(-2.0, 3.0, 1.0),
        Vec3::from(4.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Box::new(Quad::from(
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
    (cam, world)
}

pub fn two_perlin_spheres_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let pertext = NoiseTexture::new(4.0);
    let pertext_mat = Rc::new(MatEnum::Lambertian(Lambertian::from_texture(TexEnum::NoiseTexture(pertext))));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        pertext_mat.clone(),
    )));
    world.add(Box::new(Sphere::from(
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
    (cam, world)
}

pub fn random_spheres_scene() -> (Camera, HittableList) {
    //World
    let mut world = HittableList::blank();

    // Ground
    let checker =
        CheckerTexture::from_color(0.32, Color::from(0.2, 0.3, 0.1), Color::from(0.9, 0.9, 0.9));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(MatEnum::Lambertian(Lambertian::from_texture(TexEnum::CheckerTexture(checker)))),
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
                    let sphere_material = Rc::new(MatEnum::Lambertian(Lambertian::from_color(albedo)));
                    world.add(Box::new(Sphere::from_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_bounded(0.5, 1.0);
                    let fuzz = random_double_bounded(0.0, 0.5);
                    let sphere_material = Rc::new(MatEnum::Metal(Metal::from(albedo, fuzz)));
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Rc::new(MatEnum::Dielectric(Dielectric::from(1.5)));
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        (sphere_material),
                    )));
                }
            }
        }
    }

    // Big Balls
    let material1 = Rc::new(MatEnum::Dielectric(Dielectric::from(1.5)));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.4, 0.2, 0.1))));
    world.add(Box::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(MatEnum::Metal(Metal::from(Color::from(0.7, 0.6, 0.5), 0.0)));
    world.add(Box::new(Sphere::from(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let _ = BvhNode::from_list(&world);

    //Camera Variables
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
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
    (cam, world)
}

pub fn two_spheres_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let checker =
        CheckerTexture::from_color(0.8, Color::from(0.2, 0.3, 0.1), Color::from(0.9, 0.9, 0.9));
    let checker_rc = Rc::new(MatEnum::Lambertian(Lambertian::from_texture(TexEnum::CheckerTexture(checker))));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -10.0, 0.0),
        10.0,
        checker_rc.clone(),
    )));
    world.add(Box::new(Sphere::from(
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
    (cam, world)
}

pub fn earth_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let earth_texture = ImageTexture::from("earthmap.jpg");
    let earth_surface = Rc::new(MatEnum::Lambertian(Lambertian::from_texture(TexEnum::ImageTexture(earth_texture))));
    let globe = Box::new(Sphere::from(
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
    (cam, world)
}

pub fn simple_list_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let pertext = NoiseTexture::new(4.0);
    let pertext_mat = Rc::new(MatEnum::Lambertian(Lambertian::from_texture(TexEnum::NoiseTexture(pertext))));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        pertext_mat.clone(),
    )));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 2.0, 0.0),
        2.0,
        pertext_mat,
    )));

    let difflight = Rc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(Color::from(4.0, 4.0, 4.0))));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Box::new(Quad::from(
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

    (cam, world)
}

pub fn cornell_box_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let red = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.65, 0.05, 0.05))));
    let white = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.73, 0.73, 0.73))));
    let green = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.12, 0.45, 0.15))));
    let light = Rc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(Color::from(15.0, 15.0, 15.0))));

    world.add(Box::new(Quad::from(
        Point3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(343.0, 554.0, 332.0),
        Vec3::from(-130.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::from(
        Point3::from(555.0, 555.0, 555.0),
        Vec3::from(-555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::from(
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
    ) as Box<dyn Hittable>;
    box1 = Box::new(RotateY::from(box1, 15.0));
    box1 = Box::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2 = Quad::make_box(
        &Point3::from(0.0, 0.0, 0.0),
        &Point3::from(165.0, 165.0, 165.0),
        white,
    ) as Box<dyn Hittable>;
    box2 = Box::new(RotateY::from(box2, -18.0));
    box2 = Box::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(box2);

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

    (cam, world)
}

pub fn cornell_smoke_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let red = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.65, 0.05, 0.05))));
    let white = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.73, 0.73, 0.73))));
    let green = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.12, 0.45, 0.15))));
    let light = Rc::new(MatEnum::DiffuseLight(DiffuseLight::from_color(Color::from(7.0, 7.0, 7.0))));

    world.add(Box::new(Quad::from(
        Point3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(113.0, 554.0, 127.0),
        Vec3::from(330.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Box::new(Quad::from(
        Point3::from(0.0, 555.0, 0.0),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::from(
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::from(
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
    ) as Box<dyn Hittable>;
    box1 = Box::new(RotateY::from(box1, 15.0));
    box1 = Box::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(Box::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::from(0.0, 0.0, 0.0),
    )));

    let mut box2 = Quad::make_box(
        &Point3::from(0.0, 0.0, 0.0),
        &Point3::from(165.0, 165.0, 165.0),
        white.clone(),
    ) as Box<dyn Hittable>;
    box2 = Box::new(RotateY::from(box2, -18.0));
    box2 = Box::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(Box::new(ConstantMedium::from_color(
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

    (cam, world)
}


fn final_scene(image_width: i64, samples_per_pixel: i64, max_depth: i64) -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let boxes1;
    let ground = Rc::new(MatEnum::Lambertian(Lambertian::from_color(Color::from(0.48, 0.83, 0.53))));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_bounded(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Quad::make_box(&Point3::from(x0, y0, z0), &Point3::from(x1, y1, z1), ground.clone()));
        }
    }


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

    (cam, world)
}
//void final_scene(int image_width, int samples_per_pixel, int max_depth) {
//     hittable_list boxes1;
//     auto ground = make_shared<lambertian>(color(0.48, 0.83, 0.53));
//
//     int boxes_per_side = 20;
//     for (int i = 0; i < boxes_per_side; i++) {
//         for (int j = 0; j < boxes_per_side; j++) {
//             auto w = 100.0;
//             auto x0 = -1000.0 + i*w;
//             auto z0 = -1000.0 + j*w;
//             auto y0 = 0.0;
//             auto x1 = x0 + w;
//             auto y1 = random_double(1,101);
//             auto z1 = z0 + w;
//
//             boxes1.add(box(point3(x0,y0,z0), point3(x1,y1,z1), ground));
//         }
//     }
//
//     hittable_list world;
//
//     world.add(make_shared<bvh_node>(boxes1));
//
//     auto light = make_shared<diffuse_light>(color(7, 7, 7));
//     world.add(make_shared<quad>(point3(123,554,147), vec3(300,0,0), vec3(0,0,265), light));
//
//     auto center1 = point3(400, 400, 200);
//     auto center2 = center1 + vec3(30,0,0);
//     auto sphere_material = make_shared<lambertian>(color(0.7, 0.3, 0.1));
//     world.add(make_shared<sphere>(center1, center2, 50, sphere_material));
//
//     world.add(make_shared<sphere>(point3(260, 150, 45), 50, make_shared<dielectric>(1.5)));
//     world.add(make_shared<sphere>(
//         point3(0, 150, 145), 50, make_shared<metal>(color(0.8, 0.8, 0.9), 1.0)
//     ));
//
//     auto boundary = make_shared<sphere>(point3(360,150,145), 70, make_shared<dielectric>(1.5));
//     world.add(boundary);
//     world.add(make_shared<constant_medium>(boundary, 0.2, color(0.2, 0.4, 0.9)));
//     boundary = make_shared<sphere>(point3(0,0,0), 5000, make_shared<dielectric>(1.5));
//     world.add(make_shared<constant_medium>(boundary, .0001, color(1,1,1)));
//
//     auto emat = make_shared<lambertian>(make_shared<image_texture>("earthmap.jpg"));
//     world.add(make_shared<sphere>(point3(400,200,400), 100, emat));
//     auto pertext = make_shared<noise_texture>(0.1);
//     world.add(make_shared<sphere>(point3(220,280,300), 80, make_shared<lambertian>(pertext)));
//
//     hittable_list boxes2;
//     auto white = make_shared<lambertian>(color(.73, .73, .73));
//     int ns = 1000;
//     for (int j = 0; j < ns; j++) {
//         boxes2.add(make_shared<sphere>(point3::random(0,165), 10, white));
//     }
//
//     world.add(make_shared<translate>(
//         make_shared<rotate_y>(
//             make_shared<bvh_node>(boxes2), 15),
//             vec3(-100,270,395)
//         )
//     );
//
//
//     cam.render(world);
// }