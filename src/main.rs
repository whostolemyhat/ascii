extern crate image;

use std::fs::File;
use std::path::Path;

use image::GenericImage;

fn main() {
    // read in image
    let path = &Path::new("/Users/james/Documents/rust/ascii/img/pic.jpg");
    let mut img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    println!("Size: {:?}, {:?}", width, height);

    let ref mut out = File::create(&Path::new("/Users/james/Documents/rust/ascii/out/test.png")).unwrap();

    // convert to greyscale
    let _ = img.grayscale().save(out, image::PNG).unwrap();

    // split into zones
    let zone_size = 10;
    let char_map = [".", ",", ":", ";", "o", "x", "%", "#", "@"];
    let map_length = char_map.len();
    let mut output = "".to_string();
    output = output + "test\r\n";

    // step_by doesn't exist yet :(
    // for x in (0..size.0).step_by(zone_size) {
    for y in 0..(height) {
        if y % zone_size == 0 {
            println!("row {:?}", y / 10);

            for x in 0..(width) {
                if x % zone_size == 0 {
                    // println!("{:?}:{:?}", x, y);

                    let sub_img = img.crop(x, y, x + zone_size, y + zone_size);
                    // let ref mut out = File::create(&Path::new("/Users/james/Documents/rust/ascii/out/test.png")).unwrap();
                    // let _ = sub_img.save(out, image::PNG);

                    // get intensity of sub img
                    // for pixel in sub_img
                    let mut r: f32 = 0.0;
                    let mut g: f32 = 0.0;
                    let mut b: f32 = 0.0;
                    let mut count = 0;
                    for (x, y, pixel) in sub_img.pixels() {
                        // println!("{:?}", pixel.data);
                        r = r + pixel.data[0] as f32;
                        g = g + pixel.data[1] as f32;
                        b = b + pixel.data[2] as f32;

                        count = count + 1;
                    }
                    // println!("{:?}, {:?}, {:?} / {:?}", r, g, b, count);
                    println!("{:?}", (((r * 0.3) + (b * 0.3) + (g * 0.3)) / count as f32).floor());
                    // add up all r, g, b vals
                    // divide by number of pixels in sub_img = average
                    // char_map[ (255 - average) * map_length/256 ] 
                }
            }
        }
    }

    // get intensity
}
