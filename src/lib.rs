#[cfg(any(
    feature = "fontawesome",
    feature = "fontawesome-brand",
    feature = "fontawesome-regular",
    feature = "fontawesome-solid",
))]
pub mod fontawesome;
#[cfg(any(feature = "fontawesome-ext", feature = "fontawesome-ext-regular"))]
pub mod fontawesome_ext;
#[cfg(feature = "hypertext")]
pub mod hypertext;
pub mod icon;
