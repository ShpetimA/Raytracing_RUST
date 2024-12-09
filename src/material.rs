use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatered_dir = rec.normal + Vec3::random_unit_vector();

        if scatered_dir.near_zero() {
            scatered_dir = rec.normal;
        }

        *scattered = Ray::with_values(rec.p, scatered_dir);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.direction(), &rec.normal);

        *scattered = Ray::with_values(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}
