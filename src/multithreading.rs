use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::math_structures::color::{write_color_string, Color};
use crate::NUM_OF_ACTIVE_THREADS;
use indicatif::ProgressBar;
use sorted_vec::SortedVec;
use std::io::Write;
use std::ops::Deref;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::time::Instant;
use threadpool::ThreadPool;

pub fn render_to_memory(
    camera: Arc<Camera>,
    world: Arc<HittableList>,
    lights: Arc<HittableList>,
    pixel_pipe: Sender<(i64, i64, Color)>,
) -> Vec<String> {
    let start_time = Instant::now();

    let pool = ThreadPool::new(NUM_OF_ACTIVE_THREADS);

    let (tx, rx) = channel();

    println!("Submitting Work To Threads...");
    let bar = Arc::new(ProgressBar::new(camera.image_height as u64 + 1));
    bar.inc(1);
    std::io::stdout().flush().unwrap();

    for i in 0..camera.image_height {
        let threads_cam = camera.deref().clone();
        let threads_world = world.deref().clone();
        let threads_lights = lights.deref().clone();
        let progress = Arc::clone(&bar);
        let thread_tx = tx.clone();
        let thread_pixel_pipe = pixel_pipe.clone();

        pool.execute(move || {
            thread_tx
                .send(thread_render(
                    threads_cam,
                    threads_world,
                    threads_lights,
                    i,
                    progress,
                    thread_pixel_pipe,
                ))
                .unwrap();
        });
    }

    // let mut results = Vec::with_capacity(camera.image_height as usize + 1);
    let mut results = SortedVec::with_capacity(camera.image_height as usize + 1);
    rx.iter().take(camera.image_height as usize).for_each(|n| {
        results.push(n);
    });

    // Render
    results.push((
        -1,
        format!("P3\n{} {}\n255\n", camera.image_width, camera.image_height),
    ));

    //Double Check All Jobs are finished; should be unnecessary
    pool.join();

    drop(pixel_pipe);

    let time_took = start_time.elapsed();
    bar.finish();
    println!("Done! Took {:?}", time_took);
    results.iter().map(|n| n.1.clone()).collect()
}

pub fn thread_render(
    cam: Camera,
    world: HittableList,
    lights: HittableList,
    row_num: i64,
    progress: Arc<ProgressBar>,
    pixel_pipe: Sender<(i64, i64, Color)>,
) -> (i64, String) {
    let j = row_num;
    let mut s = String::new();
    for i in 0..cam.image_width {
        let mut pixel_color = Color::blank();
        for s_j in 0..(cam.sqrt_spp as i64) {
            for s_i in 0..(cam.sqrt_spp as i64) {
                let r = cam.get_ray(i, j, s_i, s_j);
                let ray_color = &cam.ray_color(&r, cam.max_depth, &world, &lights);
                pixel_pipe.send((i, j, ray_color.clone())).unwrap();
                pixel_color += ray_color;
            }
        }
        s += &*write_color_string(&pixel_color, cam.samples_per_pixel);
    }
    progress.inc(1);
    (j, s)
}
