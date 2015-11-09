extern crate image;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

use image::GenericImage;

fn main() {
    // read in image
    let path = &Path::new("/Users/james/Documents/rust/ascii/img/pic-small.png");
    let mut img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    println!("Size: {:?}, {:?}", width, height);

    let ref mut out = File::create(&Path::new("/Users/james/Documents/rust/ascii/out/test.png")).unwrap();

    // convert to greyscale
    let _ = img.grayscale().save(out, image::PNG).unwrap();

    let char_map = [".", ",", ":", ";", "o", "x", "%", "#", "@"];
    let map_length = char_map.len();
    let mut output = "".to_string();
    output = output + "\r\n";

    for (x, y, pixel) in img.pixels() {
        println!("{:?}", pixel.data);
        // let (r: f32, g: f32, b: f32) = pixel.data as f32;
        let r = pixel.data[0] as f32;
        let g = pixel.data[1] as f32;
        let b = pixel.data[2] as f32;

        let average_shade = ((r * 0.3) + (b * 0.3) + (g * 0.3)).floor();
        
        let index = ((255.0 - average_shade) * (map_length as f32 / 256.0)).floor();
        let character = char_map[ index as usize ];
        output = output + character;
        if x == width - 1 {
            output = output + "\r\n";
        }
    }


    println!("{:?}", output);

    let path = Path::new("out/img.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file
    };
    // try!(f.write_all(output));

}
