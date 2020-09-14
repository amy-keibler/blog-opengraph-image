use image::{ImageBuffer, Rgb};
use imageproc::drawing::{draw_filled_circle_mut, draw_hollow_circle_mut};

use crate::{
    BACKGROUND_COLOR, IMAGE_HEIGHT, IMAGE_WIDTH, PRIMARY_DESATURATED_COLOR, SECONDARY_COLOR,
};

pub fn fill_background(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (_x, _y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb(BACKGROUND_COLOR);
    }
}

pub fn add_circles(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for _ in 0..40 {
        let x_range: f32 = rand::random();
        let y_range: f32 = rand::random();
        let radius_range: f32 = rand::random();
        let position = (
            (IMAGE_WIDTH as f32 * x_range) as i32,
            (IMAGE_HEIGHT as f32 * y_range) as i32,
        );
        let radius = (60.0 * radius_range) as i32 + 20;
        draw_filled_circle_mut(image, position, radius, Rgb(SECONDARY_COLOR));

        draw_hollow_circle_mut(image, position, radius, Rgb(PRIMARY_DESATURATED_COLOR));
    }
}
