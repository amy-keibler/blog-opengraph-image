use color_eyre::eyre::Result;
use eyre::eyre;
use image::GenericImage;
use imageproc::drawing::{draw_filled_circle_mut, draw_hollow_circle_mut, draw_text_mut};
use rand::prelude::*;
use rusttype::{Font, Scale};
use serde::Deserialize;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

static IMAGE_WIDTH: u32 = 1200;
static IMAGE_HEIGHT: u32 = 630;
static BORDER_SIZE: u32 = 10;
static BACKGROUND_COLOR: [u8; 3] = [248, 240, 255];
static PRIMARY_COLOR: [u8; 3] = [112, 33, 186];
static PRIMARY_DESATURATED_COLOR: [u8; 3] = [130, 61, 194];
static SECONDARY_COLOR: [u8; 3] = [235, 214, 255];
static TEXT_SIZE: f32 = 60.0;
static TEXT_PADDING: u32 = 30;

#[derive(PartialEq, Debug, Deserialize)]
struct Article {
    title: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let article_file: String = std::env::args()
        .skip(1)
        .next()
        .ok_or(eyre!("Expected a filepath to a blog article"))?;
    let article_path = Path::new(&article_file);
    if article_path.is_file() {
        let article = parse_article(article_path)?;
        let article_image_path = article_path.with_extension("png");
        generate_image(&article_image_path, article)?;
    } else {
        Err(eyre!("{} passed in, but it does not exists", article_file))?;
    }

    Ok(())
}

fn parse_article(article_path: &Path) -> Result<Article> {
    let article_file = BufReader::new(File::open(article_path)?);
    let article_contents: String = article_file
        .lines()
        .filter_map(|line_result| line_result.ok())
        .skip_while(|line| !line.starts_with("+++"))
        .skip_while(|line| line.starts_with("+++"))
        .take_while(|line| !line.starts_with("+++"))
        .collect::<Vec<String>>()
        .join("\n");
    let article = toml::from_str(&article_contents)?;
    Ok(article)
}

fn generate_image(article_image_path: &PathBuf, article: Article) -> Result<()> {
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = image::Rgb(BACKGROUND_COLOR);
    }

    add_circles(&mut image);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x <= BORDER_SIZE
            || x >= (IMAGE_WIDTH - BORDER_SIZE)
            || y <= BORDER_SIZE
            || y >= (IMAGE_HEIGHT - BORDER_SIZE)
        {
            *pixel = image::Rgb(PRIMARY_COLOR);
        } else if y <= BORDER_SIZE + TEXT_SIZE as u32 + TEXT_PADDING {
            *pixel = image::Rgb(SECONDARY_COLOR);
        }
    }

    let font_data: &[u8] = include_bytes!("/usr/share/fonts/TTF/DejaVuSans.ttf");
    let font: Font<'static> =
        Font::try_from_bytes(font_data).ok_or(eyre!("Could not load font"))?;

    let scale = Scale {
        x: TEXT_SIZE,
        y: TEXT_SIZE,
    };
    draw_text_mut(
        &mut image,
        image::Rgb(PRIMARY_COLOR),
        TEXT_PADDING,
        TEXT_PADDING,
        scale,
        &font,
        &article.title,
    );

    image.save(article_image_path)?;

    Ok(())
}

fn add_circles(image: &mut image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
    for _ in 0..30 {
        let x_range: f32 = rand::random();
        let y_range: f32 = rand::random();
        let radius_range: f32 = rand::random();
        let position = (
            (IMAGE_WIDTH as f32 * x_range) as i32,
            (IMAGE_HEIGHT as f32 * y_range) as i32,
        );
        let radius = (60.0 * radius_range) as i32 + 20;
        draw_filled_circle_mut(image, position, radius, image::Rgb(SECONDARY_COLOR));

        draw_hollow_circle_mut(
            image,
            position,
            radius,
            image::Rgb(PRIMARY_DESATURATED_COLOR),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_process_zola_frontmatter() {
        let article_path = Path::new("tests/blog.md");
        assert_eq!(true, article_path.is_file());
        let article = parse_article(article_path).expect("Could not read article");
        assert_eq!(
            Article {
                title: "Principles of Technology Leadership".to_owned()
            },
            article
        );
    }
}
