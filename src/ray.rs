use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn with_values(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.dir;
    }
}
