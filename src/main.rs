mod camera;
mod hittables;
mod materials;
mod math_structures;
mod rtw_image;
mod rtweekend;
mod scenes;
mod textures;
mod volume;

use std::thread::Builder;

use crate::scenes::{
    cornell_box_scene, cornell_smoke_scene, earth_scene, quads_scene, random_spheres_scene,
    simple_list_scene, two_perlin_spheres_scene, two_spheres_scene,
};
use std::fs::File;

fn uncapped_main() {
    let (cam, world);

    match 8 {
        1 => {
            (cam, world) = random_spheres_scene();
        }
        2 => {
            (cam, world) = two_spheres_scene();
        }
        3 => {
            (cam, world) = earth_scene();
        }
        4 => {
            (cam, world) = two_perlin_spheres_scene();
        }
        5 => {
            (cam, world) = quads_scene();
        }
        6 => {
            (cam, world) = simple_list_scene();
        }
        7 => {
            (cam, world) = cornell_box_scene();
        }
        8 => {
            (cam, world) = cornell_smoke_scene();
        }
        _ => {
            panic!("This scene is unknown!")
        }
    }
    //Open Image
    let mut out_file = File::create("./image.ppm").expect("Couldn't Open File!");

    cam.render(&mut out_file, &world);
}

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder.spawn(|| {
        uncapped_main();
    }).unwrap();

    handler.join().unwrap();
}