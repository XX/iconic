use crate::icon::IconData;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ChevronDown;

impl IconData for ChevronDown {
    const NAME: &'static str = "chevron-down";
    const VIEW_BOX: &'static str = "0 0 448 512";
    const PATH_DATA: &'static str =
        "M207.5 409a24 24 0 0 0 33.9 0l200-200a24 24 0 0 0-33.94-33.94l-183 183-183-183a24 24 0 0 0-33.94 33.94z";
}

#[cfg(feature = "hypertext")]
crate::hypertext_renderable!(ChevronDown);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct GripLinesVertical;

impl IconData for GripLinesVertical {
    const NAME: &'static str = "grip-lines-vertical";
    const VIEW_BOX: &'static str = "0 0 192 512";
    const PATH_DATA: &'static str =
        "M0 56a24 24 0 0 1 48 0v400a24 24 0 0 1-48 0zM144 56a24 24 0 0 1 48 0v400a24 24 0 0 1-48 0z";
}

#[cfg(feature = "hypertext")]
crate::hypertext_renderable!(GripLinesVertical);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Hashtag;

impl IconData for Hashtag {
    const NAME: &'static str = "hashtag";
    const VIEW_BOX: &'static str = "0 0 448 512";
    const PATH_DATA: &'static str = "M226.9 29a24 24 0 0 0-46.9-10.1L156.6 128H68.5a24 24 0 0 0 0 48h77.8l-34.3 160H24a24 24 0 0 0 0 48h77.7l-21.2 99a24 24 0 0 0 46.9 10.1l23.4-109h155.4l-21.2 99a24 24 0 0 0 46.9 10.1l23.4-109h88a24 24 0 0 0 0-48h-77.7l34.3-160h88.1a24 24 0 0 0 0-48h-77.8l21.2-99a24 24 0 0 0-46.9-10.1L361 128H205.6z M195.4 176h155.4l-34.3 160H161.1z";
}

#[cfg(feature = "hypertext")]
crate::hypertext_renderable!(Hashtag);
