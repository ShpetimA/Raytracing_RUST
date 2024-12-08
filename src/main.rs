use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use color::{write_color, Color};
use ray::Ray;
use vec3::{Point3, Vec3};

pub mod color;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Point3::with_values(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::with_values(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::with_values(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::with_values(1.0, 1.0, 1.0) + a * Color::with_values(0.5, 0.7, 1.0);
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = *center - r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1
    }

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new();

    let viewport_u = Vec3::with_values(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::with_values(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center
        - Vec3::with_values(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let path = Path::new("output.ppm");
    let display = path.display();
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create file {} {}", display, why),
    };

    let header = format!("P3 \n{image_width} {image_height} \n255\n");
    file.write_all(header.as_bytes())
        .expect("Unable to write to file");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::with_values(camera_center, ray_direction);

            let pixel_color = ray_color(ray);
            write_color(&mut file, pixel_color).expect("Unable to write color to file")
        }
    }
    eprint!("\r{}", " ".repeat(30));
    eprint!("\rDone.\n");
    io::stderr().flush().unwrap();
}
