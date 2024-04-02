use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::math_structures::color::{write_color_string, Color};
use indicatif::ProgressBar;
use std::sync::Arc;
use std::thread;

pub fn render_to_memory(
    camera: Arc<Camera>,
    world: Arc<HittableList>,
    num_threads: usize,
) -> Vec<String> {
    let even_load = camera.image_height / num_threads as i64;
    let mut handles = vec![];

    let bar = Arc::new(ProgressBar::new(
        (camera.image_height * camera.image_width) as u64,
    ));

    for i in 0..num_threads {
        let start = i * even_load as usize;
        let end = (i + 1) * even_load as usize;
        let threads_cam = Arc::clone(&camera);
        let threads_world = Arc::clone(&world);
        let progress = Arc::clone(&bar);
        handles.push(thread::spawn(move || {
            thread_render(threads_cam, threads_world, start, end, progress)
        }));
    }

    // If a rounding error occurs, fill in the gap with an extra thread
    if ((num_threads) * even_load as usize) != camera.image_height as usize {
        let start = (num_threads) * even_load as usize;
        let end = camera.image_height as usize;
        let threads_cam = Arc::clone(&camera);
        let threads_world = Arc::clone(&world);
        let progress = Arc::clone(&bar);

        handles.push(thread::spawn(move || {
            thread_render(threads_cam, threads_world, start, end, progress)
        }));
    }
    let mut results = Vec::with_capacity((camera.image_height * camera.image_width) as usize + 1);
    // Render
    results.push(format!(
        "P3\n{} {}\n255\n",
        camera.image_width, camera.image_height
    ));

    for h in handles {
        let x = h.join().unwrap();
        for y in x {
            results.push(y);
        }
    }
    bar.finish();

    println!("Done!");
    results
}

pub fn thread_render(
    cam: Arc<Camera>,
    world: Arc<HittableList>,
    start: usize,
    end: usize,
    progress: Arc<ProgressBar>,
) -> Vec<String> {
    let mut res = Vec::with_capacity(end - start);
    for j in start..end {
        for i in 0..cam.image_width {
            let mut pixel_color = Color::blank();
            for _ in 0..cam.samples_per_pixel {
                let r = cam.get_ray(i, j as i64);
                pixel_color += &cam.ray_color(&r, cam.max_depth, &world);
            }
            progress.inc(1);

            let s = write_color_string(&pixel_color, cam.samples_per_pixel);
            res.push(s)
        }
    }
    res
}
