use serde::Deserialize;
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Shader, Transform};

use crate::{
    color,
    drawing::shader::{self, create_linear_gradient},
    effects,
    metrics::*,
    utils::{self, make_error, AppResult},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Arc {
    pub radius: f32,
    pub color: color::Rgba,
    // pub shadow: Option<effects::BoxShadow>,
    pub position: Position,
    pub width: f32,
}

impl Default for Arc {
    fn default() -> Self {
        Self {
            radius: 0.,
            color: color::Rgba(0, 0, 0, 255),
            position: Position::default(),
            width: 1.,
        }
    }
}

impl Arc {
    pub fn draw(&self) -> AppResult {
        todo!()
    }
}
