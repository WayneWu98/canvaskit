use serde::Deserialize;
use tiny_skia::Rect;

use crate::{
    color,
    matrix::*,
    utils::{self, AppResult},
};

use super::Draw;

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub content: String,
    pub color: color::Rgba,
    pub size: f32,
    pub font: String,
    pub line_height: Option<f32>,
    pub position: Option<Position>,
    pub suffix: Option<String>,
    // pub align
    #[serde(skip)]
    pub layout_bounds: Option<Box<Rect>>,
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
        pixmap: &mut tiny_skia::Pixmap,
        bounds: tiny_skia::Rect,
        layout_bounds: Option<Box<tiny_skia::Rect>>,
    ) -> AppResult<tiny_skia::Rect> {
        self.layout_bounds = layout_bounds;
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
}
