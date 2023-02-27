use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Color {
    Rgba(Rgba),
    Gradient(LinearGradient),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum GradientStopPosition {
    Percent(f32),
    Pixel(f32),
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl From<Rgba> for tiny_skia::Color {
    fn from(Rgba(r, g, b, a): Rgba) -> Self {
        tiny_skia::Color::from_rgba8(r, g, b, a)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LinearGradient {
    pub angle: f32,
    pub stops: Vec<ColorStop>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ColorStop {
    pub position: GradientStopPosition,
    pub color: Rgba,
}
