use serde::Deserialize;
use tiny_skia::{Point, Rect};

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl Position {
    pub fn x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }
    pub fn y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = Some(x)
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = Some(y)
    }
    pub fn add_size(&mut self, size: &Size) {
        self.set_x(self.x() + size.width());
        self.set_y(self.y() + size.height());
    }
}

impl From<(f32, f32)> for Position {
    fn from((x, y): (f32, f32)) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
        }
    }
}

impl From<Rect> for Position {
    fn from(value: Rect) -> Self {
        Self {
            x: Some(value.left()),
            y: Some(value.top()),
        }
    }
}

impl From<&Rect> for Position {
    fn from(value: &Rect) -> Self {
        Self {
            x: Some(value.left()),
            y: Some(value.top()),
        }
    }
}

impl From<Position> for Point {
    fn from(value: Position) -> Self {
        Point {
            x: value.x.or(Some(0.)).unwrap(),
            y: value.y.or(Some(0.)).unwrap(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: Some(0.),
            y: Some(0.),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Size {
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl Size {
    pub fn width(&self) -> f32 {
        self.width.unwrap_or(0.)
    }
    pub fn height(&self) -> f32 {
        self.height.unwrap_or(0.)
    }
    pub fn set_width(&mut self, w: f32) {
        self.width = Some(self.width() + w)
    }
    pub fn set_height(&mut self, h: f32) {
        self.height = Some(self.height() + h)
    }
}

impl From<(f32, f32)> for Size {
    fn from((w, h): (f32, f32)) -> Self {
        Self {
            width: Some(w),
            height: Some(h),
        }
    }
}

impl From<Rect> for Size {
    fn from(value: Rect) -> Self {
        Self {
            width: Some(value.width()),
            height: Some(value.height()),
        }
    }
}
impl From<&Rect> for Size {
    fn from(value: &Rect) -> Self {
        Self {
            width: Some(value.width()),
            height: Some(value.height()),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: Some(0.),
            height: Some(0.),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Padding(f32, f32, f32, f32);

impl Default for Padding {
    fn default() -> Self {
        Self(0., 0., 0., 0.)
    }
}

impl Padding {
    pub fn top(&self) -> f32 {
        self.0
    }
    pub fn right(&self) -> f32 {
        self.1
    }

    pub fn bottom(&self) -> f32 {
        self.2
    }
    pub fn left(&self) -> f32 {
        self.3
    }
}
