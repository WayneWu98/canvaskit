use rusttype::{point, Font, Scale};
use serde::Deserialize;
use tiny_skia::{Pixmap, PremultipliedColorU8, Rect};
use wasm_bindgen_test::console_log;

macro_rules! letter_width {
    ($rect: expr) => {
        $rect.max.x - $rect.min.x
    };
}

macro_rules! letters_width {
    ($letters: expr) => {
        $letters.iter().fold(0, |a, b| a + b.max.x - b.min.x)
    };
}

use crate::{
    color, effects, empty_rect,
    metrics::*,
    utils::{self, AppResult},
    xywh_rect,
};

use super::{Align, Draw, DrawResult};

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub content: String,
    pub color: color::Rgba,
    pub size: f32,
    pub font: Option<String>,
    pub line_height: Option<f32>,
    pub position: Option<Position>,
    pub suffix: Option<String>,
    pub max_rows: Option<usize>,
    pub align: Option<Align>,
    pub shadow: Option<effects::DropShadow>,
    #[serde(skip)]
    pub pos_bounds: Option<Rect>,
    #[serde(skip)]
    pub layout_bounds: Option<Rect>,
}

impl Draw for Text {
    fn draw(
        &mut self,
        mut pixmap: Pixmap,
        pos_bounds: Rect,
        layout_bounds: Rect,
    ) -> AppResult<DrawResult> {
        self.pos_bounds = Some(pos_bounds);
        self.layout_bounds = Some(layout_bounds);
        if self.content.chars().count() <= 0 {
            return Ok(DrawResult(pixmap, pos_bounds));
        }
        let w = pixmap.width() as i32;
        let h = pixmap.height() as i32;
        let data = pixmap.pixels_mut();
        let allocated = self.allocated();
        let mut offset_y = self.y() as i32;
        let color::Rgba(r, g, b, a) = self.color;
        let mut min_x = self.row_start(0.) as i32;
        let min_y = offset_y;
        let mut max_w = 0;
        let max_h = allocated.len() as f32 * self.line_height() as f32;
        for letters in allocated {
            let row_w = letters_width!(letters
                .iter()
                .map(|letter| &letter.bounds)
                .collect::<Vec<_>>());
            max_w = max_w.max(row_w);
            let mut offset_x = self.row_start(row_w as f32) as i32;
            min_x = min_x.min(offset_x);
            for letter in letters {
                let LetterGlyph { bounds, pixels } = letter;
                for LetterPixel(x, y, v) in pixels {
                    let x = offset_x + x as i32;
                    let y = offset_y;
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let offset = ((y - 1) * w + x) as usize;

                    if data.get(offset).is_some() {
                        data[offset] = tiny_skia::ColorU8::from_rgba(r, g, b, (a as f32 * v) as u8)
                            .premultiply();
                    }
                    offset_x += letter_width!(bounds);
                }
            }
            offset_y += self.line_height() as i32;
        }
        Ok(DrawResult(
            pixmap,
            xywh_rect!(min_x as f32, min_y as f32, max_w as f32, max_h as f32),
        ))
    }
}

impl Text {
    pub fn line_height(&self) -> f32 {
        self.line_height.unwrap_or(self.size)
    }
    pub fn font(&self) -> Option<&'static Font<'static>> {
        let font: String = if let Some(ref f) = self.font {
            f.clone()
        } else {
            "".into()
        };
        crate::font::get_font(font)
    }

    pub fn allocated(&self) -> Vec<Vec<LetterGlyph>> {
        let (c_metrics, e_metrics) = self.metrics();
        let mut allocated: Vec<Vec<LetterGlyph>> = vec![];
        let max_rows = self.max_rows();
        let max_width = self.max_width() as i32;
        let mut w = 0_i32;
        for letter in c_metrics {
            if allocated.last().is_none() {
                allocated.push(vec![]);
            }
            let rows = allocated.len();
            let row = allocated.last_mut().unwrap();
            let letter_w = letter.bounds.max.x - letter.bounds.min.x;
            w += letter_w;
            if w <= max_width {
                row.push(letter);
                continue;
            }
            if rows >= max_rows {
                let suffix_w = e_metrics
                    .as_ref()
                    .and_then(|m| Some(letter_width!(&m.bounds)))
                    .unwrap_or(0_i32);
                while w + suffix_w > max_width {
                    if row.len() <= 1 {
                        break;
                    }
                    w -= letter_width!(row.pop().unwrap().bounds);
                }
                if let Some(e_metrics) = e_metrics {
                    row.push(e_metrics);
                }
                break;
            }
            w = letter_width!(letter.bounds);
            allocated.push(vec![letter]);
        }
        allocated
    }

    pub fn metrics<'a>(&self) -> (Vec<LetterGlyph>, Option<LetterGlyph>) {
        let font = self.font().unwrap();
        let scale = Scale::uniform(self.size);
        let v_metrics = font.v_metrics(scale);
        let h = (v_metrics.descent - v_metrics.ascent).ceil();
        let y = v_metrics.ascent + (self.line_height() - h) / 2.;
        let start = point(0., y);
        let glyphs: Vec<_> = font
            .layout(&self.content, scale, start)
            .filter(|glyph| glyph.pixel_bounding_box().is_some())
            .map(|glyph| {
                let bounds = glyph.pixel_bounding_box().unwrap();
                let w = bounds.width() as usize;
                let h = bounds.height() as usize;
                let mut pixels = Vec::with_capacity(w * h / 2);
                glyph.draw(|x, y, v| pixels.push(LetterPixel(x, y, v)));
                LetterGlyph { bounds, pixels }
            })
            .collect();
        let mut suffix = None;
        if self.suffix.is_some() {
            let suffix_glyphs: Vec<_> = font.layout(self.suffix(), scale, start).collect();
            let bounds = rusttype::Rect {
                min: suffix_glyphs
                    .first()
                    .unwrap()
                    .pixel_bounding_box()
                    .unwrap()
                    .min,
                max: suffix_glyphs
                    .last()
                    .unwrap()
                    .pixel_bounding_box()
                    .unwrap()
                    .max,
            };
            let mut pixels = vec![];
            for (i, glyph) in suffix_glyphs.iter().enumerate() {
                let left = suffix_glyphs
                    .get(i - 1)
                    .and_then(|m| m.pixel_bounding_box())
                    .and_then(|b| Some(b.max.x))
                    .unwrap_or(0) as u32;
                let offset = glyph.pixel_bounding_box().unwrap().min.x as u32;
                glyph.draw(|x, y, a| pixels.push(LetterPixel(x + offset - left, y, a)))
            }
            suffix = Some(LetterGlyph { bounds, pixels })
        }
        (glyphs, suffix)
    }

    pub fn max_rows(&self) -> usize {
        self.max_rows.unwrap_or(999)
    }
    // x is just an anchor position, not start position of first letter
    pub fn x(&self) -> f32 {
        if let Some(ref position) = self.position {
            if let Some(ref x) = position.x {
                return self
                    .align
                    .or(Some(Align::Left))
                    .unwrap()
                    .x(*x, 0., &self.layout_bounds());
            }
        }
        if let Some(ref align) = self.align {
            return align.x(0., 0., &self.layout_bounds());
        }
        self.pos_bounds().left()
    }
    pub fn row_start(&self, w: f32) -> f32 {
        let x = self.x();
        match self.align.unwrap_or_default() {
            Align::Left => x,
            Align::Center => x - w / 2.,
            Align::Right => x - w,
        }
    }
    pub fn y(&self) -> f32 {
        if let Some(ref position) = self.position {
            if let Some(ref y) = position.y {
                return *y + self.layout_bounds().top();
            }
        }
        self.pos_bounds().top()
    }
    fn suffix(&self) -> &str {
        if let Some(ref s) = self.suffix {
            s
        } else {
            ""
        }
    }
    fn layout_bounds(&self) -> Rect {
        self.layout_bounds.unwrap_or(empty_rect!())
    }
    fn pos_bounds(&self) -> Rect {
        self.pos_bounds.unwrap_or(empty_rect!())
    }
    pub fn max_width(&self) -> f32 {
        self.layout_bounds().width() - self.x()
    }
    pub fn max_height(&self) -> f32 {
        self.max_rows() as f32 * self.line_height()
    }
}

pub struct LetterGlyph {
    pub bounds: rusttype::Rect<i32>,
    pub pixels: Vec<LetterPixel>,
}

pub struct LetterPixel(pub u32, pub u32, pub f32);
