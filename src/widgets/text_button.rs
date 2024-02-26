use bevy::prelude::*;

use crate::theme::{ThemeData, PaintMode};

use super::*;
use super::base_button::*;

pub struct TextButton<U, M = ()>
	where U: Component + Default, M: UIOptionalUniqueIdentifier
{
	pub base_button: BaseButton<U, M>,
	pub label: TextLabel<U, M>,
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> TextButton<U, M>
{
	pub fn new(text: impl Into<String>)-> Self
	{
		Self
		{
			base_button: BaseButton::new()
				.with_auto_style(true),
			label: TextLabel::new(text)
				.with_paint_mode(PaintMode::Invisible)
		}
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Widget for TextButton<U, M>
{
	fn with_paint_mode(mut self, paint_mode: PaintMode) -> Self
		{ self.base_button = self.base_button.with_paint_mode(paint_mode); self }
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
		{ self.base_button = self.base_button.with_colour(background, foreground); self }
	fn with_border(mut self, border: UiRect) -> Self
		{ self.base_button = self.base_button.with_border(border); self }
	fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self
		{ self.base_button = self.base_button.with_aspect_ratio(aspect_ratio); self }
	fn with_direction(mut self, direction: FlexDirection) -> Self
		{ self.base_button = self.base_button.with_direction(direction); self }
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
		{ self.base_button = self.base_button.with_wrap(wrap); self }
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
		{ self.base_button = self.base_button.with_align_self(align_self); self }
	fn with_align_content(mut self, align_content: AlignContent) -> Self
		{ self.base_button = self.base_button.with_align_content(align_content); self }
	fn with_padding(mut self, padding: UiRect) -> Self
		{ self.base_button = self.base_button.with_padding(padding); self }
	fn with_margin(mut self, margin: UiRect) -> Self
		{ self.base_button = self.base_button.with_margin(margin); self }
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
		{ self.base_button = self.base_button.with_fill_portion(fill_portion); self }
	fn with_theme(mut self, theme: Theme) -> Self
		{ self.base_button = self.base_button.with_theme(theme); self }
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> WidgetBuilder<U> for TextButton<U, M>
{
	fn build(&mut self, ui_tree: &mut crate::UIHierarchy<U>, theme_data: &ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		// Build the button.
		let button_entity = self.base_button.build(ui_tree, theme_data, parent_data, commands);

		// Build the label.
		let label_entity = self.label.build(ui_tree, theme_data, parent_data, commands);

		// Add the label to the button.
		commands.entity(button_entity).push_children(&[label_entity]);

		button_entity
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Into<Box<dyn WidgetBuilder<U>>> for TextButton<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
