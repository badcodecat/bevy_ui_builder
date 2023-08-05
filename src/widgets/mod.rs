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

pub const BASE_TEXT_SIZE: f32 = 16f32;

pub mod text_label;
pub use text_label::*;

pub mod text_input;
pub use text_input::*;

pub mod base_button;
// Don't pub use button, users probablt want TextButton or ImageButton instead.

pub mod text_button;
pub use text_button::*;

pub mod checkbox;
pub use checkbox::*;

use crate::theme::Theme;

// pub fn compute_val(val: Val, parent_size: f32) -> f32
// {
// 	match val
// 	{
// 		Val::Px(px) => px,
// 		Val::Percent(percent) => parent_size * percent,
// 		Val::Auto
// 	}
// }

pub fn resize_on_window_resize
(
	window_resized_reader: EventReader<bevy::window::WindowResized>,
	mut resize_writer: EventWriter<TextResizeEvent>,
	mut aspect_ratio_writer: EventWriter<AspectRatioEvent>
)
{
	if window_resized_reader.is_empty()
		{ return; }
	resize_writer.send(TextResizeEvent);
	aspect_ratio_writer.send(AspectRatioEvent);
}

#[derive(Event)]
pub struct AspectRatioEvent;


#[derive(Component, Default)]
pub struct AspectRatio(pub f32);

pub fn ensure_aspect_ratio
(
	aspect_ratio_events: EventReader<AspectRatioEvent>,
	mut query: Query<(&AspectRatio, &mut Style, &Node)>,
	mut text_resize_writer: EventWriter<TextResizeEvent>
)
{
	if aspect_ratio_events.is_empty()
		{ return; }
	for (AspectRatio(aspect_ratio), mut style, node) in query.iter_mut()
	{
		let size = node.size();
		use std::cmp::Ordering;
		match size.x.partial_cmp(&size.y).unwrap()
		{
			Ordering::Less =>
			{
				style.height = Val::Px(size.x / aspect_ratio);
				style.width = Val::Px(size.x);
			},
			Ordering::Greater =>
			{
				style.width = Val::Px(size.y * aspect_ratio);
				style.height = Val::Px(size.y);
			},
			Ordering::Equal =>
			{
				if *aspect_ratio != 1f32
					{ todo!("Aspect ratio is not 1. This is not supported yet."); }
			}
		}
	}
	text_resize_writer.send(TextResizeEvent);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ParentData
{
	/// The last theme in the tree that isn't Theme::Auto or Theme::Custom.
	pub last_theme: Theme,
	/// The parent's theme. (Can be Theme::Auto or Theme::Custom)
	pub current_theme: Theme,
	/// The Z index of the parent.
	pub z_index: i8,
}

impl ParentData
{
	/// This is mostly for internal use.
	pub fn new(last_theme: Theme, current_theme: Theme, z_index: i8) -> Self
	{
		Self
		{
			last_theme,
			current_theme,
			z_index,
		}
	}

	/// Picks the theme that the child widget should inherit from.
	pub fn resolve_theme(&self) -> Theme
	{
		if self.current_theme == Theme::Auto
			{ self.last_theme }
		else
			{ self.current_theme }
	}
	fn from_current(&self, current_theme: Theme) -> Self
	{
		let last_theme = match current_theme
		{
			Theme::Auto => self.resolve_theme(),
			_ => current_theme,
		};
		Self
		{
			last_theme,
			current_theme,
			z_index: self.z_index,
		}
	}
}

impl Default for ParentData
{
	fn default() -> Self
	{
		Self
		{
			last_theme: Theme::Auto,
			current_theme: Theme::Auto,
			z_index: 0,
		}
	}

}

pub trait WidgetBuilder<U>
	where U: Component + Default
{
	fn build(&mut self, theme: &crate::theme::ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity;
}

impl<U: Component + Default> WidgetBuilder<U> for Entity
{
	fn build(&mut self, _: &crate::theme::ThemeData, _parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		commands.entity(*self).insert(U::default()).id()
	}
}

pub trait Widget
	where Self: Sized
{
	fn with_colour(self, background: Color, foreground: Color) -> Self;
	fn with_border(self, border: UiRect) -> Self;
	/// Note a few important things:
	///
	/// Bevy's aspect ratio fields are completely different from this one.
	///
	/// This cannot be used with fill portion.
	fn with_aspect_ratio(self, aspect_ratio: f32) -> Self;
	fn with_direction(self, direction: FlexDirection) -> Self;
	fn with_wrap(self, wrap: FlexWrap) -> Self;
	fn with_align_self(self, align_self: AlignSelf) -> Self;
	fn with_align_content(self, align_content: AlignContent) -> Self;
	fn with_padding(self, padding: UiRect) -> Self;
	fn with_fill_portion(self, fill_portion: f32) -> Self;
	fn with_margin(self, margin: UiRect) -> Self;
	fn with_theme(self, theme: Theme) -> Self;
}
