pub mod arc;
pub mod line;
pub mod rectangle;
pub mod text;

use std::f32::consts::PI;

use serde::Deserialize;
use tiny_skia::{Paint, Path, Pixmap, Point, Shader};

use crate::{
    color,
    drawing::shader::create_linear_gradient,
    matrix::*,
    utils::{self, AppResult},
};

pub trait Draw {
    fn draw(&self, pixmap: &mut Pixmap) -> AppResult;
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Graphic {
    // Text(text::Text),
    Rectangle(rectangle::Rectangle),
    Line(line::Line),
}

impl Draw for Graphic {
    fn draw(&self, pixmap: &mut Pixmap) -> AppResult {
        match self {
            Graphic::Rectangle(rect) => rect.draw(pixmap),
            Graphic::Line(line) => line.draw(pixmap),
            _ => Ok(()),
        }
    }
}
