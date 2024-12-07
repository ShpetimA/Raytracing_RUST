use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use color::{write_color, Color};

pub mod color;
pub mod vec3;

fn main() {
    let path = Path::new("hello_world.ppm");
    let display = path.display();
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create file {} {}", display, why),
    };

    let image_width = 256;
    let image_height = 256;

    let header = format!("P3 \n{image_width} {image_height} \n255\n");
    file.write_all(header.as_bytes())
        .expect("Unable to write to file");

    for j in 0..image_height {
        for i in 0..image_width {
            eprint!("\rScanlines remaining: {}", image_height - j);
            io::stderr().flush().unwrap();
            let pixel_color = Color::with_values(
                i as f32 / (image_width as f32 - 1.0),
                j as f32 / (image_height as f32 - 1.0),
                0.0,
            );
            write_color(&mut file, pixel_color).expect("Unable to write color to file")
        }
    }
    eprint!("\r{}", " ".repeat(30));
    eprint!("\rDone.\n");
    io::stderr().flush().unwrap();
}
