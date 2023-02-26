#![allow(unused_variables, unused_imports)]
mod canvas;
mod drawing;
mod graphics;
mod utils;
use graphics::{Color, Graphic, Position, Size};
use tiny_skia::Pixmap;
use utils::{make_error, AppResult};
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CanvasBuilder {
    pub size: Size,
    pub background: Color,
    pub graphics: Vec<Graphic>,
}

#[wasm_bindgen]
pub fn draw(val: String) -> Result<Vec<u8>, JsValue> {
    let CanvasBuilder {
        size,
        background,
        graphics,
    }: CanvasBuilder = serde_json::from_str(&val).map_err(|_| make_error("invalid options"))?;

    let mut cvs = canvas::Canvas::new(size)?;
    cvs.draw_rectangle(Graphic::Rectangle {
        corner: None,
        color: background,
        shadow: None,
        position: (0., 0.).into(),
        size,
    })?;
    for graphic in graphics {
        match graphic {
            // Graphic::Rectangle => cvs.draw_rectangle(graphic)?,
            Graphic::Rectangle { .. } => cvs.draw_rectangle(graphic)?,
            _ => (),
        };
    }
    return Ok(cvs.export()?);
}

// #[wasm_bindgen]
// pub fn make_watermark(text: &str, font_data: &[u8]) -> Vec<u8> {
//     let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing font");
//     let scale = Scale::uniform(32.0);
//     let color = (255, 0, 0);
//     let v_metrics = font.v_metrics(scale);
//     let glyphs: Vec<_> = font
//         .layout(text, scale, point(0., v_metrics.ascent))
//         .collect();
//     let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
//     let glyphs_width = {
//         let min = glyphs
//             .first()
//             .map(|g| g.pixel_bounding_box().unwrap().min.x)
//             .unwrap();
//         let max = glyphs
//             .last()
//             .map(|g| g.pixel_bounding_box().unwrap().max.x)
//             .unwrap();
//         (max - min) as u32
//     };
//     let mut img = DynamicImage::new_rgba8(glyphs_width, glyphs_height).to_rgba8();
//     let area = (0..img.width(), 0..img.height());
//     for glyph in glyphs {
//         if let Some(bounding_box) = glyph.pixel_bounding_box() {
//             glyph.draw(|mut x, mut y, v| {
//                 x = x + bounding_box.min.x as u32;
//                 y = y + bounding_box.min.y as u32;
//                 if area.0.contains(&x) && area.1.contains(&y) {
//                     img.put_pixel(x, y, Rgba([color.0, color.1, color.2, (v * 255.0) as u8]))
//                 }
//             })
//         }
//     }

//     let mut f: Cursor<Vec<u8>> = Cursor::new(Vec::new());
//     img.write_to(&mut f, ImageOutputFormat::Png).unwrap();
//     f.into_inner()
// }

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use image::{DynamicImage, EncodableLayout, Rgba};
    use rusttype::{point, Font, Scale};

    use super::*;

    #[test]
    fn it_works() {
        let s = r#"{
            "size":[1200,800],
            "background":{"type":"Rgba","value":[255,255,0,200]},
            "graphics":[
                { "type": "Rectangle", "value": { "corner": [24, 24, 24, 24], "color": { "type": "Rgba", "value": [255, 0, 0, 120] }, "position": [30, 30], "size": [400, 300] } }
            ]
        }"#;
        let buff = draw(s.to_string()).unwrap();
        save(&buff);
    }

    fn save(buff: &Vec<u8>) {
        let mut f = File::options()
            .write(true)
            .create(true)
            .open("test.png")
            .unwrap();
        f.write(buff).unwrap();
    }
}
