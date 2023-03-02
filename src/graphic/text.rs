use rusttype::Font;
use serde::Deserialize;
use tiny_skia::{Pixmap, Rect};

use crate::{
    color,
    metrics::*,
    utils::{self, AppResult},
};

use super::{Draw, DrawResult};

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub content: String,
    pub color: color::Rgba,
    pub size: f32,
    pub font: Option<String>,
    pub line_height: Option<f32>,
    pub position: Option<Position>,
    pub suffix: Option<String>,
    #[serde(default)]
    pub auto_wrap: bool,
    #[serde(default)]
    pub align: TextAlign,
    #[serde(skip)]
    pub pos_bounds: Option<Rect>,
    #[serde(skip)]
    pub layout_bounds: Option<Rect>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

impl Draw for Text {
    // fn draw(&mut self, pixmap: &mut Pixmap) -> AppResult {
    //     let w = pixmap.width();
    //     let h = pixmap.height();
    //     let graphic = utils::create_empty_pixmap(w, h)?;

    //     todo!()
    // }

    fn draw(
        &mut self,
        pixmap: Pixmap,
        pos_bounds: Rect,
        layout_bounds: Rect,
    ) -> AppResult<DrawResult> {
        self.pos_bounds = Some(pos_bounds);
        self.layout_bounds = Some(layout_bounds);
        // if let Some(font) = get_font
        todo!()
    }
}

impl Text {
    pub fn measure(content: String, font: String) -> AppResult<f32> {
        todo!()
    }
    pub fn line_height(&self) -> f32 {
        self.line_height.unwrap_or(self.size)
    }
    pub fn font(&self) -> Option<&'static Font> {
        let font: &str = if let Some(ref f) = self.font { f } else { "" };
        crate::font::get_font(font)
    }
}
