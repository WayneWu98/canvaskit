pub mod color;
pub mod effects;
pub mod matrix;

use crate::graphic::Draw;
use serde::Deserialize;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
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

#[derive(Deserialize, Debug)]
pub struct CanvasConfiguration {
    pub size: Size,
    pub background: color::Color,
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
        let mut canvas = Canvas::new(self.size)?;
        canvas.draw(Graphic::Rectangle(Rectangle {
            color: self.background,
            size: self.size,
            ..Rectangle::default()
        }))?;
        for graphic in self.graphics {
            canvas.draw(graphic)?;
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
            pixmap: Pixmap::new(size.0 as u32, size.1 as u32)
                .map_or(Err(make_error("init pixmap fail!")), |v| Ok(v))?,
        })
    }

    pub fn draw(&mut self, graphic: Graphic) -> AppResult {
        graphic.draw(&mut self.pixmap)
    }

    pub fn export(&self) -> AppResult<Vec<u8>> {
        Ok(self
            .pixmap
            .encode_png()
            .map_err(|_| make_error("export fail!"))?)
    }
}
