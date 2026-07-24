use derive_more::From;

crate::define_icon!(
    ChevronDown,
    "chevron-down",
    "0 0 448 512",
    "M207.5 409a24 24 0 0 0 33.9 0l200-200a24 24 0 0 0-33.94-33.94l-183 183-183-183a24 24 0 0 0-33.94 33.94z"
);

crate::define_icon!(
    GripLinesVertical,
    "grip-lines-vertical",
    "0 0 192 512",
    "M0 56a24 24 0 0 1 48 0v400a24 24 0 0 1-48 0zM144 56a24 24 0 0 1 48 0v400a24 24 0 0 1-48 0z"
);

crate::define_icon!(
    Hashtag,
    "hashtag",
    "0 0 448 512",
    "M226.9 29a24 24 0 0 0-46.9-10.1L156.6 128H68.5a24 24 0 0 0 0 48h77.8l-34.3 160H24a24 24 0 0 0 0 48h77.7l-21.2 99a24 24 0 0 0 46.9 10.1l23.4-109h155.4l-21.2 99a24 24 0 0 0 46.9 10.1l23.4-109h88a24 24 0 0 0 0-48h-77.7l34.3-160h88.1a24 24 0 0 0 0-48h-77.8l21.2-99a24 24 0 0 0-46.9-10.1L361 128H205.6z M195.4 176h155.4l-34.3 160H161.1z"
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, From)]
pub enum Icon {
    ChevronDown(ChevronDown),
    GripLinesVertical(GripLinesVertical),
    Hashtag(Hashtag),
}

#[cfg(feature = "hypertext")]
impl hypertext::Renderable for Icon {
    fn render_to(&self, buffer: &mut hypertext::Buffer<hypertext::context::Node>) {
        match self {
            Self::ChevronDown(icon) => icon.render_to(buffer),
            Self::GripLinesVertical(icon) => icon.render_to(buffer),
            Self::Hashtag(icon) => icon.render_to(buffer),
        }
    }
}
