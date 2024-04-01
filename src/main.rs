mod camera;
mod materials;
mod rtw_image;
mod rtweekend;
mod scenes;
mod hittables;
mod textures;
mod math_structures;
mod volume;

use std::fs::File;
use crate::scenes::{cornell_box_scene, cornell_smoke_scene, earth_scene, quads_scene, random_spheres_scene, simple_list_scene, two_perlin_spheres_scene, two_spheres_scene};


fn main() {
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
