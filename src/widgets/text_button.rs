use bevy::prelude::*;

use crate::theme::ThemeData;

use super::*;
use super::base_button::*;

pub struct TextButton<U, M>
	where U: Component + Default, M: Component + Default
{
	pub base_button: BaseButton<U, M>,
	pub label: TextLabel<U>,
}

impl<U: Component + Default, M: Component + Default> TextButton<U, M>
{
	pub fn new(text: impl Into<String>)-> Self
	{
		Self
		{
			base_button: BaseButton::new()
				.with_auto_style(true),
			label: TextLabel::new(text),
		}
	}
}

impl<U: Component + Default, M: Component + Default> Widget for TextButton<U, M>
{
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
	{
		self.base_button = self.base_button.with_colour(background, foreground);
		self
	}
	fn with_border(mut self, border: UiRect) -> Self
	{
		self.base_button = self.base_button.with_border(border);
		self
	}
	fn with_direction(mut self, direction: FlexDirection) -> Self
	{
		self.base_button = self.base_button.with_direction(direction);
		self
	}
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
	{
		self.base_button = self.base_button.with_wrap(wrap);
		self
	}
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
	{
		self.base_button = self.base_button.with_align_self(align_self);
		self
	}
	fn with_align_content(mut self, align_content: AlignContent) -> Self
	{
		self.base_button = self.base_button.with_align_content(align_content);
		self
	}
	fn with_padding(mut self, padding: UiRect) -> Self
	{
		self.base_button = self.base_button.with_padding(padding);
		self
	}
	fn with_margin(mut self, margin: UiRect) -> Self
	{
		self.base_button = self.base_button.with_margin(margin);
		self
	}
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
	{
		self.base_button = self.base_button.with_fill_portion(fill_portion);
		self
	}
	fn with_theme(mut self, theme: Theme) -> Self
	{
		self.base_button = self.base_button.with_theme(theme);
		self
	}
}

impl<U: Component + Default, M: Component + Default> WidgetBuilder<U> for TextButton<U, M>
{
	fn build(&mut self, theme_data: &ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		// Build the button.
		let button_entity = self.base_button.build(theme_data, parent_theme, commands);

		// Build the label.
		let label_entity = self.label.build(theme_data, self.base_button.theme, commands);

		// Add the label to the button.
		commands.entity(button_entity).push_children(&[label_entity]);

		button_entity
	}
}

impl<U: Component + Default, M: Component + Default> Into<Box<dyn WidgetBuilder<U>>> for TextButton<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
