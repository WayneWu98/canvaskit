use std::cmp::min;

use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use wasm_bindgen::JsValue;

use crate::{
    graphics::{self, Color, Corner, Graphic, Position, Size},
    utils::{self, error_mapper, make_error, AppResult},
};

pub struct Canvas {
    pub pixmap: tiny_skia::Pixmap,
}

impl Canvas {
    pub fn new(size: Size) -> AppResult<Self> {
        Ok(Self {
            pixmap: Pixmap::new(size.0 as u32, size.1 as u32)
                .map_or(Err(make_error("init pixmap fail!")), |v| Ok(v))?,
        })
    }

    pub fn draw_rectangle(&mut self, mut graphic: Graphic) -> AppResult {
        if let Graphic::Rectangle {
            corner,
            color,
            shadow,
            position: Position(x, y),
            size: Size(w, h),
        } = graphic.clone()
        {
            let mut pb = PathBuilder::new();
            let Corner(c0, c1, c2, c3) = corner
                .or(Some(graphics::Corner::default()))
                .map(|c| c.get_fitted(&(w, h).into()))
                .unwrap();
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
            let path = pb
                .finish()
                .map_or(Err(make_error("path generation fail!")), |v| Ok(v))?;
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.shader = graphic.get_shader()?;
            self.pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
        Ok(())
    }

    pub fn export(&self) -> AppResult<Vec<u8>> {
        Ok(self
            .pixmap
            .encode_png()
            .map_err(|_| make_error("export fail!"))?)
    }
}
