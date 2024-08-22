/// Re-export of the `palette` crate, for convenience.
pub use palette;

macro_rules! mk_theme_trait {
    ($(
        $name:ident$( = $($alias:expr),*)?;
    )*) => {
        /// The main trait of this crate.
        ///
        /// Contains methods to retrieve a variety of colours for a colour scheme.
        pub trait Theme {
            fn name(&self) -> std::borrow::Cow<str>;

            $(
                $($(
                    #[doc(alias = $alias)]
                )*)?
                fn $name(&self) -> $crate::palette::LinSrgba;
            )*

            fn all(&self) -> [$crate::palette::LinSrgba; 30] {
                [$(self.$name()),*]
            }
        }
    };
}
mk_theme_trait!(
    bg_dim;
    bg0;
    bg1;
    bg2;
    bg3;
    bg4;
    bg5;

    red_dark;
    orange_dark;
    yellow_dark;
    green_dark;
    aqua_dark = "turquoise_dark";
    blue_dark;
    purple_dark;

    red_light;
    orange_light;
    yellow_light;
    green_light;
    aqua_light = "turquoise_light";
    blue_light;
    purple_light;

    fg;

    sucess;
    neutral;
    danger;

    grey0;
    grey1;
    grey2;

    accent_light;
    accent_dark;
);

/// Easy implementation of the [`Theme`] trait.
#[macro_export]
macro_rules! impl_theme {
    ($vis:vis $theme_name:ident: $theme_name_str:expr => $(
        $color_name:ident = $color:expr;
    )*) => {
        $vis struct $theme_name;
        impl $crate::Theme for $theme_name {
            fn name(&self) -> std::borrow::Cow<str> {
                $theme_name_str.into()
            }
            $(
                fn $color_name(&self) -> $crate::palette::LinSrgba {
                    $crate::palette::LinSrgba::<f32>::from($crate::palette::LinSrgba::from($color))
                }
            )*
        }
    };
}

/// Prebuilt themes.
pub mod themes {
    impl_theme!(pub EverforestDarkHard: "Everforest Dark Hard" =>
        bg_dim = 0x1e2326ff;
        bg0 = 0x272e33ff;
        bg1 = 0x2e383cff;
        bg2 = 0x374145ff;
        bg3 = 0x414b50ff;
        bg4 = 0x495156ff;
        bg5 = 0x4f5b58ff;

        red_light = 0xe67e80ff;
        red_dark = 0x4c3743ff;
        orange_light = 0xe69875ff;
        orange_dark = 0x493b40ff;
        yellow_light = 0xdbbc7fff;
        yellow_dark = 0x45443cff;
        green_light = 0xa7c080ff;
        green_dark = 0x3c4841ff;
        aqua_light = 0x83c092ff;
        aqua_dark = 0x39453dff;
        blue_light = 0x7fbbb3ff;
        blue_dark = 0x384b55ff;
        purple_light = 0xd699b6ff;
        purple_dark = 0x5e4b54ff;

        fg = 0xd3c6aaff;

        accent_light = 0x7fbbb3ff;
        accent_dark = 0x384b55ff;

        sucess = 0xa7c080ff;
        neutral = 0xd3c6aaff;
        danger = 0xe67e80ff;

        grey0 = 0x7a8478ff;
        grey1 = 0x859289ff;
        grey2 = 0x9da9a0ff;
    );
}
