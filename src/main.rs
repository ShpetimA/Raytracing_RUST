use std::{fs::File, io::Write, path::Path};

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
            let r: f32 = i as f32 / (image_width as f32 - 1.0);
            let g: f32 = j as f32 / (image_height as f32 - 1.0);
            let b = 0.0;

            let ir = (r * 256.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            let pixel_data = format!("{ir} {ig} {ib} \n");
            file.write_all(pixel_data.as_bytes())
                .expect("Unable to write to file");
        }
    }
}
