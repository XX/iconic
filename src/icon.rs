pub trait IconData {
    const NAME: &'static str = "";

    const VIEW_BOX: &'static str = "0 0 640 640";

    const PATH_FILL: &'static str = "currentColor";

    const PATH_DATA: &'static str = "";

    const REF: &'static IconRef = &IconRef {
        name: Self::NAME,
        view_box: Self::VIEW_BOX,
        path_fill: Self::PATH_FILL,
        path_data: Self::PATH_DATA,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IconRef {
    pub name: &'static str,
    pub view_box: &'static str,
    pub path_fill: &'static str,
    pub path_data: &'static str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Icon(&'static IconRef);

impl Icon {
    pub const fn new<I: IconData>() -> Self {
        Self(I::REF)
    }

    pub const fn data(&self) -> &'static IconRef {
        self.0
    }

    pub const fn name(&self) -> &'static str {
        self.0.name
    }

    pub const fn view_box(&self) -> &'static str {
        self.0.view_box
    }

    pub const fn path_fill(&self) -> &'static str {
        self.0.path_fill
    }

    pub const fn path_data(&self) -> &'static str {
        self.0.path_data
    }
}

impl<I: IconData> From<I> for Icon {
    fn from(_: I) -> Self {
        Self::new::<I>()
    }
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

        #[cfg(feature = "hypertext")]
        $crate::hypertext_renderable!($type);
    };
}
