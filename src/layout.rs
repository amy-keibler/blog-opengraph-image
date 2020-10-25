use color_eyre::eyre::Result;
use eyre::eyre;
use imageproc::drawing::Canvas;
use imageproc::pixelops::weighted_sum;
use rusttype::{point, Font, PositionedGlyph, Scale};

use std::collections::VecDeque;
use std::iter::FromIterator;

const MAX_FONT_SIZE: u32 = 120;
const MIN_FONT_SIZE: u32 = 20;
const MIN_DISTANCE_TO_CENTER_TEXT: u32 = 70;

enum HorizontalLayoutCalculator {
    LeftAligned,
    CenterAligned(Box<dyn Fn(u32) -> u32>),
}

impl HorizontalLayoutCalculator {
    fn left_padding(&self, line_width: u32) -> u32 {
        match self {
            HorizontalLayoutCalculator::LeftAligned => 0,
            HorizontalLayoutCalculator::CenterAligned(f) => f(line_width),
        }
    }
}

pub struct Layout<'a> {
    lines: Vec<Line<'a>>,
    line_height: u32,
}

impl<'a> Layout<'a> {
    pub fn new(font: &'a Font, text: &str, width: u32, height: u32) -> Result<Self> {
        let words: Vec<&str> = text.split_ascii_whitespace().collect();
        for font_size in (MIN_FONT_SIZE..=MAX_FONT_SIZE).rev() {
            if let Ok((lines, line_height)) =
                Layout::layout_at_font_size(font, &words, width, height, font_size)
            {
                return Ok(Self { lines, line_height });
            } else {
                println!("Couldn't lay out the text at {}", font_size);
            }
        }
        Err(eyre!(
            "Could not find a font size that fit the dimensions given {}x{}",
            width,
            height
        ))
    }

    fn layout_at_font_size(
        font: &'a Font,
        words: &Vec<&str>,
        width: u32,
        height: u32,
        font_size: u32,
    ) -> Result<(Vec<Line<'a>>, u32)> {
        println!("Trying font size {}", font_size);
        let scale = Scale::uniform(font_size as f32);
        let v_metrics = font.v_metrics(scale);
        let glyph_height = v_metrics.ascent - v_metrics.descent;
        let space_glyph = font.glyph(' ').scaled(scale);
        let space_width = space_glyph.h_metrics().advance_width as i32;

        let width = width as i32;
        let height = height as i32;
        let mut lines = Vec::<Line>::new();
        let mut remaining_width: i32 = width;
        let mut remaining_height: i32 = height - (glyph_height as i32);
        let mut current_line = Vec::<PositionedGlyph>::new();

        let mut words = VecDeque::from_iter(words.iter().copied());

        while let Some(word) = words.pop_front() {
            let mut layout: Vec<PositionedGlyph> = font
                .layout(
                    word,
                    scale,
                    point(
                        (width - remaining_width) as f32,
                        (lines.len() + 1) as f32 * glyph_height + v_metrics.descent,
                    ),
                )
                .collect();
            let word_width = word_width(&layout)?;
            if word_width < remaining_width {
                remaining_width -= word_width;
                current_line.append(&mut layout);
                // remove width of space from remaining width
                remaining_width -= space_width;
            } else {
                words.push_front(word);
                // new line
                lines.push(Line {
                    glyphs: current_line,
                });
                current_line = Vec::new();
                remaining_width = width;
                remaining_height -= glyph_height as i32;
                if remaining_height <= 0 {
                    return Err(eyre!(
                        "Could not fit another line inside the rectangle for {}",
                        font_size
                    ));
                }
            }
        }
        lines.push(Line {
            glyphs: current_line,
        });
        Ok((lines, glyph_height as u32))
    }

    pub fn calculated_width(&self) -> u32 {
        self.lines
            .iter()
            .map(Line::calculated_width)
            .max()
            .unwrap_or_default()
    }

    fn calculate_padding(&self) -> HorizontalLayoutCalculator {
        let max_width = self.calculated_width();
        if self
            .lines
            .iter()
            .map(Line::calculated_width)
            .any(|x| max_width - x > MIN_DISTANCE_TO_CENTER_TEXT)
        {
            HorizontalLayoutCalculator::CenterAligned(Box::new(move |x| {
                (max_width.clone() - x) / 2
            }))
        } else {
            HorizontalLayoutCalculator::LeftAligned
        }
    }

    pub fn calculated_height(&self) -> u32 {
        self.lines.len() as u32 * self.line_height
    }

    pub fn render<'b>(
        &self,
        canvas: &'b mut image::RgbaImage,
        color: image::Rgba<u8>,
        x: u32,
        y: u32,
    ) -> Result<()> {
        let padding_calculator = self.calculate_padding();
        for line in &self.lines {
            line.render(canvas, color, x, y, &padding_calculator)?;
        }
        Ok(())
    }
}

struct Line<'a> {
    glyphs: Vec<PositionedGlyph<'a>>,
}

impl<'a> Line<'a> {
    fn render<'b>(
        &self,
        canvas: &'b mut image::RgbaImage,
        color: image::Rgba<u8>,
        x: u32,
        y: u32,
        padding_calculator: &HorizontalLayoutCalculator,
    ) -> Result<()> {
        let width = self.calculated_width();
        let centering_padding: i32 = padding_calculator.left_padding(width) as i32;
        // code adapted from https://github.com/image-rs/imageproc/blob/7569542ba9bf7a5523850bd7cf34c79df29ba285/src/drawing/text.rs#L12 (used under MIT license)
        for g in &self.glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|gx, gy, gv| {
                    let gx = gx as i32 + bb.min.x;
                    let gy = gy as i32 + bb.min.y;

                    let image_x = gx + x as i32 + centering_padding;
                    let image_y = gy + y as i32;

                    let image_width = canvas.width() as i32;
                    let image_height = canvas.height() as i32;

                    if image_x >= 0
                        && image_x < image_width
                        && image_y >= 0
                        && image_y < image_height
                    {
                        let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                        let weighted_color = weighted_sum(*pixel, color, 1.0 - gv, gv);
                        canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                    }
                });
            }
        }
        Ok(())
    }

    fn calculated_width(&self) -> u32 {
        let initial_padding = self
            .glyphs
            .first()
            .map(|g| g.unpositioned().h_metrics().left_side_bearing)
            .unwrap_or_default();
        (initial_padding as i32 + word_width(&self.glyphs).unwrap_or_default()) as u32
    }
}

fn word_width(word: &Vec<PositionedGlyph>) -> Result<i32> {
    let first = word
        .first()
        .ok_or_else(|| eyre!("Word doesn't have a first letter"))?;
    let last = word
        .iter()
        .filter(|g| g.unpositioned().exact_bounding_box().is_some())
        .last()
        .ok_or_else(|| eyre!("Word doesn't have a last letter"))?;
    let bounding_box = last
        .unpositioned()
        .exact_bounding_box()
        .ok_or_else(|| eyre!("Could not create a bounding box for the last letter"))?;

    Ok((bounding_box.width() + (last.position().x - first.position().x)) as i32)
}
