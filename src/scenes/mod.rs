use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use std::sync::Arc;

pub mod cornell_box_scene;
pub mod cornell_smoke_scene;
pub mod earth_scene;
pub mod final_scene;
pub mod quads_scene;
pub mod random_spheres_scene;
pub mod simple_list_scene;
pub mod two_perlin_spheres_scene;
pub mod two_spheres_scene;

pub trait Scene {
    fn generate_scene(&mut self, image_width: i64, samples_per_pixel: i64, max_depth: i64);

    fn get_cam(&self) -> Arc<Camera>;

    fn get_world(&self) -> Arc<HittableList>;
    fn get_lights(&self) -> Arc<HittableList>;
}
