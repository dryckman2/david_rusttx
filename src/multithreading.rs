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
use std::thread;
use std::time::Instant;

pub fn render_to_memory(
    camera: Arc<Camera>,
    world: Arc<HittableList>,
    lights: Arc<HittableList>,
) -> Vec<String> {
    let start_time = Instant::now();

    let mut handles = vec![];

    let fair_share = camera.image_height / NUM_OF_ACTIVE_THREADS as i64;

    let (tx, rx) = channel();

    println!("Submitting Work To Threads...");
    let bar = Arc::new(ProgressBar::new(camera.image_height as u64 + 1));
    bar.inc(1);
    std::io::stdout().flush().unwrap();

    let mut last_end = 0;
    for i in 0..NUM_OF_ACTIVE_THREADS {
        let thread_start = fair_share * i as i64;
        let thread_end = fair_share * (i as i64 + 1);
        last_end = thread_end;

        let threads_cam = camera.deref().clone();
        let threads_world = world.deref().clone();
        let threads_lights = lights.deref().clone();
        let progress = Arc::clone(&bar);
        let thread_tx = tx.clone();

        handles.push(thread::spawn(move || {
            thread_render(
                threads_cam,
                threads_world,
                threads_lights,
                thread_start,
                thread_end,
                thread_tx,
                progress,
            );
        }));
    }

    if last_end != camera.image_height {
        let threads_cam = camera.deref().clone();
        let threads_world = world.deref().clone();
        let threads_lights = lights.deref().clone();
        let progress = Arc::clone(&bar);
        let thread_tx = tx.clone();
        let end = camera.image_height;
        handles.push(thread::spawn(move || {
            thread_render(
                threads_cam,
                threads_world,
                threads_lights,
                last_end,
                end,
                thread_tx,
                progress,
            );
        }));
    }

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
    for h in handles {
        h.join().unwrap();
    }

    // results.sort_by(|a, b| { a.0.cmp(&b.0) });

    let time_took = start_time.elapsed();
    bar.finish();
    println!("Done! Took {:?}", time_took);
    results.iter().map(|n| n.1.clone()).collect()
}

pub fn thread_render(
    cam: Camera,
    world: HittableList,
    lights: HittableList,
    start: i64,
    end: i64,
    chan: Sender<(i64, String)>,
    progress: Arc<ProgressBar>,
) {
    for j in start..end {
        let mut s = String::new();
        for i in 0..cam.image_width {
            let mut pixel_color = Color::blank();
            for s_j in 0..(cam.sqrt_spp as i64) {
                for s_i in 0..(cam.sqrt_spp as i64) {
                    let r = cam.get_ray(i, j, s_i, s_j);
                    pixel_color += &cam.ray_color(&r, cam.max_depth, &world, &lights);
                }
            }

            s += &*write_color_string(&pixel_color, cam.samples_per_pixel);
        }
        progress.inc(1);
        chan.send((j, s)).unwrap();
    }
}
