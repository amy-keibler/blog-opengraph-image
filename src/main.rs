use color_eyre::eyre::Result;
use eyre::eyre;
use image::Rgb;
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut};
use imageproc::rect::Rect;
use rusttype::Font;

mod article;
mod background;
mod layout;

use article::ArticleInformation;
use background::{add_circles, fill_background};
use layout::Layout;

static IMAGE_WIDTH: u32 = 1200;
static IMAGE_HEIGHT: u32 = 1200;
static TEXT_BOX_PADDING: u32 = 100;
static VERTICAL_PADDING: u32 = 285;
static TEXT_PADDING: u32 = 30;

static BACKGROUND_COLOR: [u8; 3] = [248, 240, 255];
static PRIMARY_COLOR: [u8; 3] = [112, 33, 186];
static PRIMARY_DESATURATED_COLOR: [u8; 3] = [130, 61, 194];
static SECONDARY_COLOR: [u8; 3] = [235, 214, 255];

fn main() -> Result<()> {
    color_eyre::install()?;
    let article_file: String = std::env::args()
        .skip(1)
        .next()
        .ok_or(eyre!("Expected a filepath to a blog article"))?;
    let article_information = ArticleInformation::retrieve(&article_file)?;
    generate_image(&article_information)?;
    println!(
        "Output {}",
        article_information.image_path.to_string_lossy()
    );

    Ok(())
}

fn generate_image(article_information: &ArticleInformation) -> Result<()> {
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    fill_background(&mut image);
    add_circles(&mut image);

    let font_data: &[u8] = include_bytes!("/usr/share/fonts/TTF/DejaVuSans.ttf");
    let font: Font<'static> =
        Font::try_from_bytes(font_data).ok_or(eyre!("Could not load font"))?;

    let max_width = IMAGE_WIDTH - (2 * TEXT_BOX_PADDING);
    let max_height = IMAGE_HEIGHT - (2 * VERTICAL_PADDING + 2 * TEXT_BOX_PADDING);

    let layout = Layout::new(
        &font,
        &article_information.title,
        max_width - (2 * TEXT_PADDING),
        max_height - (2 * TEXT_PADDING),
    )?;

    let width = layout.calculated_width();
    let height = layout.calculated_height();

    let x = IMAGE_WIDTH / 2 - width / 2;
    let y = IMAGE_HEIGHT / 2 - height / 2;

    let rect = Rect::at((x - TEXT_PADDING) as i32, (y - TEXT_PADDING) as i32)
        .of_size(width + (2 * TEXT_PADDING), height + (2 * TEXT_PADDING));
    draw_filled_rect_mut(&mut image, rect, Rgb(SECONDARY_COLOR));
    draw_hollow_rect_mut(&mut image, rect, Rgb(PRIMARY_COLOR));

    layout.render(&mut image, image::Rgb(PRIMARY_COLOR), x, y)?;

    image.save(&article_information.image_path)?;

    Ok(())
}
