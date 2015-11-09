extern crate image;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

use image::GenericImage;
use image::Pixel;

fn pixel_to_char(pixel: Pixel, map_length: usize) {
    let r = pixel.data[0] as f32;
    let g = pixel.data[1] as f32;
    let b = pixel.data[2] as f32;

    let average_shade = ((r * 0.3) + (b * 0.3) + (g * 0.3)).floor();
    
    ((255.0 - average_shade) * (map_length as f32 / 256.0)).floor()
}

fn main() {
    // read in image
    let path = &Path::new("/Users/james/Documents/rust/ascii/img/gret.jpg");
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    let char_map = [".", ",", ":", ";", "o", "x", "%", "#", "@"];
    let map_length = char_map.len();
    let mut output = "".to_string();
    output = output + "\r\n";

    for (x, _, pixel) in img.pixels() {
        // let r = pixel.data[0] as f32;
        // let g = pixel.data[1] as f32;
        // let b = pixel.data[2] as f32;

        // let average_shade = ((r * 0.3) + (b * 0.3) + (g * 0.3)).floor();
        
        // let index = ((255.0 - average_shade) * (map_length as f32 / 256.0)).floor();
        // let character = char_map[ index as usize ];
        let character = char_map[ pixel_to_char(pixel, map_length) ];
        output = output + character;

        if x == width - 1 {
            output = output + "\r\n";
        }
    }


    let path = Path::new("out/img.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file
    };

    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("Successfully saved to {}", display)
    }

}
