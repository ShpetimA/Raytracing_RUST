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
    utils::{degrees_to_radians, random_f64},
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub smaples_per_pixel: i32,
    pub max_depth: i32,
    pub v_fov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    //camera frame bassis vecs
    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
            v_fov: 90.0,
            lookfrom: Point3::with_values(0.0, 0.0, 0.0),
            lookat: Point3::with_values(0.0, 0.0, -1.0),
            vup: Vec3::with_values(0.0, 1.0, 0.0),
            u: Vec3::new(),
            v: Vec3::new(),
            w: Vec3::new(),
            defocus_angle: 10.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::new(),
            defocus_disk_v: Vec3::new(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1
        }
        self.pixel_samples_scale = 1.0 / self.smaples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
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

        let ray_origin = if self.focus_dist > 0.0 {
            self.defocus_disk_sample()
        } else {
            self.center
        };

        let ray_direction = pixel_sample - ray_origin;

        return Ray::with_values(ray_origin, ray_direction);
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        return self.center + (p * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3::with_values(random_f64() - 0.5, random_f64() - 0.5, 0.0);
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth < 0 {
            return Color::new();
        }

        let mut rec = HitRecord::new();

        if world.hit(&r, Interval::with_values(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::new();
            let mut attenuation = Color::new();
            let mat = rec.mat.clone();

            if mat.scatter(&r, &mut rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(scattered, depth - 1, world);
            }

            return Color::new();
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::with_values(1.0, 1.0, 1.0)
            + a * Color::with_values(0.5, 0.7, 1.0);
    }
}
