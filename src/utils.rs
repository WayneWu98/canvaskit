use std::fmt::Display;

use crate::{
    color,
    metrics::{Position, Size},
};
use tiny_skia::{Paint, Pixmap, Rect, Shader, Transform};
use wasm_bindgen::JsValue;

pub type AppResult<T = ()> = Result<T, AppError>;

pub trait Union {
    fn union(&self, rect: &Rect) -> Rect;
}

impl Union for Rect {
    fn union(&self, rect: &Rect) -> Rect {
        let l = min(&[self.left(), rect.left()]);
        let t = min(&[self.top(), rect.top()]);
        let r = max(&[self.right(), rect.right()]);
        let b = max(&[self.bottom(), rect.bottom()]);
        crate::ltrb_rect!(l, t, r, b)
    }
}

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

pub fn max<T: std::cmp::PartialOrd + From<u8> + Copy>(nums: &[T]) -> T {
    let mut v: T = *nums.get(0).unwrap();
    for num in nums {
        if num.gt(&v) {
            v = *num;
        }
    }
    v
}

pub fn merge_pixmap(a: &mut Pixmap, b: &Pixmap, offset: Option<Position>) {
    let position = offset.or(Some(Position::default())).unwrap();
    let x = position.x();
    let y = position.y();
    a.draw_pixmap(
        x as i32,
        y as i32,
        b.as_ref(),
        &tiny_skia::PixmapPaint::default(),
        Transform::identity(),
        None,
    );
}
