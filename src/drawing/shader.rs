use std::f32::consts::PI;

use tiny_skia::{GradientStop, LinearGradient, Point, Shader, Transform};

use crate::{
    color,
    graphic::rectangle::Corner,
    matrix::*,
    utils::{make_error, AppResult},
};

pub fn create_linear_gradient<'a>(
    mut angle: f32,
    position: Position,
    size: Size,
    corner: Corner,
    colors: Vec<color::ColorStop>,
) -> AppResult<Shader<'a>> {
    let x = position.x();
    let y = position.y();
    let w = size.width();
    let h = size.height();
    let Corner(c0, c1, c2, c3) = corner.get_fitted(&(w, h).into());
    let (mid_w, mid_h) = (w / 2., h / 2.);
    let lt = Point::from_xy(x, y);
    let rt = Point::from_xy(x + w, y);
    let rb = Point::from_xy(x + w, y + h);
    let lb = Point::from_xy(x, y + h);
    let center: Point = Point::from_xy(x + mid_w, y + mid_h);
    let at = (mid_w / mid_h).atan();
    let origin = Point::zero();
    angle = angle / 180. * PI % (2. * PI);
    if angle < 0. {
        angle += 2. * PI;
    }
    let calc_endian_v = |angle: f32| {
        let l = angle.tan() * mid_h;
        let mut end = Point::from_xy(l, 0.) - Point::from_xy(0., mid_h);
        end.set_length(end.length() + angle.sin() * (mid_w - l.abs()));
        let start = -end;
        (start, end)
    };
    let calc_endian_h = |angle: f32| {
        let mut l = mid_w / angle.tan();
        if angle > PI / 4. {
            l = -l;
        }
        let mut end = Point::from_xy(0., l) + Point::from_xy(mid_w, 0.);
        end.set_length(end.length() + (mid_h - l.abs()) * angle.cos());
        let start = -end;
        (start, end)
    };
    let (start, end) = if angle == 0. || angle == 2. * PI {
        (Point::from_xy(0., mid_h), Point::from_xy(0., -mid_h))
    } else if angle == PI / 2. {
        (Point::from_xy(-mid_w, 0.), Point::from_xy(mid_w, 0.))
    } else if angle == PI {
        (Point::from_xy(0., -mid_h), Point::from_xy(0., mid_h))
    } else if angle == PI / 2. * 3. {
        (Point::from_xy(mid_w, 0.), Point::from_xy(-mid_w, 0.))
    } else if angle.tan().abs() * mid_h <= mid_w + 0.01 {
        // +0.01 to mid_w is to solve accuracy bug
        if angle <= PI / 2. || angle >= PI / 2. * 3. {
            calc_endian_v(angle)
        } else {
            let (start, end) = calc_endian_v((angle + PI) % (2. * PI));
            (end, start)
        }
    } else if angle <= PI {
        calc_endian_h(angle)
    } else {
        let (start, end) = calc_endian_h((angle + PI) % (2. * PI));
        (end, start)
    };
    let (start, end) = (start + center, end + center);
    let distance = start.distance(end);
    let stops: Vec<_> = colors
        .iter()
        .map(|color| {
            let pos = match color.position {
                color::GradientStopPosition::Percent(p) => p,
                color::GradientStopPosition::Pixel(p) => p / distance,
            };
            let color::Rgba(r, g, b, a) = color.color;
            GradientStop::new(pos, tiny_skia::Color::from_rgba8(r, g, b, a))
        })
        .collect();
    LinearGradient::new(
        start,
        end,
        stops,
        tiny_skia::SpreadMode::Pad,
        Transform::identity(),
    )
    .map_or(Err(make_error("create linear gradient fail!")), |v| Ok(v))
}
