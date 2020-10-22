/*!

This crate provides the default theme resources of OrbTks default theme dark and light.
It provides fonts, icons and colors.

 */

use orbtk_theming::{config::ThemeConfig, prelude::*};

/// provides `constants` to reference colors.
pub mod colors;
/// provides `constants` associated to fonts.
pub mod fonts;
pub mod prelude;
/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod vector_graphics;

/// Resource file of default theme
pub const THEME_DEFAULT: &str = include_str!("../theme/theme_default.ron");

/// The default dark theme colors resource file.
pub const THEME_DEFAULT_COLORS_DARK: &str = include_str!("../theme/theme_default_colors_dark.ron");

/// The default light theme colors resource file.
pub const THEME_DEFAULT_COLORS_LIGHT: &str =
    include_str!("../theme/theme_default_colors_light.ron");

/// The font resources of the default theme
pub const THEME_DEFAULT_FONTS: &str = include_str!("../theme/theme_default_fonts.ron");

/// Returns the default OrbTk theme.
pub fn theme_default() -> Theme {
    theme_default_dark()
}

/// Creates OrbTks default dark theme.
pub fn theme_default_dark() -> Theme {
    register_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

/// Creates OrbTks default light theme.
pub fn theme_default_light() -> Theme {
    register_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn register_fonts(theme: Theme) -> Theme {
    theme
        .register_font("Roboto-Regular", crate::fonts::ROBOTO_REGULAR_FONT)
        .register_font("Roboto-Medium", crate::fonts::ROBOTO_MEDIUM_FONT)
        .register_font("MaterialIcons-Regular", crate::fonts::MATERIAL_ICONS_FONT)
}

#[cfg(target_arch = "wasm32")]
fn register_fonts(theme: Theme) -> Theme {
    theme
}
