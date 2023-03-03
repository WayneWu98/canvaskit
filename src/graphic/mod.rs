pub mod arc;
pub mod container;
pub mod line;
pub mod text;

use std::{f32::consts::PI, ops::Bound};

use serde::Deserialize;
use tiny_skia::{Paint, Path, Pixmap, Point, Rect, Shader};

use crate::{
    color,
    drawing::shader::create_linear_gradient,
    metrics::*,
    utils::{self, AppResult},
};

pub struct DrawResult(pub Pixmap, pub Rect);

pub trait Draw {
    fn draw(
        &mut self,
        pixmap: Pixmap,
        pos_bounds: Rect,
        layout_bounds: Rect,
    ) -> AppResult<DrawResult>;
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Default for Align {
    fn default() -> Self {
        Align::Left
    }
}

impl Align {
    fn x(&self, x: f32, w: f32, bounds: &Rect) -> f32 {
        match self {
            Align::Left => bounds.left() + x,
            Align::Center => bounds.left() + bounds.width() / 2. - w / 2.,
            Align::Right => bounds.right() - w - x,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Graphic {
    Text(text::Text),
    Container(container::Container),
    Line(line::Line),
}

impl Draw for Graphic {
    fn draw(
        &mut self,
        pixmap: Pixmap,
        pos_bounds: Rect,
        layout_bounds: Rect,
    ) -> AppResult<DrawResult> {
        match self {
            Graphic::Container(container) => container.draw(pixmap, pos_bounds, layout_bounds),
            Graphic::Line(line) => line.draw(pixmap, pos_bounds, layout_bounds),
            Graphic::Text(text) => text.draw(pixmap, pos_bounds, layout_bounds),
        }
    }
}
