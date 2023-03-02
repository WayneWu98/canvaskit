pub mod color;
pub mod effects;
pub mod font;
pub mod matrix;

use std::collections::HashMap;

use crate::{
    clip_pixmap, empty_pixmap, empty_rect,
    graphic::{Draw, DrawResult},
};
use serde::Deserialize;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Transform};
use wasm_bindgen::JsValue;

use crate::{
    graphic::{self, container::Container, Graphic},
    matrix::*,
    utils::{self, error_mapper, make_error, AppError, AppResult},
};

use self::font::init_fonts;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanvasConfiguration {
    pub font_set: Option<HashMap<String, Vec<u8>>>,
    pub graphic: graphic::container::Container,
}

impl TryFrom<String> for CanvasConfiguration {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(&value).map_err(|err| make_error(&err.to_string()))
    }
}

impl CanvasConfiguration {
    pub fn build(self) -> AppResult<Canvas> {
        if let Some(font_set) = self.font_set {
            init_fonts(font_set)?;
        }
        let canvas = Canvas::new().draw(Graphic::Container(self.graphic))?;
        Ok(canvas)
    }
}

pub struct Canvas {
    pub pixmap: tiny_skia::Pixmap,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            pixmap: empty_pixmap!(1, 1),
        }
    }

    pub fn draw(self, mut graphic: Graphic) -> AppResult<Self> {
        let pb = empty_rect!();
        let lb = empty_rect!();
        let DrawResult(pixmap, bounds) = graphic.draw(self.pixmap, pb, lb)?;
        Ok(Self {
            pixmap: clip_pixmap!(pixmap, bounds),
        })
    }

    pub fn export(&self) -> AppResult<Vec<u8>> {
        Ok(self
            .pixmap
            .encode_png()
            .map_err(|_| make_error("export fail!"))?)
    }
}
