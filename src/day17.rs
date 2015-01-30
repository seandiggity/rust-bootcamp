#![feature(io, path, rand)]

extern crate image;
extern crate nalgebra;

use image::Pixel;
use nalgebra::DMat;

use std::old_io::File;
use std::rand;

fn main() {
    println!("24 days of Rust - patterns (day 17)");
    let v = (0us..10).map(|x| x * 3).collect::<Vec<_>>();
    println!("{:?}", v);
    let v = (0..10).map(|_| rand::random::<u32>()).collect::<Vec<_>>();
    println!("{:?}", v);
    let mat: DMat<u32> = DMat::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
    println!("{:?}", mat);
    let buffer = image::ImageBuffer::from_fn(512u32, 512u32, Box::new(|&: x: u32, y: u32| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    }));
    let img = image::DynamicImage::ImageRgba8(buffer);
    let out = File::create(&Path::new("out_pattern.png")).unwrap();
    let _ = img.save(out, image::PNG);
}
