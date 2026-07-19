use hypertext::prelude::{SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements};
use hypertext::{Renderable, rsx};

use crate::icon::IconData;

pub trait HypertextSvgIcon {
    fn svg() -> impl Renderable {}
}

impl<I: IconData> HypertextSvgIcon for I {
    fn svg() -> impl Renderable {
        rsx! {
            <svg xmlns="http://www.w3.org/2000/svg" viewBox=(I::VIEW_BOX)>
                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                <path fill=(I::PATH_FILL) d=(I::PATH_DATA)/>
            </svg>
        }
    }
}
