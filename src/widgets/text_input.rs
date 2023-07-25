// Really a TextInput is just a label with extra steps

use crate::theme::ThemeData;

use super::*;

#[derive(Component, Default)]
pub struct TextInputReciever;




pub struct TextInput<U>
	where U: Component + Default
{
	pub label: TextLabel<U>,
}

impl<U: Component + Default> TextInput<U>
{
	pub fn new(text: impl Into<String>) -> Self
	{
		Self
		{
			label: TextLabel::new(text)
				.with_border(UiRect::all(Val::Percent(3f32)))
		}
	}
}

impl<U: Component + Default> Widget for TextInput<U>
{
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
	{
		self.label = self.label.with_colour(background, foreground);
		self
	}
	fn with_border(mut self, border: UiRect) -> Self
	{
		self.label = self.label.with_border(border);
		self
	}
	fn with_direction(mut self, direction: FlexDirection) -> Self
	{
		self.label = self.label.with_direction(direction);
		self
	}
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
	{
		self.label = self.label.with_wrap(wrap);
		self
	}
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
	{
		self.label = self.label.with_align_self(align_self);
		self
	}
	fn with_align_content(mut self, align_content: AlignContent) -> Self
	{
		self.label = self.label.with_align_content(align_content);
		self
	}
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
	{
		self.label = self.label.with_fill_portion(fill_portion);
		self
	}
	fn with_theme(mut self, theme: Theme) -> Self
	{
		self.label = self.label.with_theme(theme);
		self
	}
}

impl<U: Component + Default> WidgetBuilder<U> for TextInput<U>
{
	fn build(&mut self, theme_data: &ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		self.label.build(theme_data, parent_theme, commands)
	}
}
