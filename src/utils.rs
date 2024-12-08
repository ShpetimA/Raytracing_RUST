use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random_between(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_f32() -> f32 {
    rand::thread_rng().gen_range(0.0..1.0)
}
