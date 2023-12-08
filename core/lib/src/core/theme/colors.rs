use std::slice::Iter;
use palette::{Darken, Desaturate, Hsluv, IntoColor, Lighten, Oklab, Saturate, Srgb, Srgba, WithAlpha, WithHue};
use palette::color_difference::Wcag21RelativeContrast;
use strum::EnumIter;
use crate::CONFIG;
use crate::core::config::HexColor;
use crate::core::theme::negative_contrast;
use crate::prelude::Theme;

#[derive(EnumIter, Copy, Clone)]
pub enum Color {
    Accent,
    OnAccent,
    Background,
    OnBackground,
    SurfaceDim,
    Surface,
    OnSurface,
    SurfaceBright,
    AccentuatedSurfaceDim,
    AccentuatedSurface,
    AccentuatedSurfaceBright,
    Border,
    Success,
    SuccessSurface,
    Destructive,
    OnDestructive,
    DestructiveSurface,
    Warning,
    WarningSurface,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Accent => "--accent",
            Color::OnAccent => "--on-accent",
            Color::Background => "--background",
            Color::OnBackground => "--on-background",
            Color::SurfaceDim => "--surface-dim",
            Color::Surface => "--surface",
            Color::SurfaceBright => "--surface-bright",
            Color::OnSurface => "--on-surface",
            Color::AccentuatedSurfaceDim => "--accentuated-surface-dim",
            Color::AccentuatedSurface => "--accentuated-surface",
            Color::AccentuatedSurfaceBright => "--accentuated-surface-bright",
            Color::Border => "--border",
            Color::Success => "--success",
            Color::SuccessSurface => "--success-surface",
            Color::Destructive => "--destructive",
            Color::DestructiveSurface => "--destructive-surface",
            Color::Warning => "--warning",
            Color::WarningSurface => "--warning-surface",
            Color::OnDestructive => "--on-destructive"
        }
    }

    pub fn get_hex_color(&self, theme_variant: &Theme) -> HexColor {

        match self {
            Color::Accent => {
                match theme_variant {
                    Theme::Dark => {CONFIG.colors.accent.dark}
                    Theme::Auto | Theme::Light => {CONFIG.colors.accent.light}
                }
            }
            Color::Background => {
                match theme_variant {
                    Theme::Dark => {CONFIG.colors.background.dark}
                    Theme::Auto | Theme::Light => {CONFIG.colors.background.light}
                }
            }
            Color::OnBackground => {
                match theme_variant {
                    Theme::Dark => {
                        let base: Srgba<u8> = negative_contrast(CONFIG.colors.background.dark).into();
                        let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                        let accent: Srgba<u8> = CONFIG.colors.accent.dark.into();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                        let on_background = base_color.with_hue(accent_color.hue).saturate(0.8).darken(0.02);

                        HexColor::from(Srgb::from_linear(on_background.into_color()).with_alpha(255))
                    }
                    Theme::Auto | Theme::Light => {
                        let base: Srgba<u8> = negative_contrast(CONFIG.colors.background.light).into();
                        let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                        let accent: Srgba<u8> = CONFIG.colors.accent.light.into();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                        let on_background = base_color.with_hue(accent_color.hue).saturate(0.8).lighten(0.02);

                        HexColor::from(Srgb::from_linear(on_background.into_color()).with_alpha(255))
                    }
                }
            }
            Color::Surface => {
                match theme_variant {
                    Theme::Dark => {CONFIG.colors.surface.dark}
                    Theme::Auto | Theme::Light => {
                        let mut base: Srgba<u8> = CONFIG.colors.surface.light.into();
                        let accent: Srgba<u8> = CONFIG.colors.accent.light.into();

                        let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                        let accentuated_surface = base_color.with_hue(accent_color.hue).saturate(0.1);

                        HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))

                    }
                }
            }
            Color::SurfaceDim => {
                match theme_variant {
                    Theme::Dark => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                        let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.darken(0.1);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                    Theme::Auto | Theme::Light => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                        let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.darken(0.06);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                }
            }
            Color::SurfaceBright => {
                match theme_variant {
                    Theme::Dark => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                        let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.lighten(0.06);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                    Theme::Auto | Theme::Light => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                        let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.lighten(0.5);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                }
            }
            Color::AccentuatedSurfaceDim => {
                match theme_variant {
                    Theme::Dark => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                        let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.darken(0.1);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                    Theme::Auto | Theme::Light => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                        let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.darken(0.06);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                }
            }
            Color::AccentuatedSurface => {
                match theme_variant {
                    Theme::Dark => {
                        let mut background: Srgba<u8> = CONFIG.colors.surface.dark.into();
                        let accent: Srgba<u8> = CONFIG.colors.accent.dark.into();
                        let background_color: Hsluv = background.without_alpha().into_linear::<f32>().into_color();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();
                        let luminance = Srgba::<f64>::from_format(accent).relative_luminance();
                        let accentuated_surface = if luminance.luma == 1.0 {
                            background_color.lighten(0.1)
                        } else {
                            background_color.with_hue(accent_color.hue).saturate(1.0).lighten(0.1)
                        };


                        HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))
                    }
                    Theme::Auto | Theme::Light => {
                        let mut background: Srgba<u8> = CONFIG.colors.surface.light.into();
                        let accent: Srgba<u8> = CONFIG.colors.accent.light.into();
                        let background_color: Hsluv = background.without_alpha().into_linear::<f32>().into_color();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();
                        let luminance = Srgba::<f64>::from_format(accent).relative_luminance();
                        let accentuated_surface = if luminance.luma == 0.0 {
                            background_color.darken(0.05)
                        } else {
                            background_color.with_hue(accent_color.hue).saturate(1.0).darken(0.05)
                        };

                        HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))
                    }
                }
            }
            Color::AccentuatedSurfaceBright => {
                match theme_variant {
                    Theme::Dark => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                        let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.lighten(0.06);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                    Theme::Auto | Theme::Light => {
                        let mut surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                        let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                        let lightened_color = color.lighten(0.5);
                        // Converting back to non-linear sRGB with u8 components.
                        surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                        HexColor::from(surface)
                    }
                }
            }
            Color::Success => {
                match theme_variant {
                    Theme::Dark => {
                        CONFIG.colors.success.dark
                    }
                    Theme::Auto | Theme::Light => {
                        CONFIG.colors.success.light
                    }
                }
            }
            Color::Border => {
                let background: Srgba<u8> = Color::get_hex_color(&Color::OnBackground, theme_variant).into();
                let background_color: Hsluv = background.without_alpha().into_linear::<f32>().into_color();
                let accent: Srgba<u8> = Color::get_hex_color(&Color::Accent, theme_variant).into();
                let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();
                let border = match theme_variant {
                    Theme::Dark => {
                        background_color.with_hue(accent_color.hue).darken(0.8).desaturate(0.9)
                    }
                    Theme::Auto | Theme::Light => {
                        background_color.with_hue(accent_color.hue).lighten(0.85).desaturate(0.9)
                    }
                };

                HexColor::from(Srgb::from_linear(border.into_color()).with_alpha(255))
            }
            Color::Destructive => {
                match theme_variant {
                    Theme::Dark => {
                        CONFIG.colors.destructive.dark
                    }
                    Theme::Auto | Theme::Light => {
                        CONFIG.colors.destructive.light
                    }
                }
            }
            Color::DestructiveSurface => {
                let surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurfaceBright, theme_variant).into();
                let surface_color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();
                let base: Srgba<u8> = Color::get_hex_color(&Color::Destructive, theme_variant).into();
                let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();

                let success_surface = surface_color.with_hue(base_color.hue);

                HexColor::from(Srgb::from_linear(success_surface.into_color()).with_alpha(255))
            }
            Color::Warning => {
                match theme_variant {
                    Theme::Dark => {
                        CONFIG.colors.warning.dark
                    }
                    Theme::Auto | Theme::Light => {
                        CONFIG.colors.warning.light
                    }
                }
            }
            Color::WarningSurface => {
                let surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurfaceBright, theme_variant).into();
                let surface_color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();
                let base: Srgba<u8> = Color::get_hex_color(&Color::Warning, theme_variant).into();
                let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();

                let success_surface = surface_color.with_hue(base_color.hue);

                HexColor::from(Srgb::from_linear(success_surface.into_color()).with_alpha(255))
            }
            Color::OnAccent => {
                match theme_variant {
                    Theme::Dark => {
                        negative_contrast(CONFIG.colors.accent.dark)
                    }
                    Theme::Auto | Theme::Light => {
                        negative_contrast(CONFIG.colors.accent.light)
                    }
                }
            }

            Color::OnSurface => {
                match theme_variant {
                    Theme::Dark => {
                        negative_contrast(CONFIG.colors.surface.dark)
                    }
                    Theme::Auto | Theme::Light => {
                        negative_contrast(CONFIG.colors.surface.light)
                    }
                }
            }
            Color::SuccessSurface => {
                let surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurfaceBright, theme_variant).into();
                let surface_color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();
                let success: Srgba<u8> = Color::get_hex_color(&Color::Success, theme_variant).into();
                let success_color: Hsluv = success.without_alpha().into_linear::<f32>().into_color();

                let success_surface = surface_color.with_hue(success_color.hue);

                HexColor::from(Srgb::from_linear(success_surface.into_color()).with_alpha(255))

            }
            Color::OnDestructive => {
                match theme_variant {
                    Theme::Dark => {
                        let base: Srgba<u8> = negative_contrast(CONFIG.colors.destructive.dark).into();
                        let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                        let accent: Srgba<u8> = CONFIG.colors.destructive.dark.into();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                        let on_background = base_color.with_hue(accent_color.hue).saturate(0.8).darken(0.02);

                        HexColor::from(Srgb::from_linear(on_background.into_color()).with_alpha(255))
                    }
                    Theme::Auto | Theme::Light => {
                        let base: Srgba<u8> = negative_contrast(CONFIG.colors.destructive.light).into();
                        let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                        let accent: Srgba<u8> = CONFIG.colors.destructive.light.into();
                        let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                        let on_background = base_color.with_hue(accent_color.hue).saturate(0.8).lighten(0.02);

                        HexColor::from(Srgb::from_linear(on_background.into_color()).with_alpha(255))
                    }
                }
            }
        }
    }
}
