use bevy::prelude::*;

use super::*;

use super::Container;

pub struct Column
{
	pub container: Container,
}

impl Column
{
	pub fn new() -> Self
	{
		Self
		{
			container: Container::new()
				.with_direction(FlexDirection::Column),
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn super::WidgetBuilder>>) -> Self
		{ self.container = self.container.push(child); self }
}

impl super::Widget for Column
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
}

impl super::WidgetBuilder for Column
{
	fn build(&self, commands: &mut Commands) -> Entity
	{
		self.container.build(commands)
	}
}

impl Into<Box<dyn super::WidgetBuilder>> for Column
{
	fn into(self) -> Box<dyn super::WidgetBuilder>
	{
		Box::new(self)
	}
}
