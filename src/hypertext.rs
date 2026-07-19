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
                <path fill=(I::PATH_FILL) d=(I::PATH_DATA)/>
            </svg>
        }
    }
}

#[macro_export]
macro_rules! hypertext_renderable {
    ($type_name:ty) => {
        impl ::hypertext::Renderable for $type_name {
            fn render_to(&self, buffer: &mut ::hypertext::Buffer<::hypertext::context::Node>) {
                use $crate::hypertext::HypertextSvgIcon;
                <$type_name>::svg().render_to(buffer);
            }
        }
    };
}
