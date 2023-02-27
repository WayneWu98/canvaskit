use std::fmt::Display;

use crate::{
    color,
    matrix::{Position, Size},
};
use tiny_skia::{Paint, Pixmap, Shader, Transform};
use wasm_bindgen::JsValue;

pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<AppError> for JsValue {
    fn from(value: AppError) -> Self {
        JsValue::from_str(&value.message)
    }
}

impl std::error::Error for AppError {}

pub fn make_error(msg: &str) -> AppError {
    AppError {
        message: msg.into(),
    }
}

pub fn error_mapper(e: impl std::error::Error) -> AppError {
    make_error(&e.to_string())
}

pub fn min<T: std::cmp::PartialOrd + From<u8> + Copy>(nums: &[T]) -> T {
    let mut v: T = *nums.get(0).unwrap();
    for num in nums {
        if num.lt(&v) {
            v = *num;
        }
    }
    v
}

pub fn create_empty_pixmap(w: u32, h: u32) -> AppResult<Pixmap> {
    Pixmap::new(w, h).map_or(Err(make_error("create pixmap fail!")), |v| Ok(v))
}

pub fn merge_pixmap(a: &mut Pixmap, b: &Pixmap, offset: Option<Position>) {
    let Position(x, y) = offset.or(Some(Position(0., 0.))).unwrap();
    a.draw_pixmap(
        x as i32,
        y as i32,
        b.as_ref(),
        &tiny_skia::PixmapPaint::default(),
        Transform::identity(),
        None,
    );
}

pub fn create_paint() -> Paint<'static> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint
}

pub fn create_rgba_paint(color: color::Rgba) -> Paint<'static> {
    let mut paint = create_paint();
    paint.shader = Shader::SolidColor(color.into());
    paint
}
