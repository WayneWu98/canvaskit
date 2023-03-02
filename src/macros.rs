#[macro_export]
macro_rules! xywh_rect {
    ($x: expr, $y: expr, $w: expr, $h: expr) => {
        Rect::from_xywh($x, $y, $w, $h).unwrap()
    };
}

#[macro_export]
macro_rules! ltrb_rect {
    ($l: expr, $t: expr, $r: expr, $b: expr) => {
        Rect::from_ltrb($l, $t, $r, $b).unwrap()
    };
}

#[macro_export]
macro_rules! lt_rect {
    ($l: expr, $t: expr) => {
        Rect::from_xywh($l, $t, 0., 0.).unwrap()
    };
}

#[macro_export]
macro_rules! empty_rect {
    () => {
        crate::xywh_rect!(0., 0., 0., 0.)
    };
}
#[macro_export]
macro_rules! paint {
    () => {{
        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint
    }};
}

#[macro_export]
macro_rules! rgba_paint {
    ($color: expr) => {{
        let mut paint = crate::paint!();
        paint.shader = Shader::SolidColor($color.into());
        paint
    }};
}

#[macro_export]
macro_rules! empty_pixmap {
    ($w: expr, $h: expr) => {
        Pixmap::new($w as u32, $h as u32).unwrap()
    };
}

#[macro_export]
macro_rules! merge_pixmap {
    ($a: expr, $b: expr) => {
        merge_pixmap!($a, $b, 0., 0., BlendMode::SourceOver, None)
    };
    ($a: expr, $b: expr, $mode: expr, $path: expr) => {
        merge_pixmap!($a, $b, 0., 0., $mode, $path)
    };
    ($a: expr, $b: expr, $x: expr, $y: expr, $mode: expr) => {
        merge_pixmap!($a, $b, $x, $y, $mode, None)
    };
    ($a: expr, $b: expr, $x: expr, $y: expr, $mode: expr, $path: expr) => {{
        let aw = $a.width();
        let ah = $a.height();
        let bw = $b.width();
        let bh = $b.height();
        let paint = PixmapPaint {
            blend_mode: $mode,
            opacity: 1.,
            quality: FilterQuality::Nearest,
        };
        let transform = Transform::identity();
        let mut mask;
        let mut option_mask = None;
        if (bw > aw || bh > ah) {
            let mut new_pixmap = crate::empty_pixmap!(aw.max(bw), ah.max(bh));
            if let Some(path) = $path {
                mask = tiny_skia::ClipMask::new();
                mask.set_path(
                    new_pixmap.width(),
                    new_pixmap.height(),
                    path,
                    tiny_skia::FillRule::EvenOdd,
                    true,
                );
                option_mask = Some(&mask)
            }
            new_pixmap.draw_pixmap(0, 0, $a.as_ref(), &paint, transform, None);
            new_pixmap.draw_pixmap(
                $x as i32,
                $y as i32,
                $b.as_ref(),
                &paint,
                transform,
                option_mask,
            );
            new_pixmap
        } else {
            if let Some(path) = $path {
                mask = tiny_skia::ClipMask::new();
                mask.set_path(
                    $a.width(),
                    $a.height(),
                    path,
                    tiny_skia::FillRule::EvenOdd,
                    true,
                );
                option_mask = Some(&mask)
            }
            $a.draw_pixmap(
                $x as i32,
                $y as i32,
                $b.as_ref(),
                &paint,
                transform,
                option_mask,
            );
            $a
        }
    }};
}

#[macro_export]
macro_rules! expand_pixmap {
    ($bounds: expr, $pixmap: expr) => {{
        let bw = $bounds.right();
        let bh = $bounds.bottom();
        let pw = $pixmap.width() as f32;
        let ph = $pixmap.height() as f32;
        if (bw > pw || bh > ph) {
            let mut new_pixmap = crate::empty_pixmap!(bw.max(pw) as u32, bh.max(ph) as u32);
            new_pixmap.draw_pixmap(
                0,
                0,
                $pixmap.as_ref(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
            new_pixmap
        } else {
            $pixmap
        }
    }};
}

#[macro_export]
macro_rules! clip_pixmap {
    ($pixmap: expr, $bounds: expr) => {{
        let mut new_pixmap = crate::empty_pixmap!($bounds.width(), $bounds.height());
        new_pixmap.draw_pixmap(
            0,
            0,
            $pixmap.as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
        new_pixmap
    }};
}
