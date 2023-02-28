pub mod arc;
pub mod line;
pub mod rectangle;
pub mod text;

use std::f32::consts::PI;

use serde::Deserialize;
use tiny_skia::{Paint, Path, Pixmap, Point, Rect, Shader};

use crate::{
    color,
    drawing::shader::create_linear_gradient,
    matrix::*,
    utils::{self, AppResult},
};

pub trait Draw {
    fn draw(
        &mut self,
        pixmap: &mut Pixmap,
        bounds: Rect,
        layout_bounds: Option<Box<Rect>>,
    ) -> AppResult<Rect>;
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Graphic {
    // Text(text::Text),
    Rectangle(rectangle::Rectangle),
    Line(line::Line),
}

impl Draw for Graphic {
    fn draw(
        &mut self,
        pixmap: &mut Pixmap,
        bounds: Rect,
        layout_bounds: Option<Box<Rect>>,
    ) -> AppResult<Rect> {
        match self {
            Graphic::Rectangle(rect) => rect.draw(pixmap, bounds, layout_bounds),
            Graphic::Line(line) => line.draw(pixmap, bounds, layout_bounds),
            _ => Ok(Rect::from_xywh(0., 0., 0., 0.).unwrap()),
        }
    }
}
