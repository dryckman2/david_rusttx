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
use crate::materials::MatEnum;
use crate::materials::metal::Metal;
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
    let left_red = Lambertian::from_color(Color::from(1.0, 0.2, 0.2));
    let back_green = Lambertian::from_color(Color::from(0.2, 1., 0.2));
    let right_blue = Lambertian::from_color(Color::from(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::from_color(Color::from(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::from_color(Color::from(0.2, 0.8, 0.8));

    // Quads
    world.add(Box::new(Quad::from(Point3::from(-3.0, -2.0, 5.0), Vec3::from(0.0, 0.0, -4.0), Vec3::from(0.0, 4.0, 0.0), MatEnum::Lambertian(left_red))));
    world.add(Box::new(Quad::from(Point3::from(-2.0, -2.0, 0.0), Vec3::from(4.0, 0.0, 0.0), Vec3::from(0.0, 4.0, 0.0), MatEnum::Lambertian(back_green))));
    world.add(Box::new(Quad::from(Point3::from(3.0, -2.0, 1.0), Vec3::from(0.0, 0.0, 4.0), Vec3::from(0.0, 4.0, 0.0), MatEnum::Lambertian(right_blue))));
    world.add(Box::new(Quad::from(Point3::from(-2.0, 3.0, 1.0), Vec3::from(4.0, 0.0, 0.0), Vec3::from(0.0, 0.0, 4.0), MatEnum::Lambertian(upper_orange))));
    world.add(Box::new(Quad::from(Point3::from(-2.0, -3.0, 5.0), Vec3::from(4.0, 0.0, 0.0), Vec3::from(0.0, 0.0, -4.0), MatEnum::Lambertian(lower_teal))));

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
        10.0, background,
    );
    (cam, world)
}

pub fn two_perlin_spheres_scene() -> (Camera, HittableList) {
    let mut world = HittableList::blank();

    let pertext = NoiseTexture::new(4.0);
    let pertext_mat = Lambertian::from_texture(TexEnum::NoiseTexture(pertext));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        MatEnum::Lambertian(pertext_mat.clone()),
    )));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 2.0, 0.0),
        2.0,
        MatEnum::Lambertian(pertext_mat.clone()),
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
    let checker = CheckerTexture::from_color(
        0.32,
        Color::from(0.2, 0.3, 0.1),
        Color::from(0.9, 0.9, 0.9),
    );
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        MatEnum::Lambertian(Lambertian::from_texture(TexEnum::CheckerTexture(checker))),
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
                    let sphere_material = Lambertian::from_color(albedo);
                    world.add(Box::new(Sphere::from_moving(
                        center,
                        center2,
                        0.2,
                        MatEnum::Lambertian(sphere_material),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_bounded(0.5, 1.0);
                    let fuzz = random_double_bounded(0.0, 0.5);
                    let sphere_material = Metal::from(albedo, fuzz);
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        MatEnum::Metal(sphere_material),
                    )));
                } else {
                    // glass
                    let sphere_material = Dielectric::from(1.5);
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        MatEnum::Dielectric(sphere_material),
                    )));
                }
            }
        }
    }

    // Big Balls
    let material1 = Dielectric::from(1.5);
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        MatEnum::Dielectric(material1),
    )));

    let material2 = Lambertian::from_color(Color::from(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        MatEnum::Lambertian(material2),
    )));

    let material3 = Metal::from(Color::from(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::from(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        MatEnum::Metal(material3),
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
    let checker_rc = Lambertian::from_texture(TexEnum::CheckerTexture(checker));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -10.0, 0.0),
        10.0,
        MatEnum::Lambertian(checker_rc.clone()),
    )));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 10.0, 0.0),
        10.0,
        MatEnum::Lambertian(checker_rc),
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
    let earth_surface = Lambertian::from_texture(TexEnum::ImageTexture(earth_texture));
    let globe = Box::new(Sphere::from(
        Point3::from(0.0, 0.0, 0.0),
        2.0,
        MatEnum::Lambertian(earth_surface),
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
    let pertext_mat = Lambertian::from_texture(TexEnum::NoiseTexture(pertext));
    world.add(Box::new(Sphere::from(Point3::from(0.0, -1000.0, 0.0), 1000.0, MatEnum::Lambertian(pertext_mat.clone()))));
    world.add(Box::new(Sphere::from(Point3::from(0.0, 2.0, 0.0), 2.0, MatEnum::Lambertian(pertext_mat))));

    let difflight = DiffuseLight::from_color(Color::from(4.0, 4.0, 4.0));
    world.add(Box::new(Sphere::from(Point3::from(0.0, 7.0, 0.0), 2.0, MatEnum::DiffuseLight(difflight.clone()))));
    world.add(Box::new(Quad::from(Point3::from(3.0, 1.0, -2.0), Vec3::from(2.0, 0.0, 0.0), Vec3::from(0.0, 2.0, 0.0), MatEnum::DiffuseLight(difflight))));

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

    let red = Lambertian::from_color(Color::from(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Color::from(0.73, 0.73, 0.73));
    let green = Lambertian::from_color(Color::from(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::from(15.0, 15.0, 15.0));

    world.add(Box::new(Quad::from(Point3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 555.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(green))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 0.0, 0.0), Vec3::from(0.0, 555.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(red))));
    world.add(Box::new(Quad::from(Point3::from(343.0, 554.0, 332.0), Vec3::from(-130.0, 0.0, 0.0), Vec3::from(0.0, 0.0, -105.0), MatEnum::DiffuseLight(light))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 0.0, 0.0), Vec3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(white.clone()))));
    world.add(Box::new(Quad::from(Point3::from(555.0, 555.0, 555.0), Vec3::from(-555.0, 0.0, 0.0), Vec3::from(0.0, 0.0, -555.0), MatEnum::Lambertian(white.clone()))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 0.0, 555.0), Vec3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 555.0, 0.0), MatEnum::Lambertian(white.clone()))));

    //Boxes
    let mut box1 = Quad::make_box(&Point3::from(0.0, 0.0, 0.0), &Point3::from(165.0, 330.0, 165.0), MatEnum::Lambertian(white.clone())) as Box<dyn Hittable>;
    box1 = Box::new(RotateY::from(box1, 15.0));
    box1 = Box::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2 = Quad::make_box(&Point3::from(0.0, 0.0, 0.0), &Point3::from(165.0, 165.0, 165.0), MatEnum::Lambertian(white)) as Box<dyn Hittable>;
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

    let red = Lambertian::from_color(Color::from(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Color::from(0.73, 0.73, 0.73));
    let green = Lambertian::from_color(Color::from(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::from(7.0, 7.0, 7.0));

    world.add(Box::new(Quad::from(Point3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 555.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(green))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 0.0, 0.0), Vec3::from(0.0, 555.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(red))));
    world.add(Box::new(Quad::from(Point3::from(113.0, 554.0, 127.0), Vec3::from(330.0, 0.0, 0.0), Vec3::from(0.0, 0.0, 305.0), MatEnum::DiffuseLight(light))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 555.0, 0.0), Vec3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(white.clone()))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 0.0, 0.0), Vec3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 0.0, 555.0), MatEnum::Lambertian(white.clone()))));
    world.add(Box::new(Quad::from(Point3::from(0.0, 0.0, 555.0), Vec3::from(555.0, 0.0, 0.0), Vec3::from(0.0, 555.0, 0.0), MatEnum::Lambertian(white.clone()))));

    //Boxes
    let mut box1 = Quad::make_box(&Point3::from(0.0, 0.0, 0.0), &Point3::from(165.0, 330.0, 165.0), MatEnum::Lambertian(white.clone())) as Box<dyn Hittable>;
    box1 = Box::new(RotateY::from(box1, 15.0));
    box1 = Box::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(Box::new(ConstantMedium::from_color(box1, 0.01, Color::from(0.0, 0.0, 0.0))));

    let mut box2 = Quad::make_box(&Point3::from(0.0, 0.0, 0.0), &Point3::from(165.0, 165.0, 165.0), MatEnum::Lambertian(white.clone())) as Box<dyn Hittable>;
    box2 = Box::new(RotateY::from(box2, -18.0));
    box2 = Box::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(Box::new(ConstantMedium::from_color(box2, 0.01, Color::from(1.0, 1.0, 1.0))));

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