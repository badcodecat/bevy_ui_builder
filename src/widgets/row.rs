use bevy::prelude::*;

use crate::theme::ThemeApplicator;

use super::*;

use super::Container;

pub struct Row<U>
	where U: Component + Default
{
	pub container: Container<U>,
}

impl<U: Component + Default> Row<U>
{
	pub fn new() -> Self
	{
		Self
		{
			container: Container::new()
				.with_direction(FlexDirection::Row),
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn super::WidgetBuilder<U>>>) -> Self
		{ self.container = self.container.push(child); self }
}

impl<U: Component + Default> super::Widget for Row<U>
{
	fn with_colour(mut self, colour: Color) -> Self
		{ self.container = self.container.with_colour(colour); self }
	fn with_direction(mut self, direction: FlexDirection) -> Self
		{ self.container = self.container.with_direction(direction); self }
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
		{ self.container = self.container.with_wrap(wrap); self }
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
		{ self.container = self.container.with_align_self(align_self); self }
	fn with_align_content(mut self, align_content: AlignContent) -> Self
		{ self.container = self.container.with_align_content(align_content); self }
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
		{ self.container = self.container.with_fill_portion(fill_portion); self }
	fn with_theme(mut self, theme: Theme) -> Self
		{ self.container = self.container.with_theme(theme); self }
}

impl<U: Component + Default> super::WidgetBuilder<U> for Row<U>
{
	fn build(&mut self, theme_data: &crate::theme::ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		// Apply theming.
		self.container.apply_theme(self.container.theme, theme_data);

		let parent_theme = if self.container.theme == Theme::Auto { parent_theme } else { self.container.theme };
		self.container.build(theme_data, parent_theme, commands)
	}
}


impl<U: Component + Default> Into<Box<dyn super::WidgetBuilder<U>>> for Row<U>
{
	fn into(self) -> Box<dyn super::WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
