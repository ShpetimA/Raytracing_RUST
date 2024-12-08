use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::with_values(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(Sphere::new(
        Point3::with_values(0.0, -100.5, -1.0),
        100.0,
    )));
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.smaples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
