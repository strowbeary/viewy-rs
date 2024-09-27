use std::slice::Iter;
use palette::{Darken, Desaturate, Hsl, Hsluv, IntoColor, Lighten, Oklab, RelativeContrast, Saturate, Srgb, Srgba, WithAlpha, WithHue};
use palette::color_difference::Wcag21RelativeContrast;
use strum::EnumIter;
use crate::CONFIG;
use crate::core::config::HexColor;
use crate::core::theme::negative_contrast;
use crate::prelude::Theme;

/// Describe UI colors
#[derive(EnumIter, Copy, Clone)]
pub enum Color {
    Accent,
    OnAccent,
    Background,
    OnBackground,
    SurfaceDim,
    Surface,
    SurfaceBright,
    OnSurface,
    AccentuatedSurfaceDim,
    AccentuatedSurface,
    AccentuatedSurfaceBright,
    Border,
    Success,
    SuccessSurface,
    Destructive,
    OnDestructive,
    DestructiveSurfaceDim,
    DestructiveSurface,
    DestructiveSurfaceBright,
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
            Color::OnDestructive => "--on-destructive",
            Color::DestructiveSurfaceDim => "--destructive-surface-dim",
            Color::DestructiveSurface => "--destructive-surface",
            Color::DestructiveSurfaceBright => "--destructive-surface-bright",
            Color::Warning => "--warning",
            Color::WarningSurface => "--warning-surface",
        }
    }

    pub fn get_hex_color(&self, theme_variant: &Theme) -> HexColor {
        match self {
            Color::Accent => match theme_variant {
                Theme::Dark => CONFIG.colors.accent.dark,
                Theme::Auto | Theme::Light => CONFIG.colors.accent.light,
            },
            Color::OnAccent => {
                match theme_variant {
                    Theme::Dark => {
                        println!("OnAccent dark");
                        negative_contrast(CONFIG.colors.accent.dark)
                    }
                    Theme::Auto | Theme::Light => {
                        println!("OnAccent Light");
                        negative_contrast(CONFIG.colors.accent.light)
                    }
                }
            },
            Color::Background => match theme_variant {
                Theme::Dark => CONFIG.colors.background.dark,
                Theme::Auto | Theme::Light => CONFIG.colors.background.light,
            },
            Color::OnBackground => match theme_variant {
                Theme::Dark => {
                    negative_contrast(CONFIG.colors.background.dark)
                }
                Theme::Auto | Theme::Light => {
                    negative_contrast(CONFIG.colors.background.light)
                }
            },
            Color::SurfaceDim => match theme_variant {
                Theme::Dark => {
                    let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                    let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.3);
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                    let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.06);
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
            },
            Color::Surface => match theme_variant {
                Theme::Dark => {
                    let base: Srgba<u8> = CONFIG.colors.surface.dark.into();
                    let accent: Srgba<u8> = CONFIG.colors.accent.dark.into();

                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let mut accentuated_surface = base_color.with_hue(accent_color.hue).saturate(0.1);

                    HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))
                }
                Theme::Auto | Theme::Light => {
                    let base: Srgba<u8> = CONFIG.colors.surface.light.into();
                    let accent: Srgba<u8> = CONFIG.colors.accent.light.into();

                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let mut accentuated_surface = base_color.with_hue(accent_color.hue).saturate(0.1);


                    HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))
                }
            },
            Color::SurfaceBright => match theme_variant {
                Theme::Dark => {
                    let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                    let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut surface: Srgba<u8> = Color::get_hex_color(&Color::Surface, theme_variant).into();
                    let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
            },
            Color::OnSurface => match theme_variant {
                Theme::Dark => {
                    let base: Srgba<u8> = negative_contrast(CONFIG.colors.surface.dark).into();
                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent: Srgba<u8> = CONFIG.colors.accent.dark.into();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let mut on_surface = base_color.with_hue(accent_color.hue).saturate(0.8).darken(0.02);
                    if !is_contrast_sufficient(CONFIG.colors.surface.dark, on_surface, 4.5) {
                        on_surface = adjust_contrast(on_surface, true);
                    }

                    HexColor::from(Srgb::from_linear(on_surface.into_color()).with_alpha(255))
                }
                Theme::Auto | Theme::Light => {
                    let base: Srgba<u8> = negative_contrast(CONFIG.colors.surface.light).into();
                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent: Srgba<u8> = CONFIG.colors.accent.light.into();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let mut on_surface = base_color.with_hue(accent_color.hue).saturate(0.8).lighten(0.02);
                    if !is_contrast_sufficient(CONFIG.colors.surface.light, on_surface, 4.5) {
                        on_surface = adjust_contrast(on_surface, false);
                    }

                    HexColor::from(Srgb::from_linear(on_surface.into_color()).with_alpha(255))
                }
            },
            Color::AccentuatedSurfaceDim => match theme_variant {
                Theme::Dark => {
                    let mut accentuated_surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                    let color: Hsluv = accentuated_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.3);
                    accentuated_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(accentuated_surface.alpha);
                    HexColor::from(accentuated_surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut accentuated_surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                    let color: Hsluv = accentuated_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.06);
                    accentuated_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(accentuated_surface.alpha);
                    HexColor::from(accentuated_surface)
                }
            },
            Color::AccentuatedSurface => match theme_variant {
                Theme::Dark => {
                    let base: Srgba<u8> = CONFIG.colors.surface.dark.into();
                    let accent: Srgba<u8> = CONFIG.colors.accent.dark.into();

                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let mut accentuated_surface = base_color.with_hue(accent_color.hue).saturate(0.5);

                    HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))
                }
                Theme::Auto | Theme::Light => {
                    let base: Srgba<u8> = CONFIG.colors.surface.light.into();
                    let accent: Srgba<u8> = CONFIG.colors.accent.light.into();

                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let mut accentuated_surface = base_color.with_hue(accent_color.hue).saturate(0.5);


                    HexColor::from(Srgb::from_linear(accentuated_surface.into_color()).with_alpha(255))
                }
            },
            Color::AccentuatedSurfaceBright => match theme_variant {
                Theme::Dark => {
                    let mut accentuated_surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                    let color: Hsluv = accentuated_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    accentuated_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(accentuated_surface.alpha);
                    HexColor::from(accentuated_surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut accentuated_surface: Srgba<u8> = Color::get_hex_color(&Color::AccentuatedSurface, theme_variant).into();
                    let color: Hsluv = accentuated_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    accentuated_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(accentuated_surface.alpha);
                    HexColor::from(accentuated_surface)
                }
            },
            Color::Border => {
                let background: Srgba<u8> = Color::get_hex_color(&Color::OnBackground, theme_variant).into();
                let background_color: Hsluv = background.without_alpha().into_linear::<f32>().into_color();
                let accent: Srgba<u8> = Color::get_hex_color(&Color::Accent, theme_variant).into();
                let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();
                let border = match theme_variant {
                    Theme::Dark => {
                        background_color.with_hue(accent_color.hue).darken(0.7).desaturate(0.9)
                    }
                    Theme::Auto | Theme::Light => {
                        background_color.with_hue(accent_color.hue).lighten(0.85).desaturate(0.9)
                    }
                };

                HexColor::from(Srgb::from_linear(border.into_color()).with_alpha(255))
            },
            Color::Success => match theme_variant {
                Theme::Dark => CONFIG.colors.success.dark,
                Theme::Auto | Theme::Light => CONFIG.colors.success.light,
            },
            Color::SuccessSurface => match theme_variant {
                Theme::Dark => {
                    let mut success_surface: Srgba<u8> = Color::get_hex_color(&Color::Success, theme_variant).into();
                    let color: Hsluv = success_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.15);
                    success_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(success_surface.alpha);
                    HexColor::from(success_surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut success_surface: Srgba<u8> = Color::get_hex_color(&Color::Success, theme_variant).into();
                    let color: Hsluv = success_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.03);
                    success_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(success_surface.alpha);
                    HexColor::from(success_surface)
                }
            },
            Color::Destructive => match theme_variant {
                Theme::Dark => CONFIG.colors.destructive.dark,
                Theme::Auto | Theme::Light => CONFIG.colors.destructive.light,
            },
            Color::OnDestructive => match theme_variant {
                Theme::Dark => {
                    negative_contrast(CONFIG.colors.destructive.dark)
                }
                Theme::Auto | Theme::Light => {
                    negative_contrast(CONFIG.colors.destructive.light)
                }
            },
            Color::DestructiveSurfaceDim => match theme_variant {
                Theme::Dark => {
                    let mut destructive_surface: Srgba<u8> = Color::get_hex_color(&Color::DestructiveSurface, theme_variant).into();
                    let color: Hsluv = destructive_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.3);
                    destructive_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(destructive_surface.alpha);
                    HexColor::from(destructive_surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut destructive_surface: Srgba<u8> = Color::get_hex_color(&Color::DestructiveSurface, theme_variant).into();
                    let color: Hsluv = destructive_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.06);
                    destructive_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(destructive_surface.alpha);
                    HexColor::from(destructive_surface)
                }
            },
            Color::DestructiveSurface => match theme_variant {
                Theme::Dark => {
                    let base: Srgba<u8> = CONFIG.colors.surface.dark.into();
                    let destructive: Srgba<u8> = CONFIG.colors.destructive.dark.into();

                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let destructive_color: Hsluv = destructive.without_alpha().into_linear::<f32>().into_color();

                    let mut destructive_surface = base_color.with_hue(destructive_color.hue).saturate(0.7);


                    HexColor::from(Srgb::from_linear(destructive_surface.into_color()).with_alpha(255))
                }
                Theme::Auto | Theme::Light => {
                    let base: Srgba<u8> = CONFIG.colors.surface.light.into();
                    let destructive: Srgba<u8> = CONFIG.colors.destructive.light.into();

                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let destructive_color: Hsluv = destructive.without_alpha().into_linear::<f32>().into_color();

                    let mut destructive_surface = base_color.with_hue(destructive_color.hue).saturate(0.7);


                    HexColor::from(Srgb::from_linear(destructive_surface.into_color()).with_alpha(255))
                }
            },
            Color::DestructiveSurfaceBright => match theme_variant {
                Theme::Dark => {
                    let mut destructive_surface: Srgba<u8> = Color::get_hex_color(&Color::DestructiveSurface, theme_variant).into();
                    let color: Hsluv = destructive_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    destructive_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(destructive_surface.alpha);
                    HexColor::from(destructive_surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut destructive_surface: Srgba<u8> = Color::get_hex_color(&Color::DestructiveSurface, theme_variant).into();
                    let color: Hsluv = destructive_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    destructive_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(destructive_surface.alpha);
                    HexColor::from(destructive_surface)
                }
            },
            Color::Warning => match theme_variant {
                Theme::Dark => CONFIG.colors.warning.dark,
                Theme::Auto | Theme::Light => CONFIG.colors.warning.light,
            },
            Color::WarningSurface => match theme_variant {
                Theme::Dark => {
                    let mut warning_surface: Srgba<u8> = Color::get_hex_color(&Color::Warning, theme_variant).into();
                    let color: Hsluv = warning_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.15);
                    warning_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(warning_surface.alpha);
                    HexColor::from(warning_surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut warning_surface: Srgba<u8> = Color::get_hex_color(&Color::Warning, theme_variant).into();
                    let color: Hsluv = warning_surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.03);
                    warning_surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(warning_surface.alpha);
                    HexColor::from(warning_surface)
                }
            },
        }
    }
}

/// Function to check if contrast is sufficient according to WCAG 2.1
fn is_contrast_sufficient(background: HexColor, foreground: Hsluv, mcr: f32) -> bool {
    let background_color: Srgba<f32> = background.into();
    let foreground_color: Srgba<f32> = Srgb::from_linear(foreground.into_color()).with_alpha(1.0);
    let contrast = background_color.relative_contrast(*foreground_color);
    contrast >= mcr // Minimum contrast ratio for normal text
}

/// Function to adjust color to meet contrast requirements
fn adjust_contrast(color: Hsluv, dark_mode: bool) -> Hsluv {
    if dark_mode {
        color.lighten(0.4)
    } else {
        color.darken(0.4)
    }
}
