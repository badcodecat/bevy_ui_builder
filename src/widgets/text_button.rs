use bevy::prelude::*;

use crate::theme::{ThemeData, ThemeApplicator};

use super::*;
use super::base_button::*;

pub struct TextButton<U>
	where U: Component + Default
{
	pub base_button: BaseButton<U>,
	pub label: TextLabel<U>,
}

impl<U: Component + Default> TextButton<U>
{
	pub fn new(text: impl Into<String>)-> Self
	{
		Self
		{
			base_button: BaseButton::new(),
			label: TextLabel::new(text),
		}
	}
}

impl<U: Component + Default> Widget for TextButton<U>
{
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
	{
		self.base_button = self.base_button.with_colour(background, foreground);
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

impl<U: Component + Default> ThemeApplicator for TextButton<U>
{
	fn apply_theme(&mut self, parent_theme: Theme, theme_data: &ThemeData)
	{
		self.base_button.apply_theme(parent_theme, theme_data);
		self.label.apply_theme(parent_theme, theme_data);
	}
}

impl<U: Component + Default> WidgetBuilder<U> for TextButton<U>
{
	fn build(&mut self, theme_data: &ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		let parent_theme = if parent_theme == Theme::Auto { Theme::Background } else { parent_theme };
		// Apply theming.
		let parent_theme = if self.base_button.theme == Theme::Auto { parent_theme } else { self.base_button.theme };
		self.base_button.apply_theme(parent_theme, theme_data);

		// Build the button.
		let button_entity = self.base_button.build(theme_data, parent_theme, commands);

		// Build the label.
		let label_entity = self.label.build(theme_data, self.base_button.theme, commands);

		// Add the label to the button.
		commands.entity(button_entity).push_children(&[label_entity]);

		button_entity
	}
}

impl<U: Component + Default> Into<Box<dyn WidgetBuilder<U>>> for TextButton<U>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
