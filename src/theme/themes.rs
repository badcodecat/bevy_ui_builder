// File Containing default themes
use once_cell::sync::Lazy;
use super::*;
pub static LIGHT: Lazy<ThemeData> = Lazy::new
(
	||
	ThemeData
	{
		base: Color::WHITE.darken(0.33),
		base_foreground: Color::BLACK,
		base_container: Color::WHITE,
		base_container_foreground: Color::BLACK,

		primary: colours::ENGLISH_VIOLET.lighten(0.50),
		primary_foreground: *colours::ENGLISH_VIOLET,
		primary_container: colours::ENGLISH_VIOLET.lighten(0.33),
		primary_container_foreground: colours::ENGLISH_VIOLET.lighten(0.9),

		secondary: colours::RAISIN_BLACK.lighten(0.50),
		secondary_foreground: *colours::RAISIN_BLACK,
		secondary_container: colours::RAISIN_BLACK.lighten(0.33),
		secondary_container_foreground: colours::RAISIN_BLACK.lighten(0.9),

		tertiary: colours::WALNUT_BROWN.lighten(0.50),
		tertiary_foreground: *colours::WALNUT_BROWN,
		tertiary_container: colours::WALNUT_BROWN.lighten(0.20),
		tertiary_container_foreground: colours::WALNUT_BROWN.lighten(0.9),

		disabled: Color::GRAY.darken(0.33),
		disabled_foreground: Color::WHITE,

		destructive: Color::RED.darken(0.33),
		destructive_foreground: Color::WHITE,

		default_font: None,
	}
);

pub static DARK: Lazy<ThemeData> = Lazy::new
(
	||
	ThemeData
	{
		base: Color::BLACK.lighten(0.33),
		base_foreground: Color::WHITE,
		base_container: Color::BLACK,
		base_container_foreground: Color::WHITE,

		primary: colours::ENGLISH_VIOLET.darken(0.20),
		primary_foreground: colours::ENGLISH_VIOLET.lighten(0.75),
		primary_container: *colours::ENGLISH_VIOLET,
		primary_container_foreground: colours::ENGLISH_VIOLET.lighten(0.9),

		secondary: *colours::RAISIN_BLACK,
		secondary_foreground: colours::RAISIN_BLACK.lighten(0.75),
		secondary_container: colours::RAISIN_BLACK.lighten(0.33),
		secondary_container_foreground: colours::RAISIN_BLACK.lighten(0.9),

		tertiary: colours::WALNUT_BROWN.darken(0.33),
		tertiary_foreground: colours::WALNUT_BROWN.lighten(0.75),
		tertiary_container: *colours::WALNUT_BROWN,
		tertiary_container_foreground: colours::WALNUT_BROWN.lighten(0.9),

		disabled: Color::GRAY.darken(0.33),
		disabled_foreground: Color::WHITE,

		destructive: Color::RED.darken(0.33),
		destructive_foreground: Color::WHITE,

		default_font: None,
	}
);

pub const DEFAULT_TRANSPARENT_OPACITY: f32 = 0.85;

pub static TRANSPARENT: Lazy<ThemeData> = Lazy::new
(
	||
	ThemeData
	{
		base: Color::BLACK.lighten(0.33),
		base_foreground: Color::WHITE,
		base_container: Color::BLACK.with_a(0.0),
		base_container_foreground: Color::WHITE,

		primary: colours::ENGLISH_VIOLET.darken(0.20),
		primary_foreground: colours::ENGLISH_VIOLET.lighten(0.75),
		primary_container: colours::ENGLISH_VIOLET.with_a(DEFAULT_TRANSPARENT_OPACITY),
		primary_container_foreground: colours::ENGLISH_VIOLET.lighten(0.9),

		secondary: *colours::RAISIN_BLACK,
		secondary_foreground: colours::RAISIN_BLACK.lighten(0.75),
		secondary_container: colours::RAISIN_BLACK.lighten(0.33).with_a(DEFAULT_TRANSPARENT_OPACITY),
		secondary_container_foreground: colours::RAISIN_BLACK.lighten(0.9),

		tertiary: colours::WALNUT_BROWN.darken(0.33),
		tertiary_foreground: colours::WALNUT_BROWN.lighten(0.75),
		tertiary_container: colours::WALNUT_BROWN.with_a(DEFAULT_TRANSPARENT_OPACITY),
		tertiary_container_foreground: colours::WALNUT_BROWN.lighten(0.9),

		disabled: Color::GRAY.darken(0.33),
		disabled_foreground: Color::WHITE,

		destructive: Color::RED.darken(0.33),
		destructive_foreground: Color::WHITE,

		default_font: None,
	}
);


impl Default for ThemeData
{
	fn default() -> Self
	{
		match dark_light::detect()
		{
			dark_light::Mode::Dark => themes::DARK.clone(),
			dark_light::Mode::Light => themes::LIGHT.clone(),
			dark_light::Mode::Default => themes::DARK.clone()
		}
	}
}

#[cfg(test)]
mod tests
{
	#[test]
	fn default_themes_are_accessible()
	{
		use super::*;
		themes::DARK.is_accessible();
		themes::LIGHT.is_accessible();
	}
}
