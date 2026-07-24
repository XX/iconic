pub trait IconData {
    const NAME: &'static str = "";

    const VIEW_BOX: &'static str = "0 0 640 640";

    const PATH_FILL: &'static str = "currentColor";

    const PATH_DATA: &'static str = "";
}

#[macro_export]
macro_rules! define_icon {
    ($type:ident, $name:literal, $view_box:literal, $path_data:literal) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct $type;

        impl $crate::icon::IconData for $type {
            const NAME: &'static str = $name;
            const VIEW_BOX: &'static str = $view_box;
            const PATH_DATA: &'static str = $path_data;
        }

        impl From<$type> for $crate::icon::Icon {
            fn from(icon: $type) -> Self {
                self::Icon::from(icon).into()
            }
        }

        #[cfg(feature = "hypertext")]
        $crate::hypertext_renderable!($type);
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Icon {
    #[cfg(any(feature = "fontawesome", feature = "fontawesome-brand"))]
    FontawesomeBrand(crate::fontawesome::brand::Icon),

    #[cfg(any(feature = "fontawesome", feature = "fontawesome-regular"))]
    FontawesomeRegular(crate::fontawesome::regular::Icon),

    #[cfg(any(feature = "fontawesome", feature = "fontawesome-solid"))]
    FontawesomeSolid(crate::fontawesome::solid::Icon),

    #[cfg(any(feature = "fontawesome-ext", feature = "fontawesome-ext-regular"))]
    FontawesomeExtRegular(crate::fontawesome_ext::regular::Icon),
}

#[cfg(any(feature = "fontawesome", feature = "fontawesome-brand"))]
impl From<crate::fontawesome::brand::Icon> for Icon {
    fn from(icon: crate::fontawesome::brand::Icon) -> Self {
        Self::FontawesomeBrand(icon.into())
    }
}

#[cfg(any(feature = "fontawesome", feature = "fontawesome-regular"))]
impl From<crate::fontawesome::regular::Icon> for Icon {
    fn from(icon: crate::fontawesome::regular::Icon) -> Self {
        Self::FontawesomeRegular(icon.into())
    }
}

#[cfg(any(feature = "fontawesome", feature = "fontawesome-solid"))]
impl From<crate::fontawesome::solid::Icon> for Icon {
    fn from(icon: crate::fontawesome::solid::Icon) -> Self {
        Self::FontawesomeSolid(icon.into())
    }
}

#[cfg(any(feature = "fontawesome-ext", feature = "fontawesome-ext-regular"))]
impl From<crate::fontawesome_ext::regular::Icon> for Icon {
    fn from(icon: crate::fontawesome_ext::regular::Icon) -> Self {
        Self::FontawesomeExtRegular(icon.into())
    }
}
