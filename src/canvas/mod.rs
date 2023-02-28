pub mod color;
pub mod effects;
pub mod font;
pub mod matrix;

use std::collections::HashMap;

use crate::graphic::Draw;
use serde::Deserialize;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Transform};
use wasm_bindgen::JsValue;

use crate::{
    graphic::{
        self,
        rectangle::{self, Rectangle},
        Graphic,
    },
    matrix::*,
    utils::{self, error_mapper, make_error, AppError, AppResult},
};

use self::font::init_fonts;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanvasConfiguration {
    pub size: Size,
    pub background: color::Color,
    pub font_set: Option<HashMap<String, Vec<u8>>>,
    pub graphics: Vec<Graphic>,
}

impl TryFrom<String> for CanvasConfiguration {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(&value).map_err(|_| make_error("invalid configuration!!"))
    }
}

impl CanvasConfiguration {
    pub fn build(self) -> AppResult<Canvas> {
        if let Some(font_set) = self.font_set {
            init_fonts(font_set)?;
        }
        let mut canvas = Canvas::new(self.size)?;
        let mut bounds = Rect::from_xywh(0., 0., self.size.width(), self.size.height())
            .map_or(Err(make_error("create bounds fail!!")), |v| Ok(v))?;
        canvas.draw(
            Graphic::Rectangle(Rectangle {
                color: self.background,
                size: Some(self.size),
                ..Rectangle::default()
            }),
            bounds.clone(),
            None,
        )?;
        for graphic in self.graphics {
            bounds = canvas.draw(graphic, bounds, None)?;
        }
        Ok(canvas)
    }
}

pub struct Canvas {
    pub pixmap: tiny_skia::Pixmap,
}

impl Canvas {
    pub fn new(size: Size) -> AppResult<Self> {
        Ok(Self {
            pixmap: Pixmap::new(size.width() as u32, size.height() as u32)
                .map_or(Err(make_error("init pixmap fail!")), |v| Ok(v))?,
        })
    }

    pub fn draw(
        &mut self,
        mut graphic: Graphic,
        bounds: Rect,
        layout_bounds: Option<Box<Rect>>,
    ) -> AppResult<Rect> {
        graphic.draw(&mut self.pixmap, bounds, layout_bounds)
    }

    pub fn export(&self) -> AppResult<Vec<u8>> {
        Ok(self
            .pixmap
            .encode_png()
            .map_err(|_| make_error("export fail!"))?)
    }
}
