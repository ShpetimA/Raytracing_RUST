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
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
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
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::with_values(self.center, ray_direction);

                let pixel_color = self.ray_color(ray, world);
                write_color(&mut file, pixel_color).expect("Unable to write color to file")
            }
        }
        eprint!("\r{}", " ".repeat(30));
        eprint!("\rDone.\n");
        io::stderr().flush().unwrap();
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1
        }

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);
        self.center = Point3::new();

        let viewport_u = Vec3::with_values(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::with_values(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left = self.center
            - Vec3::with_values(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: Ray, world: &dyn Hittable) -> Color {
        let mut hit_record = HitRecord::new();

        if world.hit(
            &r,
            Interval::with_values(0.0, f32::INFINITY),
            &mut hit_record,
        ) {
            return 0.5 * (hit_record.normal + Color::with_values(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::with_values(1.0, 1.0, 1.0)
            + a * Color::with_values(0.5, 0.7, 1.0);
    }
}
