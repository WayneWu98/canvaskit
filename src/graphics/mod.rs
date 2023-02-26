use std::f32::consts::PI;

use serde::{Deserialize, Serialize};
use tiny_skia::{Point, Shader};

use crate::{
    drawing::shader::create_linear_gradient,
    utils::{self, AppResult},
};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Position(pub f32, pub f32);

impl From<(f32, f32)> for Position {
    fn from(val: (f32, f32)) -> Self {
        Self(val.0, val.1)
    }
}

impl From<Position> for Point {
    fn from(value: Position) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Size(pub f32, pub f32);

impl From<(f32, f32)> for Size {
    fn from(val: (f32, f32)) -> Self {
        Self(val.0, val.1)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum GradientStopPosition {
    Percent(f32),
    Pixel(f32),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColorStop {
    pub position: GradientStopPosition,
    pub color: (u8, u8, u8, u8),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Color {
    Rgba(u8, u8, u8, u8),
    Gradient { angle: f32, stops: Vec<ColorStop> },
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shadow(pub f32, pub f32, pub f32, pub f32, pub Color);

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Corner(pub f32, pub f32, pub f32, pub f32);

impl Default for Corner {
    fn default() -> Self {
        Self(0., 0., 0., 0.)
    }
}

impl Corner {
    pub fn get_fitted(&self, Size(w, h): &Size) -> Self {
        let mc_x = w / 2.;
        let mc_y = h / 2.;
        Corner(
            utils::min(&[self.0, mc_x, mc_y]),
            utils::min(&[self.1, mc_x, mc_y]),
            utils::min(&[self.2, mc_x, mc_y]),
            utils::min(&[self.3, mc_x, mc_y]),
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Graphic {
    Text {
        position: Position,
        text: String,
        color: Color,
        size: Size,
        font_size: u32,
        max_rows: u8,
        suffix: String,
    },
    Rectangle {
        corner: Option<Corner>,
        color: Color,
        shadow: Option<Shadow>,
        position: Position,
        size: Size,
    },
}

impl Graphic {
    pub fn get_shader(&self) -> AppResult<Shader> {
        match self {
            Graphic::Rectangle {
                color,
                corner,
                position,
                size,
                ..
            } => match color {
                Color::Rgba(r, g, b, a) => Ok(Shader::SolidColor(tiny_skia::Color::from_rgba8(
                    *r, *g, *b, *a,
                ))),
                Color::Gradient { angle, stops } => Ok(create_linear_gradient(
                    *angle,
                    position.clone(),
                    size.clone(),
                    corner.clone().or(Some(Corner::default())).unwrap(),
                    stops.clone(),
                )?),
            },
            _ => Ok(Shader::SolidColor(tiny_skia::Color::from_rgba8(
                255, 255, 255, 0,
            ))),
        }
    }
}
