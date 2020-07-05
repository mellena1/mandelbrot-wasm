extern crate bmp;
use bmp::{Image, Pixel};

use std::io;

mod mandelbrot;

fn main() {
    let pixel_matrix = mandelbrot::generate(3000, 4500);

    make_bmp(pixel_matrix, "mandelbrot.bmp").unwrap();

    let pixel_matrix = mandelbrot::generate_portion(3000, 4500, (500, 1000), (1000, 2000));

    make_bmp(pixel_matrix, "mandelbrot_portion.bmp").unwrap();
}

fn make_bmp(pixel_matrix: mandelbrot::PixelsArray, file_name: &str) -> io::Result<()> {
    let mandelbrot::PixelsArray(pixel_matrix) = pixel_matrix;

    let mut img = Image::new(pixel_matrix.num_columns() as u32, pixel_matrix.num_rows() as u32);

    for (x, y) in img.coordinates() {
        let pixel_val = *pixel_matrix.get(y as usize, x as usize).unwrap();
        let pixel = Pixel::new(pixel_val.red, pixel_val.green, pixel_val.blue);
        img.set_pixel(x, y, pixel);
    }

    img.save(file_name)
}
