use derive_more::From;

crate::define_icon!(
    MoonStars,
    "moon-stars",
    "0 -32 512 544",
    "M439.8 89.8l-11-38.6-38.6-11a8.52 8.52 0 0 1 0-16.4l38.6-11 11-38.6a8.52 8.52 0 0 1 16.4 0l11 38.6 38.6 11a8.52 8.52 0 0 1 0 16.4l-38.6 11-11 38.6a8.52 8.52 0 0 1-16.4 0z M371.8 326.8l-16.6-58-58-16.6a12.69 12.69 0 0 1 0-24.4l58-16.6 16.6-58a12.69 12.69 0 0 1 24.4 0l16.6 58 58 16.6a12.69 12.69 0 0 1 0 24.4l-58 16.6-16.6 58a12.69 12.69 0 0 1-24.4 0z M362.9 413.5a21 21 0 0 1 16.3 36.1a224 224 0 1 1-105.6-380.1a21 21 0 0 1 4.7 39.3a161 161 0 0 0 84.6 304.7z"
);

crate::define_icon!(
    SunBright,
    "sun-bright",
    "0 -32 576 576",
    "M264 456a24 24 0 0 1 48 0v64a24 24 0 0 1-48 0z M129.6 380.4a24 24 0 0 1 33.94 33.94l-45.25 45.25a24 24 0 0 1-33.94-33.94z M412.46 414.34a24 24 0 0 1 33.94-33.94l45.25 45.25a24 24 0 0 1-33.94 33.94z M288 144a112 112 0 1 1 0 224 112 112 0 1 1 0-224z M88 232a24 24 0 0 1 0 48H24a24 24 0 0 1 0-48z M552 232a24 24 0 0 1 0 48h-64a24 24 0 0 1 0-48z M84.36 86.34a24 24 0 0 1 33.94-33.94l45.25 45.25a24 24 0 0 1-33.94 33.94z M457.7 52.3a24 24 0 0 1 33.94 33.94l-45.25 45.25a24 24 0 0 1-33.94-33.94z M264-8a24 24 0 0 1 48 0v64a24 24 0 0 1-48 0z"
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, From)]
pub enum Icon {
    MoonStars(MoonStars),
    SunBright(SunBright),
}

#[cfg(feature = "hypertext")]
impl hypertext::Renderable for Icon {
    fn render_to(&self, buffer: &mut hypertext::Buffer<hypertext::context::Node>) {
        match self {
            Self::MoonStars(icon) => icon.render_to(buffer),
            Self::SunBright(icon) => icon.render_to(buffer),
        }
    }
}
