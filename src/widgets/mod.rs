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

pub enum TextSize
{
	Custom(f32),
	Body,
	SubHeading,
	Heading,
}

pub const BASE_TEXT_SIZE: f32 = 16f32;

pub mod label;
pub use label::*;

pub mod base_button;
// Don't pub use button, users probablt want TextButton or ImageButton instead.

use crate::theme::Theme;

pub trait WidgetBuilder<U>
	where U: Component + Default
{
	fn build(&mut self, theme: &crate::theme::ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity;
}

impl<U: Component + Default> WidgetBuilder<U> for Entity
{
	fn build(&mut self, _: &crate::theme::ThemeData, _parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		commands.entity(*self).insert(U::default()).id()
	}
}

pub trait Widget
	where Self: Sized
{
	fn with_colour(self, background: Color, foreground: Color) -> Self;
	fn with_direction(self, direction: FlexDirection) -> Self;
	fn with_wrap(self, wrap: FlexWrap) -> Self;
	fn with_align_self(self, align_self: AlignSelf) -> Self;
	fn with_align_content(self, align_content: AlignContent) -> Self;
	fn with_fill_portion(self, fill_portion: f32) -> Self;
	fn with_theme(self, theme: Theme) -> Self;
}
