use fastblur::{gaussian_blur_asymmetric, gaussian_blur_asymmetric_single_channel};
use serde::Deserialize;
use tiny_skia::{FillRule, Paint, Path, Pixmap, Transform};
use wasm_bindgen_test::console_log;

use crate::{
    color,
    matrix::Position,
    utils::{self, make_error, AppResult},
};

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct BoxShadow {
    pub x: f32,
    pub y: f32,
    pub spread: f32,
    pub blur: f32,
    pub color: color::Rgba,
}

impl BoxShadow {
    pub fn draw(&self, pixmap: &mut Pixmap, path: &Path) -> AppResult {
        let path = path
            .clone()
            .transform(Transform::from_translate(self.x, self.y))
            .unwrap_or_else(|| path.clone());
        let bounds = path.bounds();
        let w = bounds.width();
        let h = bounds.height();
        let sx = (w + self.spread) / w;
        let sy = (h + self.spread) / h;
        let path = path
            .clone()
            .transform(Transform::from_row(
                sx,
                0.,
                0.,
                sy,
                -self.spread / 2.,
                -self.spread / 2.,
            ))
            .unwrap_or_else(|| path.clone());
        let mut blurred = Pixmap::new(pixmap.width(), pixmap.height())
            .map_or(Err(make_error("create pixmap fail!!")), |v| Ok(v))?;

        blurred.fill(tiny_skia::Color::from_rgba8(0, 0, 0, 0));
        blurred.fill_path(
            &path,
            &utils::create_rgba_paint(self.color),
            FillRule::Winding,
            Transform::identity(),
            None,
        );
        blur(&mut blurred, self.blur, self.blur);
        utils::merge_pixmap(pixmap, &blurred, None);
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct DropShadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub color: color::Rgba,
}

impl DropShadow {
    pub fn draw(&self, pixmap: &mut Pixmap, parent: &Pixmap) -> AppResult {
        let w = pixmap.width();
        let h = pixmap.height();
        let mut shadow_pixmap = utils::create_empty_pixmap(w, h)?;
        utils::merge_pixmap(&mut shadow_pixmap, parent, Some(Position(self.x, self.y)));
        blur(&mut shadow_pixmap, self.blur, self.blur);
        utils::merge_pixmap(pixmap, &shadow_pixmap, None);
        Ok(())
    }
}

fn blur(pixmap: &mut Pixmap, blur_x: f32, blur_y: f32) {
    let w = pixmap.width() as usize;
    let h = pixmap.height() as usize;
    let pixels = pixmap
        .pixels()
        .iter()
        .map(|color| [color.red(), color.green(), color.blue(), color.alpha()])
        .collect::<Vec<_>>();

    let data = pixmap.data_mut();
    for i in 0..4 {
        let mut pixels = pixels
            .iter()
            .map(|item| *item.get(i).unwrap())
            .collect::<Vec<_>>();
        fastblur::gaussian_blur_asymmetric_single_channel(&mut pixels, w, h, blur_x, blur_y);
        for (j, pixel) in pixels.into_iter().enumerate() {
            data[j * 4 + i] = pixel;
        }
    }
}
