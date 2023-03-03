use serde::Deserialize;
use tiny_skia::{
    BlendMode, FillRule, FilterQuality, Paint, Path, PathBuilder, Pixmap, PixmapPaint, Rect,
    Shader, Transform,
};
use wasm_bindgen_test::console_log;

use crate::{
    color,
    drawing::shader::{self, create_linear_gradient},
    effects, empty_pixmap, empty_rect, expand_pixmap, lt_rect, ltrb_rect, merge_pixmap,
    metrics::*,
    paint, rgba_paint,
    utils::{self, make_error, AppResult, Union},
};

use super::{Align, Draw, DrawResult, Graphic};

#[derive(Deserialize, Debug, Clone)]
pub struct Container {
    pub corner: Option<Corner>,
    pub color: Option<color::Color>,
    pub shadow: Option<effects::BoxShadow>,
    pub position: Option<Position>,
    pub size: Option<Size>,
    pub border: Option<Border>,
    pub children: Option<Vec<Graphic>>,
    pub padding: Option<Padding>,
    pub align: Option<Align>,
    #[serde(default)]
    pub clip: bool,
    #[serde(skip)]
    children_bounds: Option<Rect>,
    #[serde(skip)]
    pos_bounds: Option<Rect>,
    #[serde(skip)]
    layout_bounds: Option<Rect>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            corner: None,
            color: None,
            shadow: None,
            position: Some((0., 0.).into()),
            size: Some((0., 0.).into()),
            border: None,
            children: None,
            padding: None,
            align: None,
            children_bounds: None,
            clip: false,
            pos_bounds: None,
            layout_bounds: None,
        }
    }
}

impl Draw for Container {
    fn draw(
        &mut self,
        mut pixmap: Pixmap,
        pos_bounds: Rect,
        layout_bounds: Rect,
    ) -> AppResult<DrawResult> {
        self.pos_bounds = Some(pos_bounds);
        self.layout_bounds = Some(layout_bounds);
        let mut children_bounds = self.children_layout_bounds();
        let mut children_pixmap = empty_pixmap!(pixmap.width(), pixmap.height());
        if let Some(children) = self.children.take() {
            let layout_bounds = self.children_layout_bounds();
            let mut pos_bounds = layout_bounds.clone();
            for mut child in children {
                let DrawResult(pixmap, bounds) =
                    child.draw(children_pixmap, pos_bounds, layout_bounds)?;
                children_pixmap = pixmap;
                pos_bounds = lt_rect!(bounds.right(), bounds.bottom());
                children_bounds = children_bounds.union(&bounds);
            }
        }
        self.children_bounds = Some(children_bounds);
        let path = self.path()?;
        if let Some(shadow) = self.shadow {
            pixmap = shadow.draw(pixmap, &path)?;
        }
        let bounds = path.bounds();
        pixmap = expand_pixmap!(bounds, pixmap);
        if self.color.is_some() {
            let paint = self.paint()?;
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
        if let Some(border) = self.border {
            border.draw(&mut pixmap, &path)?;
        }
        if self.clip {
            return Ok(DrawResult(
                merge_pixmap!(
                    pixmap,
                    children_pixmap,
                    tiny_skia::BlendMode::SourceOver,
                    Some(&path)
                ),
                bounds,
            ));
        }
        Ok(DrawResult(merge_pixmap!(pixmap, children_pixmap), bounds))
    }
}

impl Container {
    pub fn paint(&self) -> AppResult<Paint> {
        let mut paint = paint!();
        paint.shader = match self.color() {
            color::Color::Rgba(color::Rgba(r, g, b, a)) => {
                Shader::SolidColor(tiny_skia::Color::from_rgba8(r, g, b, a))
            }
            color::Color::Gradient(color::LinearGradient { angle, stops }) => {
                let x = self.x();
                let y = self.y();
                let w = self.width();
                let h = self.height();

                create_linear_gradient(
                    angle,
                    (x, y).into(),
                    (w, h).into(),
                    self.corner(),
                    stops.clone(),
                )?
            }
        };
        Ok(paint)
    }

    pub fn path(&self) -> AppResult<Path> {
        let x = self.x();
        let y = self.y();
        let w = self.width();
        let h = self.height();
        let mut pb = PathBuilder::new();
        let Corner(c0, c1, c2, c3) = self.corner();
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

    pub fn corner(&self) -> Corner {
        self.corner
            .unwrap_or_default()
            .get_fitted(&(self.width(), self.height()).into())
    }

    pub fn color(&self) -> color::Color {
        if let Some(ref color) = self.color {
            color.clone()
        } else {
            color::Color::default()
        }
    }

    pub fn x(&self) -> f32 {
        if let Some(ref position) = self.position {
            if let Some(ref x) = position.x {
                return self.align.or(Some(Align::Left)).unwrap().x(
                    *x,
                    self.width(),
                    &self.layout_bounds(),
                );
            }
        }
        if let Some(ref align) = self.align {
            return align.x(0., self.width(), &self.layout_bounds());
        }
        self.pos_bounds().left()
    }
    pub fn y(&self) -> f32 {
        if let Some(ref position) = self.position {
            if let Some(ref y) = position.y {
                return *y + self.layout_bounds().top();
            }
        }
        self.pos_bounds().top()
    }
    pub fn padding(&self) -> Padding {
        self.padding.unwrap_or_default()
    }
    pub fn width(&self) -> f32 {
        if let Some(ref size) = self.size {
            if let Some(ref w) = size.width {
                return *w;
            }
        }
        self.layout_bounds().width()
    }
    pub fn height(&self) -> f32 {
        if let Some(ref size) = self.size {
            if let Some(ref h) = size.height {
                return *h;
            }
        }
        let p = self.padding();
        self.children_bounds().height() + p.top() + p.bottom()
    }
    fn layout_bounds(&self) -> Rect {
        self.layout_bounds.unwrap_or(empty_rect!())
    }
    fn children_bounds(&self) -> Rect {
        self.children_bounds.unwrap_or(empty_rect!())
    }
    fn pos_bounds(&self) -> Rect {
        self.pos_bounds.unwrap_or(empty_rect!())
    }
    fn children_layout_bounds(&self) -> Rect {
        let padding = self.padding();
        let x = self.x();
        let y = self.y();
        let l = x + padding.left();
        let t = y + padding.top();
        let r = x + self.width() - padding.right();
        let b = y + self.height() - padding.bottom();
        ltrb_rect!(l, t, r, b)
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
        let stroke = tiny_skia::Stroke {
            width: self.width,
            ..tiny_skia::Stroke::default()
        };
        pixmap.stroke_path(
            path,
            &rgba_paint!(self.color),
            &stroke,
            Transform::identity(),
            None,
        );
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
