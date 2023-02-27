use rusttype::Font;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::utils::{make_error, AppResult};

thread_local! {
    pub static FONTS: RefCell<HashMap<String, Font<'static>>> = RefCell::new(HashMap::new());
}

pub fn init_fonts(fontset: HashMap<String, Vec<u8>>) -> AppResult {
    for (font_family, buff) in fontset {
        FONTS.with(move |fonts| -> AppResult {
            fonts.borrow_mut().insert(
                font_family,
                Font::try_from_bytes(Box::leak(Box::new(buff)))
                    .map_or(Err(make_error("invalid font!!")), |v| Ok(v))?,
            );
            Ok(())
        })?
    }
    Ok(())
}
