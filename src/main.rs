extern crate image;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

use image::GenericImage;

struct Rgb {
    r: f32,
    g: f32,
    b: f32
}

fn pixel_to_char(pixel: Rgb, map_length: usize) -> usize {
    let average_shade = ((pixel.r * 0.3) + (pixel.b * 0.3) + (pixel.g * 0.3)).floor();
    
    ((255.0 - average_shade) * (map_length as f32 / 256.0)).floor() as usize
}

fn main() {
    let path = Path::new("img/pic-tiny.jpg");
    let img = image::open(path).unwrap();
    let width = img.width();

    let char_map = [".", ",", ":", ";", "o", "x", "%", "#", "@"];
    let map_length = char_map.len();
    let mut output = "".to_string();

    for (x, _, pixel) in img.pixels() {
        let rgb = Rgb{
            r: pixel.data[0] as f32,
            g: pixel.data[1] as f32,
            b: pixel.data[2] as f32
        };

        let character = char_map[ pixel_to_char(rgb, map_length) ];
        output = output + character;

        if x == width - 1 {
            output = output + "\r\n";
        }
    }


    let output_path = Path::new("out/img.txt");
    let display = output_path.display();

    let mut file = match File::create(&output_path) {
        Err(why) => panic!("Couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file
    };

    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("Successfully saved to {}", display)
    }

}
