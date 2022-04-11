use eyre::Result;
use image::imageops::overlay;
use image::{load_from_memory_with_format, ImageBuffer, ImageFormat, Rgba};
use imageproc::drawing::{draw_filled_circle_mut, draw_hollow_circle_mut};

use crate::article::ArticleInformation;

use crate::{
    BACKGROUND_COLOR, IMAGE_HEIGHT, IMAGE_WIDTH, PRIMARY_DESATURATED_COLOR, SECONDARY_COLOR,
};

pub fn render_background(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    article_information: &ArticleInformation,
) -> Result<()> {
    fill_background(image);

    if article_information.tags.contains(&String::from("rust")) {
        add_ferris(image)?;
    } else {
        add_circles(image);
    }
    Ok(())
}

fn fill_background(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (_x, _y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba(BACKGROUND_COLOR);
    }
}

const FERRIS_IMAGE_SIZE: u32 = 100;
const FERRIS_PADDING_SIZE: u32 = 25;
const FERRIS_SQUARE_SIZE: u32 = FERRIS_IMAGE_SIZE + 2 * FERRIS_PADDING_SIZE;
const FERRISES_IN_A_ROW: u32 = IMAGE_WIDTH / FERRIS_SQUARE_SIZE;
const FERRIS_DRAW_THRESHOLD: f32 = 0.4;

fn add_ferris(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<()> {
    let ferris = include_bytes!("../images/ferris.png");
    let ferris = load_from_memory_with_format(ferris, ImageFormat::Png)?;

    for y in 0..(IMAGE_HEIGHT / FERRIS_SQUARE_SIZE) {
        let odd_row = Some(1) == u32::checked_rem(y, 2);
        let draw_x_offset = if odd_row { FERRIS_SQUARE_SIZE / 2 } else { 0 };
        for x in 0..(FERRISES_IN_A_ROW) {
            let draw_chance: f32 = rand::random();
            if draw_chance <= FERRIS_DRAW_THRESHOLD {
                if !odd_row || x + 1 != FERRISES_IN_A_ROW {
                    overlay(
                        image,
                        &ferris,
                        (x * FERRIS_SQUARE_SIZE + FERRIS_PADDING_SIZE + draw_x_offset).into(),
                        (y * FERRIS_SQUARE_SIZE + FERRIS_PADDING_SIZE).into(),
                    );
                }
            }
        }
    }
    Ok(())
}

fn add_circles(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for _ in 0..40 {
        let x_range: f32 = rand::random();
        let y_range: f32 = rand::random();
        let radius_range: f32 = rand::random();
        let position = (
            (IMAGE_WIDTH as f32 * x_range) as i32,
            (IMAGE_HEIGHT as f32 * y_range) as i32,
        );
        let radius = (60.0 * radius_range) as i32 + 20;
        draw_filled_circle_mut(image, position, radius, Rgba(SECONDARY_COLOR));

        draw_hollow_circle_mut(image, position, radius, Rgba(PRIMARY_DESATURATED_COLOR));
    }
}
