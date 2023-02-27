pub mod rectangle;
pub mod text;

use std::f32::consts::PI;

use serde::{Deserialize, Serialize};
use tiny_skia::{Paint, Path, Point, Shader};

use crate::{
    color,
    drawing::shader::create_linear_gradient,
    matrix::*,
    utils::{self, AppResult},
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Graphic {
    Text(text::Text),
    Rectangle(rectangle::Rectangle),
}
