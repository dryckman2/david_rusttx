mod camera;
mod hittables;
mod materials;
mod math_structures;
mod rtw_image;
mod rtweekend;
mod scenes;
mod textures;
mod volume;

mod live_render;
mod multithreading;
mod pdf;
mod winsdl;

use scenes::chat_scene::ChatScene;
use scenes::purple_cornell_box_scene::PurpleCornellBoxScene;

// use crate::live_render::show_screen;
use crate::live_render::show_screen;
use crate::multithreading::render_to_memory;
use crate::scenes::cornell_box_scene::CornellBoxScene;
use crate::scenes::cornell_smoke_scene::CornellSmokeScene;
use crate::scenes::diff_final_scene::DiffFinalScene;
use crate::scenes::earth_in_a_ball::EarthInABallScene;
use crate::scenes::earth_scene::EarthScene;
use crate::scenes::final_scene::FinalScene;
use crate::scenes::quads_scene::QuadsScene;
use crate::scenes::random_spheres_scene::RandomSpheresScene;
use crate::scenes::simple_list_scene::SimpleListScene;
use crate::scenes::two_perlin_spheres_scene::TwoPerlinSpheresScene;
use crate::scenes::two_spheres_scene::TwoSpheresScene;
use crate::scenes::Scene;
use std::fs::File;
use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::thread::{self};

pub const NUM_OF_ACTIVE_THREADS: usize = 12;
pub const IMAGE_WIDTH: i64 = 1080;
pub const SAMPLE_PP: i64 = 1000;
pub const MAX_DEPTH: i64 = 50;

fn main() {
    let mut scene;
    let i = scene_selector().unwrap();
    match i {
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
        10 => {
            scene = Box::new(EarthInABallScene::blank());
        }
        11 => {
            scene = Box::new(DiffFinalScene::blank());
        }
        12 => {
            scene = Box::new(PurpleCornellBoxScene::blank());
        }
        13 => {
            scene = Box::new(ChatScene::blank());
        }
        _ => {
            panic!("Invalid Scene Selected")
        }
    };
    scene.generate_scene(IMAGE_WIDTH, SAMPLE_PP, MAX_DEPTH);

    let (tx, rx) = channel();

    let cam = scene.get_cam().clone();
    let width = cam.image_width;
    let height = cam.image_height;
    let world = scene.get_world().clone();
    let lights = scene.get_lights().clone();
    let h = thread::spawn(|| render_to_memory(cam, world, lights, tx));

    show_screen(width as usize, height as usize, rx).unwrap();

    //After Image is closed write results to file
    let mut out_file = File::create("image_output.ppm").unwrap();
    let res = h.join().unwrap();
    for y in res {
        out_file.write(y.as_bytes()).expect("TODO: panic message");
    }
}

fn scene_selector() -> Result<i64, Box<dyn std::error::Error>> {
    println!("{SCENE_LIST}");
    println!("Select Scene:");
    let mut buff = String::new();
    let _buff_n = io::stdin().read_line(&mut buff)?;
    let i = buff.trim().parse()?;
    Ok(i)
}

const SCENE_LIST: &'static str = " 1 => Quads Scene
 2 =>Two Perlin Spheres Scene
 3 => Random Sphere Scene
 4 => Two Spheres Scene
 5 => Earth Scene (Slow)
 6 => Simple List Scene
 7 => Cornell Box Scene
 8 => Cornell Smoke Scene (Slow)
 9 => Final Scene (Slow!)
10 => Earth In A Ball Scene (Not working)
11 => Different Final Scene (Kinda Slow)
12 => Purple Cornell Box Scene
13 => Chat Scene\
";
