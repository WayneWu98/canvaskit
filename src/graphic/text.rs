use rusttype::{point, Font, Scale, VMetrics};
use serde::Deserialize;
use tiny_skia::{
    BlendMode, FilterQuality, Pixmap, PixmapPaint, PremultipliedColorU8, Rect, Transform,
};
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
    color, effects, empty_pixmap, empty_rect,
    font::get_font,
    merge_pixmap,
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
        let w = pixmap.width();
        let h = pixmap.height();
        let mut text_pixmap = empty_pixmap!(w, h);
        let rows = self.allocated();
        let data = text_pixmap.pixels_mut();
        let color::Rgba(r, g, b, a) = self.color;
        let v_metrics = self.v_metrics();
        let font_h = (v_metrics.ascent - v_metrics.descent).ceil() as f32;
        let offset_y = self.y() + (self.line_height() - font_h) / 2.;
        let mut offset_y = offset_y as u32;
        let mut text_x = pos_bounds.left();
        let text_y = offset_y as f32;
        let mut text_w: f32 = 0.;
        let text_h = rows.len() as f32 * self.line_height();
        for letters in rows {
            let row_w = letters_width!(letters
                .iter()
                .map(|letter| letter.bounds)
                .collect::<Vec<_>>());
            text_w = text_w.max(row_w as f32);
            let mut offset_x = self.row_start(w as f32) as u32;
            text_x = text_x.min(offset_x as f32);
            for LetterGlyph { bounds, pixels } in letters {
                for LetterPixel(mut x, mut y, v) in pixels {
                    x += offset_x;
                    y += bounds.min.y as u32 + offset_y;
                    let idx = w.checked_mul(y).and_then(|v| v.checked_add(x));
                    if let Some(idx) = idx {
                        let idx = idx as usize;
                        if data.get(idx).is_some() {
                            data[idx] = tiny_skia::ColorU8::from_rgba(
                                r,
                                g,
                                b,
                                (a as f32 * v).floor() as u8,
                            )
                            .premultiply();
                        }
                    }
                }
                offset_x += letter_width!(bounds) as u32;
            }
            offset_y += self.line_height() as u32;
        }

        Ok(DrawResult(
            merge_pixmap!(pixmap, text_pixmap),
            xywh_rect!(text_x, text_y, text_w, text_h),
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
        let (c_metrics, s_metrics) = self.glyphs();
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
                let suffix_w = s_metrics
                    .as_ref()
                    .and_then(|v| Some(letter_width!(&v.bounds)))
                    .unwrap_or(0_i32);
                while w + suffix_w > max_width {
                    if row.len() <= 1 {
                        break;
                    }
                    w -= letter_width!(row.pop().unwrap().bounds);
                }
                if let Some(s_metrics) = s_metrics {
                    row.push(s_metrics);
                }
                break;
            }
            w = letter_width!(&letter.bounds);
            allocated.push(vec![letter]);
        }
        allocated
    }

    pub fn glyphs<'a>(&self) -> (Vec<LetterGlyph>, Option<LetterGlyph>) {
        let font = self.font().unwrap();
        let scale = Scale::uniform(self.size);
        let v_metrics = self.v_metrics();
        let start = point(0., v_metrics.ascent);
        let glyphs: Vec<_> = font
            .layout(&self.content, scale, start)
            .filter(|glyph| glyph.pixel_bounding_box().is_some())
            .map(|glyph| {
                let bounds = glyph.pixel_bounding_box().unwrap();
                let mut pixels = vec![];
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
                let offset = suffix_glyphs
                    .get(i - 1)
                    .and_then(|m| m.pixel_bounding_box())
                    .and_then(|b| Some(b.max.x))
                    .unwrap_or(0) as u32;
                glyph.draw(|x, y, v| pixels.push(LetterPixel(x - offset, y, v)))
            }
            suffix = Some(LetterGlyph { bounds, pixels })
        }
        (glyphs, suffix)
    }

    fn v_metrics(&self) -> VMetrics {
        let font = self.font().unwrap();
        let scale = Scale::uniform(self.size);
        font.v_metrics(scale)
    }

    fn max_rows(&self) -> usize {
        self.max_rows.unwrap_or(999)
    }
    // x is just an anchor position, not start position of first letter
    fn x(&self) -> f32 {
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
    fn row_start(&self, w: f32) -> f32 {
        let x = self.x();
        if self.align.is_none() {
            return x;
        }
        match self.align.unwrap() {
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
    fn max_width(&self) -> f32 {
        if self.align.is_none() {
            self.layout_bounds().right() - self.pos_bounds().left()
        } else {
            self.layout_bounds().width()
        }
    }
}

pub struct LetterGlyph {
    pub bounds: rusttype::Rect<i32>,
    pub pixels: Vec<LetterPixel>,
}

pub struct LetterPixel(pub u32, pub u32, pub f32);
