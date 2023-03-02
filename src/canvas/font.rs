use rusttype::Font;
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::utils::{make_error, AppResult};

use once_cell::sync::Lazy;

pub static mut FONTS: Lazy<HashMap<String, Font<'static>>> = Lazy::new(|| HashMap::new());
pub static mut DEFAULT_FONT: Lazy<Option<&Font<'static>>> = Lazy::new(|| {
    #[cfg(feature = "default-font")]
    return Some(Box::leak(Box::new(init_default_font())));
    None
});

#[cfg(feature = "default-font")]
fn init_default_font() -> Font<'static> {
    let FONT_DATA: &[u8] = include_bytes!("../../assets/default.ttf");
    Font::try_from_bytes(FONT_DATA).unwrap()
}

pub fn init_fonts(fontset: HashMap<String, Vec<u8>>) -> AppResult {
    for (font_family, buff) in fontset {
        unsafe {
            FONTS.insert(
                font_family,
                Font::try_from_bytes(Box::leak(Box::new(buff))).unwrap(),
            );
        }
    }
    #[cfg(feature = "default-font")]
    init_default_font();
    Ok(())
}

pub fn get_font(font: &str) -> Option<&'static Font> {
    unsafe { FONTS.get(font).or(**DEFAULT_FONT.borrow()) }
}
