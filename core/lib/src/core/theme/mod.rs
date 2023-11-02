use grass::OutputStyle;
use palette::{Darken, Desaturate, FromColor, Hsl, Hsluv, IntoColor, Lighten, LinSrgba, Mix, Oklab, Saturate, Srgb, Srgba, WithAlpha, WithHue};
use palette::chromatic_adaptation::AdaptInto;
use palette::color_difference::Wcag21RelativeContrast;
use palette::stimulus::{FromStimulus, IntoStimulus};
use crate::prelude::Config;

mod colors;
mod sizing;
mod fonts;

pub use colors::Color;
use crate::core::config::HexColor;
use crate::widgets::get_all_stylesheet;


#[derive(Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Auto => "auto"
        }
    }
}

fn negative_contrast(color: HexColor) -> HexColor {
    let mut color: Srgba<u8> = color.into();
    let real_color = Srgba::<f64>::from_format(color);

    if real_color.relative_luminance().luma > 0.5 {
        HexColor([0, 0, 0, 255])
    } else {
        HexColor([255, 255, 255, 255])
    }


}

fn get_color(config: &Config, color: &Color, theme_variant: &Theme) -> HexColor {
    match color {
        Color::Accent => {
            match theme_variant {
                Theme::Dark => {config.colors.accent.dark}
                Theme::Auto | Theme::Light => {config.colors.accent.light}
            }
        }
        Color::Background => {
            match theme_variant {
                Theme::Dark => {config.colors.background.dark}
                Theme::Auto | Theme::Light => {config.colors.background.light}
            }
        }
        Color::OnBackground => {
            match theme_variant {
                Theme::Dark => {
                    let base: Srgba<u8> = negative_contrast(config.colors.background.dark).into();
                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent: Srgba<u8> = config.colors.accent.dark.into();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let on_background = base_color.with_hue(accent_color.hue).saturate(0.8).darken(0.02);

                    HexColor::from(Srgb::from_linear(on_background.into_color()).with_alpha(255))
                }
                Theme::Auto | Theme::Light => {
                    let base: Srgba<u8> = negative_contrast(config.colors.background.light).into();
                    let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();
                    let accent: Srgba<u8> = config.colors.accent.light.into();
                    let accent_color: Hsluv = accent.without_alpha().into_linear::<f32>().into_color();

                    let on_background = base_color.with_hue(accent_color.hue).saturate(0.8).lighten(0.02);

                    HexColor::from(Srgb::from_linear(on_background.into_color()).with_alpha(255))
                }
            }
        }
        Color::Surface => {
            match theme_variant {
                Theme::Dark => {config.colors.surface.dark}
                Theme::Auto | Theme::Light => {
                    let mut base: Srgba<u8> = config.colors.surface.light.into();
                    let accent: Srgba<u8> = config.colors.accent.light.into();

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
                    let mut surface: Srgba<u8> = get_color(config, &Color::Surface, theme_variant).into();
                    let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.1);
                    // Converting back to non-linear sRGB with u8 components.
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut surface: Srgba<u8> = get_color(config, &Color::Surface, theme_variant).into();
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
                    let mut surface: Srgba<u8> = get_color(config, &Color::Surface, theme_variant).into();
                    let color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    // Converting back to non-linear sRGB with u8 components.
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut surface: Srgba<u8> = get_color(config, &Color::Surface, theme_variant).into();
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
                    let mut surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurface, theme_variant).into();
                    let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.darken(0.1);
                    // Converting back to non-linear sRGB with u8 components.
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurface, theme_variant).into();
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
                    let mut background: Srgba<u8> = config.colors.surface.dark.into();
                    let accent: Srgba<u8> = config.colors.accent.dark.into();
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
                    let mut background: Srgba<u8> = config.colors.surface.light.into();
                    let accent: Srgba<u8> = config.colors.accent.light.into();
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
                    let mut surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurface, theme_variant).into();
                    let color: Oklab = surface.without_alpha().into_linear::<f32>().into_color();

                    let lightened_color = color.lighten(0.06);
                    // Converting back to non-linear sRGB with u8 components.
                    surface = Srgb::from_linear(lightened_color.into_color()).with_alpha(surface.alpha);
                    HexColor::from(surface)
                }
                Theme::Auto | Theme::Light => {
                    let mut surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurface, theme_variant).into();
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
                    config.colors.success.dark
                }
                Theme::Auto | Theme::Light => {
                    config.colors.success.light
                }
            }
        }
        Color::Border => {
            let background: Srgba<u8> = get_color(config, &Color::OnBackground, theme_variant).into();
            let background_color: Hsluv = background.without_alpha().into_linear::<f32>().into_color();
            let accent: Srgba<u8> = get_color(config, &Color::Accent, theme_variant).into();
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
                    config.colors.destructive.dark
                }
                Theme::Auto | Theme::Light => {
                    config.colors.destructive.light
                }
            }
        }
        Color::DestructiveSurface => {
            let surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurfaceBright, theme_variant).into();
            let surface_color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();
            let base: Srgba<u8> = get_color(config, &Color::Destructive, theme_variant).into();
            let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();

            let success_surface = surface_color.with_hue(base_color.hue);

            HexColor::from(Srgb::from_linear(success_surface.into_color()).with_alpha(255))
        }
        Color::Warning => {
            match theme_variant {
                Theme::Dark => {
                    config.colors.warning.dark
                }
                Theme::Auto | Theme::Light => {
                    config.colors.warning.light
                }
            }
        }
        Color::WarningSurface => {
            let surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurfaceBright, theme_variant).into();
            let surface_color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();
            let base: Srgba<u8> = get_color(config, &Color::Warning, theme_variant).into();
            let base_color: Hsluv = base.without_alpha().into_linear::<f32>().into_color();

            let success_surface = surface_color.with_hue(base_color.hue);

            HexColor::from(Srgb::from_linear(success_surface.into_color()).with_alpha(255))
        }
        Color::OnAccent => {
            match theme_variant {
                Theme::Dark => {
                    negative_contrast(config.colors.accent.dark)
                }
                Theme::Auto | Theme::Light => {
                    negative_contrast(config.colors.accent.light)
                }
            }
        }

        Color::OnSurface => {
            match theme_variant {
                Theme::Dark => {
                    negative_contrast(config.colors.surface.dark)
                }
                Theme::Auto | Theme::Light => {
                    negative_contrast(config.colors.surface.light)
                }
            }
        }
        Color::SuccessSurface => {
            let surface: Srgba<u8> = get_color(config, &Color::AccentuatedSurfaceBright, theme_variant).into();
            let surface_color: Hsluv = surface.without_alpha().into_linear::<f32>().into_color();
            let success: Srgba<u8> = get_color(config, &Color::Success, theme_variant).into();
            let success_color: Hsluv = success.without_alpha().into_linear::<f32>().into_color();

            let success_surface = surface_color.with_hue(success_color.hue);

            HexColor::from(Srgb::from_linear(success_surface.into_color()).with_alpha(255))

        }
    }
}

fn palette(config: &Config, theme_variant: &Theme) -> String {

   Color::iter()
        .map(|color| {
            (color, get_color(config, color, theme_variant))
        })
        .map(|(color, hex_code)| format!("{}: {};", color.as_str(), hex_code.to_string()))
        .collect::<Vec<String>>()
        .join("")
}
fn generate_color_palette(theme_variant: Theme) -> String {
    let config = Config::load();

    match theme_variant {
        Theme::Dark | Theme::Light => {
            format!(r#"
                .app-theme--{} {{
                    {}
                }}
            "#, theme_variant.as_str(), palette(&config, &theme_variant),)
        }
        Theme::Auto => {
            format!(r#"
                .app-theme--{} {{
                    @media (prefers-color-scheme: light) {{
                      {}
                    }}
                    @media (prefers-color-scheme: dark) {{
                      {}
                    }}
                }}

            "#,
                    theme_variant.as_str(),
                    palette(&config, &Theme::Light),
                    palette(&config, &Theme::Dark),
            )
        }
    }

}

pub fn get_stylesheet() -> String {
    let palette_style = generate_color_palette(Theme::Auto);
    let widget_style=  get_all_stylesheet().join("");



    grass::from_string(
        format!(r#"
        {palette_style}

        body {{
            background: var(--background);
        }}

        {widget_style}
        "#),
        &grass::Options::default().style(OutputStyle::Compressed),
    )
        .expect("Can't compile SCSS")
}