use serde::Deserialize;
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Rect, Shader, Stroke, Transform};

use crate::{
    color,
    drawing::shader::{self, create_linear_gradient},
    effects,
    matrix::*,
    utils::{self, make_error, AppResult},
};

use super::Draw;

#[derive(Deserialize, Debug, Clone)]
pub struct Line {
    pub from: Position,
    pub to: Position,
    pub width: f32,
    pub color: color::Rgba,
    pub shadow: Option<effects::DropShadow>,
    #[serde(skip)]
    pub layout_bounds: Option<Box<Rect>>,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            from: Position::default(),
            to: Position::default(),
            width: 1.,
            color: color::Rgba(0, 0, 0, 255),
            shadow: None,
            layout_bounds: None,
        }
    }
}

impl Draw for Line {
    fn draw(
        &mut self,
        pixmap: &mut Pixmap,
        bounds: Rect,
        layout_bounds: Option<Box<Rect>>,
    ) -> AppResult<Rect> {
        self.layout_bounds = layout_bounds;
        let mut pb = PathBuilder::default();
        pb.move_to(self.from().x(), self.from().y());
        pb.line_to(self.to().x(), self.to().y());
        let path = pb
            .finish()
            .map_or(Err(make_error("create path fail!!")), |v| Ok(v))?;

        let paint = utils::create_rgba_paint(self.color);
        let mut g_pixmap = utils::create_empty_pixmap(pixmap.width(), pixmap.height())?;
        g_pixmap.stroke_path(
            &path,
            &paint,
            &Stroke {
                width: self.width,
                ..Stroke::default()
            },
            Transform::default(),
            None,
        );
        if let Some(shadow) = self.shadow {
            shadow.draw(pixmap, &g_pixmap)?;
        }
        utils::merge_pixmap(pixmap, &g_pixmap, None);
        Ok(path.bounds().into())
    }
}

impl Line {
    fn from(&self) -> Position {
        if let Some(ref lb) = self.layout_bounds {
            return (self.from.x() + lb.left(), self.from.y() + lb.top()).into();
        }
        self.from
    }
    fn to(&self) -> Position {
        if let Some(ref lb) = self.layout_bounds {
            return (self.to.x() + lb.left(), self.to.y() + lb.top()).into();
        }
        self.to
    }
}
