use serde::Deserialize;
use tiny_skia::Point;

#[derive(Deserialize, Debug, Clone, Copy)]
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Size(pub f32, pub f32);

impl From<(f32, f32)> for Size {
    fn from(val: (f32, f32)) -> Self {
        Self(val.0, val.1)
    }
}
