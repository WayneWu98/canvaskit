use std::fmt::Display;

use crate::color;
use tiny_skia::{Paint, Shader};
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
