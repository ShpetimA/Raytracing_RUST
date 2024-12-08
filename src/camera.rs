use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::random_f32,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub smaples_per_pixel: i32,
    pub max_depth: i32,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 100,
            center: Point3::new(),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            smaples_per_pixel: 10,
            pixel_samples_scale: 1.0,
            max_depth: 10,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let path = Path::new("output.ppm");
        let display = path.display();
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't create file {} {}", display, why),
        };

        let image_width = self.image_width;
        let image_height = self.image_height;
        let header = format!("P3 \n{image_width} {image_height} \n255\n");
        file.write_all(header.as_bytes())
            .expect("Unable to write to file");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.smaples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(ray, self.max_depth, world);
                }
                write_color(&mut file, self.pixel_samples_scale * pixel_color)
                    .expect("Unable to write color to file")
            }
        }
        eprint!("\r{}", " ".repeat(30));
        eprint!("\rDone.\n");
        io::stderr().flush().unwrap();
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::with_values(ray_origin, ray_direction);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3::with_values(random_f32() - 0.5, random_f32() - 0.5, 0.0);
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1
        }
        self.pixel_samples_scale = 1.0 / self.smaples_per_pixel as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point3::new();

        let viewport_u = Vec3::with_values(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::with_values(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center
            - Vec3::with_values(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth < 0 {
            return Color::new();
        }

        let mut rec = HitRecord::new();

        if world.hit(&r, Interval::with_values(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();

            return 0.5 * self.ray_color(Ray::with_values(rec.p, direction), depth - 1, world);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::with_values(1.0, 1.0, 1.0)
            + a * Color::with_values(0.5, 0.7, 1.0);
    }
}
