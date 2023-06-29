use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Theme
{
	Background,
	Primary,
	Secondary,
	Tertiary,
	Disabled,
	Destructive,
	Auto
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemePallete
{
	pub background: Color,
	pub on_background: Color,

	pub primary: Color,
	pub primary_foreground: Color,
	pub primary_container: Color,
	pub primary_container_foreground: Color,

	pub secondary: Color,
	pub secondary_foreground: Color,
	pub secondary_container: Color,
	pub secondary_container_foreground: Color,

	pub tertiary: Color,
	pub tertiary_foreground: Color,
	pub tertiary_container: Color,
	pub tertiary_container_foreground: Color,

	pub disabled: Color,
	pub disabled_foreground: Color,

	pub destructive: Color,
	pub destructive_foreground: Color,
}

// See https://m3.material.io/foundations/accessible-design/patterns#c06040d0-f7dd-43d8-af92-384bbb3b0544
pub const CONTRAST_ACCESIBILITY_RATIO: f64 = 4.5;

pub fn is_contrast_accessible(color1: Color, color2: Color) -> bool
{
	get_contrast_ratio(color1, color2) >= CONTRAST_ACCESIBILITY_RATIO
}
pub trait ShiftColour
{
	fn lighten(self, amount: f32) -> Self;
	fn darken(self, amount: f32) -> Self;
}

impl ShiftColour for Color
{
	fn lighten(self, amount: f32) -> Self
	{
		let Color::Hsla { hue, saturation, lightness, alpha } = self.as_hsla()
			else { unreachable!("Color::as_hsla() returned a non-HSLA color") };
		Color::hsla(hue, saturation, lightness + amount, alpha)
	}

	fn darken(self, amount: f32) -> Self
	{
		let Color::Hsla { hue, saturation, lightness, alpha } = self.as_hsla()
			else { unreachable!("Color::as_hsla() returned a non-HSLA color") };
		Color::hsla(hue, saturation, lightness - amount, alpha)
	}

}

impl ThemePallete
{
	fn is_accessible(&self)
	{
		assert!(is_contrast_accessible(self.background, self.on_background));
		assert!(is_contrast_accessible(self.primary, self.primary_foreground));
		assert!(is_contrast_accessible(self.primary_container, self.primary_container_foreground));
		assert!(is_contrast_accessible(self.secondary, self.secondary_foreground));
		assert!(is_contrast_accessible(self.secondary_container, self.secondary_container_foreground));
		assert!(is_contrast_accessible(self.tertiary, self.tertiary_foreground));
		assert!(is_contrast_accessible(self.tertiary_container, self.tertiary_container_foreground));
		assert!(is_contrast_accessible(self.disabled, self.disabled_foreground));
 		assert!(is_contrast_accessible(self.destructive, self.destructive_foreground));
	}
}

pub mod colours
{
	use once_cell::sync::Lazy;
	use super::*;
	// Primary, HEX: #202030
	pub static RAISIN_BLACK: Lazy<Color> = Lazy::new(|| Color::hex("202030").unwrap());
	// Secondary, HEX: #39304A
	pub static  ENGLISH_VIOLET: Lazy<Color> = Lazy::new(|| Color::hex("39304A").unwrap());
	// Tertiary, HEX: #635C51
	pub static  WALNUT_BROWN: Lazy<Color> = Lazy::new(|| Color::hex("635C51").unwrap());
}

pub mod themes
{
	use once_cell::sync::Lazy;
	use super::*;
	pub static DARK: Lazy<ThemePallete> = Lazy::new
	(
		||
		ThemePallete
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

			destructive: Color::RED,
			destructive_foreground: Color::WHITE
		}
	);

	pub static LIGHT: Lazy<ThemePallete> = Lazy::new
	(
		||
		ThemePallete
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

			destructive: Color::RED,
			destructive_foreground: Color::BLACK
		}
	);
}

impl Default for ThemePallete
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


// TODO: Remove this warning when the function has been verified or rewritten.
/// WARNING: This entire function was generated by ChatGPT and GitHub Copilot,
pub fn get_contrast_ratio(color1: Color, color2: Color) -> f64
{
	// Convert RGBA values to relative luminance
	fn relative_luminance(color: Color) -> f64
	{
		let Color::Rgba { red, green, blue, .. } = color.as_rgba()
			else { unreachable!("Color is not RGBA") };
		let r = red as f64;
		let g = green as f64;
		let b = blue as f64;
		let r_l = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
		let g_l = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
		let b_l = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };
		0.2126 * r_l + 0.7152 * g_l + 0.0722 * b_l
	}

	fn mix_colors(color1: Color, color2: Color) -> Color
	{
		let Color::Rgba { red: r1, green: g1, blue: b1, alpha: a1 } = color1.as_rgba()
			else { unreachable!("Color is not RGBA") };
		let Color::Rgba { red: r2, green: g2, blue: b2, alpha: a2 } = color2.as_rgba()
			else { unreachable!("Color is not RGBA") };
		let r = ( r1 + ( r2 * ( 1.0 - a1 ) ) ) / ( a1 + a2 * ( 1.0 - a1 ) );
		let g = ( g1 + ( g2 * ( 1.0 - a1 ) ) ) / ( a1 + a2 * ( 1.0 - a1 ) );
		let b = ( b1 + ( b2 * ( 1.0 - a1 ) ) ) / ( a1 + a2 * ( 1.0 - a1 ) );
		let a = a1;
		Color::Rgba { red: r, green: g, blue: b, alpha: a }
	}

		// Calculate the contrast ratio with color mixing
		let mixed_color1 = mix_colors(color1, color2);
		let mixed_color2 = mix_colors(color2, color1);
		let l1 = relative_luminance(mixed_color1);
		let l2 = relative_luminance(mixed_color2);
		if l1 > l2
		{
			(l1 + 0.05) / (l2 + 0.05)
		}
		else
		{
			(l2 + 0.05) / (l1 + 0.05)
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
	}
}
