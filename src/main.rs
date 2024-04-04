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
mod pdf;

use std::thread::Builder;

use crate::scenes::cornell_box_scene::CornellBoxScene;
use crate::scenes::cornell_smoke_scene::CornellSmokeScene;
use crate::scenes::earth_scene::EarthScene;
use crate::scenes::final_scene::FinalScene;
use crate::scenes::quads_scene::QuadsScene;
use crate::scenes::random_spheres_scene::RandomSpheresScene;
use crate::scenes::simple_list_scene::SimpleListScene;
use crate::scenes::two_perlin_spheres_scene::TwoPerlinSpheresScene;
use crate::scenes::two_spheres_scene::TwoSpheresScene;
use crate::scenes::Scene;
use std::fs::File;
use std::io::Write;

pub const NUM_OF_ACTIVE_THREADS: usize = 12;
pub const IMAGE_WIDTH: i64 = 800;
pub const SAMPLE_PP: i64 = 1_000;
pub const MAX_DEPTH: i64 = 50;

fn uncapped_main() {
    let mut scene;
    match 1 {
        1 => {
            scene = Box::new(QuadsScene::blank()) as Box<dyn Scene>;
        }
        2 => {
            scene = Box::new(TwoPerlinSpheresScene::blank());
        }
        3 => {
            scene = Box::new(RandomSpheresScene::blank());
        }
        4 => {
            scene = Box::new(TwoSpheresScene::blank());
        }
        5 => {
            scene = Box::new(EarthScene::blank());
        }
        6 => {
            scene = Box::new(SimpleListScene::blank());
        }
        7 => {
            scene = Box::new(CornellBoxScene::blank());
        }
        8 => {
            scene = Box::new(CornellSmokeScene::blank());
        }
        9 => {
            scene = Box::new(FinalScene::blank());
        }
        _ => {
            panic!("Invalid Scene Selected")
        }
    };
    scene.generate_scene(IMAGE_WIDTH, SAMPLE_PP, MAX_DEPTH);
    let output_file = "./image_output.ppm";

    //Open Image
    let mut out_file = File::create(output_file).expect("Couldn't Open File!");
    println!("World Setup Complete!");
    std::io::stdout().flush().unwrap();
    let cam = scene.get_cam();
    let world = scene.get_world();
    let lights = scene.get_lights();
    cam.multi_threaded_render(&mut out_file, &world, &lights);
}

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024 * 4); // 128MB of stack space

    let handler = builder
        .spawn(|| {
            uncapped_main();
        })
        .unwrap();

    handler.join().unwrap();
}
