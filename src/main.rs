extern crate image;

use std::fs::File;
use std::path::Path;

use image::GenericImage;
// use image::imageops;

fn main() {
    // read in image
    let path = &Path::new("/Users/james/Documents/rust/ascii/img/pic.jpg");
    let mut img = image::open(path).unwrap();
    // img.grayscale();
    let size = img.dimensions();

    println!("Size: {:?}", size);

    let ref mut out = File::create(&Path::new("/Users/james/Documents/rust/ascii/out/test.png")).unwrap();

    // convert to greyscale
    let _ = img.grayscale().save(out, image::PNG).unwrap();

    // split into zones
    let zone_size = 10;

    // step_by doesn't exist yet :(
    // for x in (0..size.0).step_by(zone_size) {
    for y in 0..size.1 {
        if y % zone_size == 0 {
            println!("row {:?}", y / 10);
            
            for x in 0..size.0 {
                if x % zone_size == 0 {
                    // println!("{:?}:{:?}", x, y);

                    let sub_img = img.crop(x, y, x + zone_size, y + zone_size);
                    // let ref mut out = File::create(&Path::new("/Users/james/Documents/rust/ascii/out/test.png")).unwrap();
                    // let _ = sub_img.save(out, image::PNG);

                    // get intensity of sub img
                }
            }
        }
    }

    // get intensity
}
