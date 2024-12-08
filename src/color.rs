use crate::{interval::Interval, vec3::Vec3};
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::with_values(0.000, 0.999);

    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
