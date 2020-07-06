extern crate array2d;
use array2d::Array2D;

const RE_MIN: f64 = -2.5;
const RE_MAX: f64 = 1.0;
const IM_MIN: f64 = -1.0;
const IM_MAX: f64 = 1.0;
const MAX_ITERATIONS: u64 = 255;

#[derive(Clone, Default, Copy)]
pub struct Color{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

pub struct PixelsArray(pub Array2D<Color>);

impl From<PixelsArray> for Vec<u8> {
    fn from(input: PixelsArray) -> Self {
        let mut result_vec: Vec<u8> = Vec::new();

        let PixelsArray(array_pixels) = input;

        for row_i in 0..array_pixels.num_rows() {
            array_pixels.row_iter(row_i).for_each(|&val| result_vec.extend([val.red, val.green, val.blue, val.alpha].iter()));
        }

        result_vec
    }
}

pub fn generate(length: usize, width: usize) -> PixelsArray {
    generate_portion(length, width, (0, length), (0, width))
}

pub fn generate_portion(length: usize, width: usize, y_range: (usize, usize), x_range: (usize, usize)) -> PixelsArray {
    let result_width = x_range.1 - x_range.0;
    let result_length = y_range.1 - y_range.0;
    let mut result = Array2D::filled_with(Color::default(), result_length, result_width); 

    for x in x_range.0..x_range.1 {
        for y in y_range.0..y_range.1 {
            let x0: f64 = RE_MIN + ((x as f64 / width as f64)) *(RE_MAX - RE_MIN);
            let y0: f64 = IM_MIN + ((y as f64 / length as f64))*(IM_MAX - IM_MIN);

            result.set(y - y_range.0, x - x_range.0, get_pixel_color(x0, y0)).unwrap();
        }
    }

    PixelsArray(result)
}

fn get_pixel_color(x0: f64, y0: f64) -> Color {
    let mut iterations: u64 = 0;

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    while x*x + y*y <= 4.0 && iterations < MAX_ITERATIONS {
        let x_tmp = x*x - y*y + x0;
        y = 2.0*x*y + y0;
        x = x_tmp;
        iterations += 1;
    }

    // let hue = 255.0 * (iterations as f64 / MAX_ITERATIONS as f64);
    // let saturation = 255;
    // let value = if iterations < MAX_ITERATIONS {255} else {0};
    
    // hsv_to_rgb(hue, saturation as f64, value as f64)

    let grayscale = ((iterations as f64 / MAX_ITERATIONS as f64) * 255.0) as u8;
    Color{
        red: grayscale,
        green: grayscale,
        blue: grayscale,
        alpha: 255,
    }
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Color {
    if s == 0.0 {
        return Color {red: v as u8, green: v as u8, blue: v as u8, alpha: 255}
    }

    let h = h / 60.0;
    let i = h.floor();
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i as u8 {
        0 => Color{red: v as u8, green: t as u8, blue: p as u8, alpha: 255},
        1 => Color{red: q as u8, green: v as u8, blue: p as u8, alpha: 255},
        2 => Color{red: p as u8, green: v as u8, blue: t as u8, alpha: 255},
        3 => Color{red: p as u8, green: q as u8, blue: v as u8, alpha: 255},
        4 => Color{red: t as u8, green: p as u8, blue: v as u8, alpha: 255},
        _ => Color{red: v as u8, green: p as u8, blue: q as u8, alpha: 255},
    }
}
