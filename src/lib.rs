extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;

mod mandelbrot;

#[wasm_bindgen]
pub fn draw_mandelbrot() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    // let width = window.inner_width().unwrap().as_f64().unwrap();
    // let height = window.inner_height().unwrap().as_f64().unwrap();

    let width = canvas.width();
    let height = canvas.height();

    // let width_str = format!("{}", width);
    // canvas.set_attribute("width", width_str.as_str()).unwrap();
    // let height_str = format!("{}", height);
    // canvas.set_attribute("height", height_str.as_str()).unwrap();


    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let array_pixels = mandelbrot::generate(height as usize, width as usize);

    let mut result_vec: Vec<u8> = array_pixels.into();

    let clamped_array_pixels = Clamped(result_vec.as_mut_slice());
    let canvas_pixels = web_sys::ImageData::new_with_u8_clamped_array(clamped_array_pixels, width as u32).unwrap();

    context.put_image_data(&canvas_pixels, 0.0, 0.0).unwrap();
}
