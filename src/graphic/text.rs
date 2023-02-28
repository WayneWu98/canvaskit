use serde::Deserialize;

use crate::{
    color,
    matrix::*,
    utils::{self, AppResult},
};

use super::Draw;

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub content: String,
    pub color: color::Rgba,
    pub size: f32,
    pub font: String,
}

// impl Draw for Text {
//     fn draw(&self, pixmap: &mut tiny_skia::Pixmap) -> AppResult {
//         let w = pixmap.width();
//         let h = pixmap.height();
//         let graphic = utils::create_empty_pixmap(w, h)?;

//         todo!()
//     }
// }
