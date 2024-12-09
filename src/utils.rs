use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_between(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_f64() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}
