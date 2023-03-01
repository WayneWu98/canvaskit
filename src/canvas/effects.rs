use fastblur::{gaussian_blur_asymmetric, gaussian_blur_asymmetric_single_channel};
use serde::Deserialize;
use tiny_skia::{
    BlendMode, FillRule, FilterQuality, Paint, Path, Pixmap, PixmapPaint, PremultipliedColorU8,
    Rect, Shader, Transform,
};
use wasm_bindgen_test::console_log;

use crate::{
    color, empty_pixmap, expand_pixmap,
    matrix::Position,
    merge_pixmap, rgba_paint,
    utils::{self, make_error, AppResult},
    xywh_rect,
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
    pub fn draw(&self, mut pixmap: Pixmap, path: &Path) -> AppResult<Pixmap> {
        let path = path
            .clone()
            .transform(Transform::from_translate(self.x, self.y))
            .unwrap_or_else(|| path.clone());
        let bounds = path.bounds();
        let cx = bounds.x() + bounds.width() / 2.;
        let cy = bounds.y() + bounds.height() / 2.;
        let w = bounds.width();
        let h = bounds.height();
        let sx = (w + self.spread) / w;
        let sy = (h + self.spread) / h;
        let path = path
            .clone()
            .transform(Transform::from_translate(-cx, -cy))
            .and_then(|path| path.transform(Transform::from_scale(sx, sy)))
            .and_then(|path| path.transform(Transform::from_translate(cx, cy)))
            .unwrap_or_else(|| path.clone());
        let bounds = path.bounds();
        let mut blurred =
            empty_pixmap!(bounds.right() + self.spread, bounds.bottom() + self.spread);

        blurred.fill(tiny_skia::Color::from_rgba8(0, 0, 0, 0));
        blurred.fill_path(
            &path,
            &rgba_paint!(self.color),
            FillRule::Winding,
            Transform::identity(),
            None,
        );
        blur(&mut blurred, self.blur, self.blur);
        Ok(merge_pixmap!(pixmap, blurred))
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
    pub fn draw(&self, mut pixmap: Pixmap) -> AppResult<Pixmap> {
        let w = pixmap.width() as f32 + self.blur + self.x;
        let h = pixmap.height() as f32 + self.blur + self.y;
        let mut shadow_pixmap = expand_pixmap!(xywh_rect!(0., 0., w, h), pixmap.clone());
        let pixels = shadow_pixmap.pixels_mut();
        for i in 0..pixels.len() {
            let color::Rgba(r, g, b, a) = self.color;
            let a = (pixels[i].alpha() as f32 * a as f32 / 255.) as u8;
            pixels[i] = tiny_skia::ColorU8::from_rgba(r, g, b, a).premultiply();
        }
        blur(&mut shadow_pixmap, self.blur, self.blur);
        Ok(merge_pixmap!(
            pixmap,
            shadow_pixmap,
            self.x,
            self.y,
            BlendMode::DestinationOver
        ))
    }
}

fn blur(pixmap: &mut Pixmap, blur_x: f32, blur_y: f32) {
    let w = pixmap.width() as usize;
    let h = pixmap.height() as usize;
    let data = pixmap.data_mut();
    let mut pixels = Vec::with_capacity(w * h);
    for mut i in 0..(data.len() / 4) {
        i *= 4;
        pixels.push([data[i], data[i + 1], data[i + 2], data[i + 3]]);
    }
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
