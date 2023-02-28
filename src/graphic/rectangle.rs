use serde::Deserialize;
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Rect, Shader, Transform};
use wasm_bindgen_test::console_log;

use crate::{
    color,
    drawing::shader::{self, create_linear_gradient},
    effects,
    matrix::*,
    utils::{self, make_error, AppResult, Union},
};

use super::{Draw, Graphic};

#[derive(Deserialize, Debug, Clone)]
pub struct Rectangle {
    pub corner: Option<Corner>,
    pub color: color::Color,
    pub shadow: Option<effects::BoxShadow>,
    pub position: Option<Position>,
    pub size: Option<Size>,
    pub border: Option<Border>,
    pub children: Option<Vec<Graphic>>,
    pub padding: Option<Padding>,
    #[serde(skip)]
    pub layout_bounds: Option<Box<Rect>>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            corner: None,
            color: color::Color::Rgba(color::Rgba(255, 255, 255, 255)),
            shadow: None,
            position: Some((0., 0.).into()),
            size: Some((0., 0.).into()),
            border: None,
            children: None,
            padding: None,
            layout_bounds: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Corner(pub f32, pub f32, pub f32, pub f32);
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Border {
    pub width: f32,
    pub color: color::Rgba,
}

impl Border {
    pub fn draw(&self, pixmap: &mut Pixmap, path: &Path) -> AppResult {
        let mut paint = utils::create_paint();
        paint.shader = Shader::SolidColor(self.color.into());
        let stroke = tiny_skia::Stroke {
            width: self.width,
            ..tiny_skia::Stroke::default()
        };
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
        Ok(())
    }
}

impl Default for Corner {
    fn default() -> Self {
        Self(0., 0., 0., 0.)
    }
}

impl Corner {
    pub fn get_fitted(&self, Size { width, height }: &Size) -> Self {
        let w = width.unwrap_or(0.);
        let h = height.unwrap_or(0.);
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

impl Draw for Rectangle {
    fn draw(
        &mut self,
        pixmap: &mut Pixmap,
        bounds: Rect,
        layout_bounds: Option<Box<Rect>>,
    ) -> AppResult<Rect> {
        self.layout_bounds = layout_bounds;
        let mut children_bounds = self.child_layout_bounds(Some(&bounds));
        let mut children_pixmap = utils::create_empty_pixmap(pixmap.width(), pixmap.height())?;
        if let Some(children) = self.children.take() {
            let mut child_bounds = children_bounds.clone();
            for mut graphic in children {
                let nb = graphic.draw(
                    &mut children_pixmap,
                    child_bounds,
                    Some(Box::new(self.child_layout_bounds(Some(&bounds)))),
                )?;
                child_bounds =
                    Rect::from_xywh(nb.left() + nb.width(), nb.top() + nb.height(), 0., 0.)
                        .unwrap_or(nb);
                children_bounds = children_bounds.union(&child_bounds);
            }
        }
        let path = self.get_path(&children_bounds)?;
        let paint = self.get_paint(&children_bounds)?;
        if let Some(shadow) = self.shadow {
            shadow.draw(pixmap, &path)?;
        }
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
        if let Some(border) = self.border {
            border.draw(pixmap, &path)?;
        }
        utils::merge_pixmap(pixmap, &mut children_pixmap, None);
        Ok(path.bounds())
    }
}

impl Rectangle {
    pub fn get_paint(&self, bounds: &Rect) -> AppResult<Paint> {
        let mut paint = utils::create_paint();
        paint.shader = match self.color.clone() {
            color::Color::Rgba(color::Rgba(r, g, b, a)) => {
                Shader::SolidColor(tiny_skia::Color::from_rgba8(r, g, b, a))
            }
            color::Color::Gradient(color::LinearGradient { angle, stops }) => {
                let x = self.x(Some(bounds));
                let y = self.y(Some(bounds));
                let w = self.width(Some(bounds));
                let h = self.height(Some(bounds));

                create_linear_gradient(
                    angle,
                    (x, y).into(),
                    (w, h).into(),
                    self.corner(bounds),
                    stops.clone(),
                )?
            }
        };
        Ok(paint)
    }

    pub fn get_path(&self, bounds: &Rect) -> AppResult<Path> {
        let Rectangle { size, position, .. } = self.clone();
        let x = self.x(Some(bounds));
        let y = self.y(Some(bounds));
        let w = self.width(Some(bounds));
        let h = self.height(Some(bounds));
        let mut pb = PathBuilder::new();
        let Corner(c0, c1, c2, c3) = self.corner(bounds);
        let (mid_x, mid_y) = (x + w / 2., y + h / 2.);
        let (max_x, max_y) = (x + w, y + h);
        pb.move_to(x, mid_y);
        if c0 == 0. {
            pb.line_to(x, y);
        } else {
            let offset = 0.45 * c0;
            pb.line_to(x, y + c0);
            pb.cubic_to(x, y + offset, x + offset, y, x + c0, y);
        }
        pb.line_to(mid_x, y);
        if c1 == 0. {
            pb.line_to(max_x, y);
        } else {
            let offset = 0.45 * c1;
            pb.line_to(max_x - c1, y);
            pb.cubic_to(max_x - offset, y, max_x, y + offset, max_x, y + c1);
        }
        pb.line_to(max_x, mid_y);
        if c2 == 0. {
            pb.line_to(max_x, max_y);
        } else {
            let offset = 0.45 * c2;
            pb.line_to(max_x, max_y - c2);
            pb.cubic_to(
                max_x,
                max_y - offset,
                max_x - offset,
                max_y,
                max_x - c2,
                max_y,
            );
        }
        pb.line_to(mid_x, max_y);
        if c3 == 0. {
            pb.line_to(x, max_y);
        } else {
            let offset = 0.45 * c3;
            pb.line_to(x + c3, max_y);
            pb.cubic_to(x + offset, max_y, x, max_y - offset, x, max_y - c3);
        }
        pb.line_to(x, mid_y);
        pb.close();
        pb.finish()
            .map_or(Err(make_error("path generation fail!")), |v| Ok(v))
    }

    pub fn corner(&self, bounds: &Rect) -> Corner {
        self.corner
            .unwrap_or_default()
            .get_fitted(&(self.width(Some(bounds)), self.height(Some(bounds))).into())
    }

    pub fn x(&self, bounds: Option<&Rect>) -> f32 {
        if let Some(ref position) = self.position {
            if let Some(ref x) = position.x {
                return *x
                    + self
                        .layout_bounds
                        .clone()
                        .and_then(|r| Some(r.left()))
                        .unwrap_or(0.);
            }
        }
        return bounds.and_then(|v| Some(v.left())).unwrap_or(0.);
    }
    pub fn y(&self, bounds: Option<&Rect>) -> f32 {
        if let Some(ref position) = self.position {
            if let Some(ref y) = position.y {
                return *y
                    + self
                        .layout_bounds
                        .clone()
                        .and_then(|r| Some(r.top()))
                        .unwrap_or(0.);
            }
        }
        return bounds.and_then(|v| Some(v.top())).unwrap_or(0.);
    }
    pub fn padding(&self) -> Padding {
        self.padding.unwrap_or_default()
    }
    pub fn width(&self, bounds: Option<&Rect>) -> f32 {
        if let Some(ref size) = self.size {
            if let Some(ref w) = size.width {
                return *w;
            }
        }
        return bounds.and_then(|v| Some(v.width())).unwrap_or(0.)
            + self.padding().left()
            + self.padding().right();
    }
    pub fn height(&self, bounds: Option<&Rect>) -> f32 {
        if let Some(ref size) = self.size {
            if let Some(ref h) = size.height {
                return *h;
            }
        }
        return bounds.and_then(|v| Some(v.height())).unwrap_or(0.)
            + self.padding().top()
            + self.padding().bottom();
    }
    fn child_layout_bounds(&self, bounds: Option<&Rect>) -> Rect {
        let x = self.x(bounds) + self.padding().left();
        let y = self.y(bounds) + self.padding().top();
        Rect::from_xywh(x, y, 0., 0.).unwrap()
    }
}
