use hypertext::prelude::{SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements};
use hypertext::{Buffer, Renderable, context, rsx};

use crate::icon::{Icon, IconData, IconRef};

pub fn svg(icon: IconRef) -> impl Renderable {
    rsx! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox=(icon.view_box)>
            <path fill=(icon.path_fill) d=(icon.path_data)/>
        </svg>
    }
}

pub trait HypertextSvgIcon {
    fn svg() -> impl Renderable {}
}

impl<I: IconData> HypertextSvgIcon for I {
    fn svg() -> impl Renderable {
        svg(I::REF)
    }
}

impl Renderable for Icon<'_> {
    fn render_to(&self, buffer: &mut Buffer<context::Node>) {
        svg(self.data()).render_to(buffer);
    }
}

#[cfg(all(test, feature = "fontawesome-solid"))]
mod tests {
    use hypertext::RenderableExt;

    use crate::fontawesome::solid::Gear;
    use crate::icon::{Icon, IconData};

    #[test]
    fn dynamic_icon_renders_like_static() {
        let icon = Icon::from(Gear);
        assert_eq!(icon.name(), "gear");
        assert_eq!(icon.data(), Gear::REF);
        assert_eq!(icon.render().into_inner(), Gear.render().into_inner());
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
