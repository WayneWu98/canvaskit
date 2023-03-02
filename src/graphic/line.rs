use serde::Deserialize;
use tiny_skia::{
    BlendMode, FillRule, FilterQuality, Paint, Path, PathBuilder, Pixmap, PixmapPaint, Rect,
    Shader, Stroke, Transform,
};
use wasm_bindgen_test::console_log;

use crate::{
    color,
    drawing::shader::{self, create_linear_gradient},
    effects, empty_pixmap, empty_rect, merge_pixmap,
    metrics::*,
    rgba_paint,
    utils::{self, make_error, AppResult},
};

use super::{Draw, DrawResult};

#[derive(Deserialize, Debug, Clone)]
pub struct Line {
    pub from: Option<Position>,
    pub to: Option<Position>,
    pub width: f32,
    pub color: color::Rgba,
    pub shadow: Option<effects::DropShadow>,
    #[serde(skip)]
    pub pos_bounds: Option<Rect>,
    #[serde(skip)]
    pub layout_bounds: Option<Rect>,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            from: None,
            to: None,
            width: 1.,
            color: color::Rgba(0, 0, 0, 255),
            shadow: None,
            layout_bounds: None,
            pos_bounds: None,
        }
    }
}

impl Draw for Line {
    fn draw(
        &mut self,
        mut pixmap: Pixmap,
        pos_bounds: Rect,
        layout_bounds: Rect,
    ) -> AppResult<DrawResult> {
        self.pos_bounds = Some(pos_bounds);
        self.layout_bounds = Some(layout_bounds);
        let from = self.from();
        let to = self.to();
        let w = self.width;
        let mut pb = PathBuilder::default();
        pb.move_to(from.x(), from.y());
        pb.line_to(to.x(), to.y());
        let path = pb.finish().unwrap();
        let paint = rgba_paint!(self.color);
        let mut line_pixmap = empty_pixmap!(to.x() + w, to.y() + w);
        line_pixmap.stroke_path(
            &path,
            &paint,
            &Stroke {
                width: w,
                ..Stroke::default()
            },
            Transform::default(),
            None,
        );
        if let Some(shadow) = self.shadow {
            line_pixmap = shadow.draw(line_pixmap)?;
        }
        Ok(DrawResult(
            merge_pixmap!(pixmap, line_pixmap),
            path.bounds(),
        ))
    }
}

impl Line {
    fn from(&self) -> Position {
        let x = self
            .from
            .and_then(|from| from.x)
            .and_then(|x| Some(x + self.layout_bounds().left()))
            .unwrap_or(self.pos_bounds().left());
        let y = self
            .from
            .and_then(|from| from.y)
            .and_then(|y| Some(y + self.layout_bounds().top()))
            .unwrap_or(self.pos_bounds().top());
        (x, y).into()
    }
    fn to(&self) -> Position {
        let x = self
            .to
            .and_then(|to| to.x)
            .and_then(|x| Some(x + self.layout_bounds().left()))
            .unwrap_or(self.pos_bounds().left());
        let y = self
            .to
            .and_then(|to| to.y)
            .and_then(|y| Some(y + self.layout_bounds().top()))
            .unwrap_or(self.pos_bounds().top());
        (x, y).into()
    }
    fn pos_bounds(&self) -> Rect {
        self.pos_bounds.unwrap_or(empty_rect!())
    }
    fn layout_bounds(&self) -> Rect {
        self.layout_bounds.unwrap_or(empty_rect!())
    }
}
