use serde::Deserialize;
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Shader, Transform};
use wasm_bindgen_test::console_log;

use crate::{
    color,
    drawing::shader::{self, create_linear_gradient},
    effects,
    matrix::*,
    utils::{self, make_error, AppResult},
};

use super::Draw;

#[derive(Deserialize, Debug, Clone)]
pub struct Rectangle {
    pub corner: Option<Corner>,
    pub color: color::Color,
    pub shadow: Option<effects::BoxShadow>,
    pub position: Position,
    pub size: Size,
    pub border: Option<Border>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            corner: None,
            color: color::Color::Rgba(color::Rgba(255, 255, 255, 255)),
            shadow: None,
            position: Position(0., 0.),
            size: Size(0., 0.),
            border: None,
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
    pub fn get_fitted(&self, Size(w, h): &Size) -> Self {
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
    fn draw(&self, pixmap: &mut Pixmap) -> AppResult {
        let path = self.get_path()?;
        let paint = self.get_paint()?;
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
        Ok(())
    }
}

impl Rectangle {
    pub fn get_paint(&self) -> AppResult<Paint> {
        let mut paint = utils::create_paint();
        paint.shader = match self.color.clone() {
            color::Color::Rgba(color::Rgba(r, g, b, a)) => {
                Shader::SolidColor(tiny_skia::Color::from_rgba8(r, g, b, a))
            }
            color::Color::Gradient(color::LinearGradient { angle, stops }) => {
                create_linear_gradient(
                    angle,
                    self.position,
                    self.size,
                    self.corner(),
                    stops.clone(),
                )?
            }
        };
        Ok(paint)
    }

    pub fn get_path(&self) -> AppResult<Path> {
        let Rectangle {
            size: Size(w, h),
            position: Position(x, y),
            ..
        } = self.clone();
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
        self.corner.unwrap_or_default().get_fitted(&self.size)
    }
}
