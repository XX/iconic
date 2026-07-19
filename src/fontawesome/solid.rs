use crate::icon::IconData;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Xmark;

impl IconData for Xmark {
    const NAME: &'static str = "xmark";
    const VIEW_BOX: &'static str = "0 0 384 512";
    const PATH_DATA: &'static str = "M55.1 73.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L147.2 256 9.9 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192.5 301.3 329.9 438.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.8 256 375.1 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192.5 210.7 55.1 73.4z";
}

#[cfg(feature = "hypertext")]
crate::hypertext_renderable!(Xmark);
