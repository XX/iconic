#[cfg(any(
    feature = "fontawesome",
    feature = "fontawesome-brand",
    feature = "fontawesome-regular",
    feature = "fontawesome-solid",
))]
pub mod fontawesome;
#[cfg(feature = "hypertext")]
pub mod hypertext;
pub mod icon;
