// Module: widgets

use bevy::prelude::*;

pub mod container;
pub use container::*;
// Utility function to create a container with a fill portion.
pub fn create_space(size: f32) -> container::Container
{
	container::Container::new()
		.with_fill_portion(size)
}

pub mod column;
pub use column::*;

pub mod row;
pub use row::*;

pub trait WidgetBuilder
{
	fn build(&self, commands: &mut Commands) -> Entity;
}

impl WidgetBuilder for Entity
{
	fn build(&self, _: &mut Commands) -> Entity
	{
		*self
	}
}

pub trait Widget
	where Self: Sized
{
	fn with_colour(self, colour: Color) -> Self;
	fn with_direction(self, direction: FlexDirection) -> Self;
	fn with_wrap(self, wrap: FlexWrap) -> Self;
	fn with_align_self(self, align_self: AlignSelf) -> Self;
	fn with_align_content(self, align_content: AlignContent) -> Self;
	fn with_fill_portion(self, fill_portion: f32) -> Self;
}
