mod camera;
mod hittables;
mod materials;
mod math_structures;
mod rtw_image;
mod rtweekend;
mod scenes;
mod textures;
mod volume;

mod multithreading;

use std::thread::Builder;

use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::scenes::{
    cornell_box_scene, cornell_smoke_scene, earth_scene, final_scene, quads_scene,
    random_spheres_scene, simple_list_scene, two_perlin_spheres_scene, two_spheres_scene,
};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

fn uncapped_main() {
    let cam: Arc<Camera>;
    let world: Arc<HittableList>;
    match 10 {
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
        9 => {
            (cam, world) = final_scene(200, 10, 4);
        }
        10 => {
            (cam, world) = final_scene(800, 1000, 40);
        }
        _ => {
            panic!("Fuck")
        }
    }

    let output_file = "./image.ppm";

    //Open Image
    let mut out_file = File::create(output_file).expect("Couldn't Open File!");
    println!("World Setup Complete!");
    std::io::stdout().flush().unwrap();
    cam.multi_threaded_render(&mut out_file, &world);
}

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder
        .spawn(|| {
            uncapped_main();
        })
        .unwrap();

    handler.join().unwrap();
}
