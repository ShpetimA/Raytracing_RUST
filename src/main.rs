use std::sync::Arc;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

fn main() {
    let mut world = HittableList::new();
    let material_ground = Arc::new(Lambertian::new(Color::with_values(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::with_values(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(Color::with_values(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::with_values(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Box::new(Sphere::new(
        Point3::with_values(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Box::new(Sphere::new(
        Point3::with_values(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::with_values(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Point3::with_values(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.smaples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
