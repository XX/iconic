use crate::icon::IconData;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Bookmark;

impl IconData for Bookmark {
    const NAME: &'static str = "bookmark";
    const VIEW_BOX: &'static str = "0 0 384 512";
    const PATH_DATA: &'static str = "M0 64C0 28.7 28.7 0 64 0L320 0c35.3 0 64 28.7 64 64l0 417.1c0 25.6-28.5 40.8-49.8 26.6L192 412.8 49.8 507.7C28.5 521.9 0 506.6 0 481.1L0 64zM64 48c-8.8 0-16 7.2-16 16l0 387.2 117.4-78.2c16.1-10.7 37.1-10.7 53.2 0L336 451.2 336 64c0-8.8-7.2-16-16-16L64 48z";
}

#[cfg(feature = "hypertext")]
crate::hypertext_renderable!(Bookmark);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct CircleCheck;

impl IconData for CircleCheck {
    const NAME: &'static str = "circle-check";
    const VIEW_BOX: &'static str = "0 0 512 512";
    const PATH_DATA: &'static str = "M256 512a256 256 0 1 1 0-512 256 256 0 1 1 0 512zm0-464a208 208 0 1 0 0 416 208 208 0 1 0 0-416zm70.7 121.9c7.8-10.7 22.8-13.1 33.5-5.3 10.7 7.8 13.1 22.8 5.3 33.5L243.4 366.1c-4.1 5.7-10.5 9.3-17.5 9.8-7 .5-13.9-2-18.8-6.9l-55.9-55.9c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l36 36 105.6-145.2z";
}

#[cfg(feature = "hypertext")]
crate::hypertext_renderable!(CircleCheck);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct XmarkCircle;

impl IconData for XmarkCircle {
    const NAME: &'static str = "xmark-circle";
    const VIEW_BOX: &'static str = "0 0 512 512";
    const PATH_DATA: &'static str = "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464a256 256 0 1 0 0-512 256 256 0 1 0 0 512zM167 167c-9.4 9.4-9.4 24.6 0 33.9l55 55-55 55c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l55-55 55 55c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-55-55 55-55c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-55 55-55-55c-9.4-9.4-24.6-9.4-33.9 0z";
}
