// File Containing default themes
use once_cell::sync::Lazy;
use super::*;
pub static DARK: Lazy<ThemeData> = Lazy::new
(
	||
	ThemeData
	{
		background: Color::BLACK,
		on_background: Color::WHITE,

		primary: colours::RAISIN_BLACK.lighten(0.75),
		primary_foreground: *colours::RAISIN_BLACK,
		primary_container: colours::RAISIN_BLACK.lighten(0.33),
		primary_container_foreground: colours::RAISIN_BLACK.lighten(0.9),

		secondary: colours::ENGLISH_VIOLET.lighten(0.75),
		secondary_foreground: *colours::ENGLISH_VIOLET,
		secondary_container: colours::ENGLISH_VIOLET.lighten(0.33),
		secondary_container_foreground: colours::ENGLISH_VIOLET.lighten(0.9),

		tertiary: colours::WALNUT_BROWN.lighten(0.75),
		tertiary_foreground: *colours::WALNUT_BROWN,
		tertiary_container: colours::WALNUT_BROWN.lighten(0.20),
		tertiary_container_foreground: colours::WALNUT_BROWN.lighten(0.9),

		disabled: Color::GRAY.darken(0.33),
		disabled_foreground: Color::WHITE,

		destructive: Color::RED.darken(0.33),
		destructive_foreground: Color::WHITE,

		default_font: None
	}
);

pub static LIGHT: Lazy<ThemeData> = Lazy::new
(
	||
	ThemeData
	{
		background: Color::WHITE,
		on_background: Color::BLACK,

		primary: colours::RAISIN_BLACK.darken(0.75),
		primary_foreground: *colours::RAISIN_BLACK,
		primary_container: colours::RAISIN_BLACK.darken(0.33),
		primary_container_foreground: colours::RAISIN_BLACK.darken(0.9),

		secondary: colours::ENGLISH_VIOLET.darken(0.75),
		secondary_foreground: *colours::ENGLISH_VIOLET,
		secondary_container: colours::ENGLISH_VIOLET.darken(0.33),
		secondary_container_foreground: colours::ENGLISH_VIOLET.darken(0.9),

		tertiary: colours::WALNUT_BROWN.darken(0.75),
		tertiary_foreground: *colours::WALNUT_BROWN,
		tertiary_container: colours::WALNUT_BROWN.darken(0.20),
		tertiary_container_foreground: colours::WALNUT_BROWN.darken(0.9),

		disabled: Color::GRAY.lighten(0.33),
		disabled_foreground: Color::BLACK,

		destructive: Color::RED.lighten(0.33),
		destructive_foreground: Color::BLACK,

		default_font: None
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
