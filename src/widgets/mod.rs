// Module: widgets

use bevy::prelude::*;

pub mod container;
pub use container::*;
// Utility function to create a container with a fill portion.
pub fn create_space<U: Component + Default>(size: f32) -> container::Container<U>
{
	container::Container::new()
		.with_fill_portion(size)
}

pub mod column;
pub use column::*;

pub mod row;
pub use row::*;

pub trait WidgetBuilder<U>
	where U: Component + Default
{
	fn build(&self, theme: &crate::theme::ThemePallete, commands: &mut Commands) -> Entity;
}

impl<U: Component + Default> WidgetBuilder<U> for Entity
{
	fn build(&self, _: &crate::theme::ThemePallete, commands: &mut Commands) -> Entity
	{
		commands.entity(*self).insert(U::default()).id()
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
