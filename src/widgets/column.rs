use bevy::prelude::*;

use super::*;

use super::Container;

pub struct Column<U, M = ()>
	where U: Component + Default, M: Default + std::any::Any + Reflect
{
	pub container: Container<U, M>,
}

impl<U: Component + Default, M: Default + std::any::Any + Reflect> Column<U, M>
{
	pub fn new() -> Self
	{
		Self
		{
			container: Container::new()
				.with_direction(FlexDirection::Column)
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn super::WidgetBuilder<U>>>) -> Self
		{ self.container = self.container.push(child); self }
}

impl<U: Component + Default, M: Default + std::any::Any + Reflect> super::Widget for Column<U, M>
{
	fn with_paint_mode(mut self, paint_mode: PaintMode) -> Self
		{ self.container = self.container.with_paint_mode(paint_mode); self }
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
		{ self.container = self.container.with_colour(background, foreground); self }
	fn with_border(mut self, border: UiRect) -> Self
		{ self.container = self.container.with_border(border); self }
	fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self
		{ self.container = self.container.with_aspect_ratio(aspect_ratio); self }
	fn with_direction(mut self, direction: FlexDirection) -> Self
		{ self.container = self.container.with_direction(direction); self }
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
		{ self.container = self.container.with_wrap(wrap); self }
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
		{ self.container = self.container.with_align_self(align_self); self }
	fn with_align_content(mut self, align_content: AlignContent) -> Self
		{ self.container = self.container.with_align_content(align_content); self }
	fn with_padding(mut self, padding: UiRect) -> Self
		{ self.container = self.container.with_padding(padding); self }
	fn with_margin(mut self, margin: UiRect) -> Self
		{ self.container = self.container.with_margin(margin); self }
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
		{ self.container = self.container.with_fill_portion(fill_portion); self }
	fn with_theme(mut self, theme: Theme) -> Self
		{ self.container = self.container.with_theme(theme); self }
}

impl<U: Component + Default, M: Default + 'static + Reflect> super::WidgetBuilder<U> for Column<U, M>
{
	fn build(&mut self, ui_tree: &mut crate::UIHierarchy<U>, theme_data: &crate::theme::ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		self.container.build(ui_tree, theme_data, parent_data, commands)
	}
}

impl<U: Component + Default, M: Default + 'static + Reflect> Into<Box<dyn super::WidgetBuilder<U>>> for Column<U, M>
{
	fn into(self) -> Box<dyn super::WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
