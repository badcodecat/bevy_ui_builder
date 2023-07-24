use bevy::prelude::*;

use super::*;
use crate::theme::Theme;
use crate::theme::ThemeApplicator;
use crate::theme::ThemeData;

#[derive(Clone, Copy, Component, PartialEq, Eq, Debug)]
pub struct AutoSizedText;

#[derive(Clone, Copy, Event, PartialEq, Eq, Debug)]
pub struct TextResizeEvent;

pub fn resize_text_on_window_resize
(
	mut window_resized_reader: EventReader<bevy::window::WindowResized>,
	mut resize_writer: EventWriter<TextResizeEvent>,
)
{
	if window_resized_reader.iter().next().is_none()
		{ return; }
	resize_writer.send(TextResizeEvent);
}

pub fn resize_text
(
	container_query: Query<(&Children, &Node), With<AutoSizedText>>,
	mut text_query: Query<&mut Text>,
	mut resize_reader: EventReader<TextResizeEvent>,
	// mut resize_writer: EventWriter<TextResizeEvent>,
)
{
	if resize_reader.iter().next().is_none()
		{ return; }
	for (children, node) in container_query.iter()
	{
		let mut text = text_query.get_mut(children[0]).unwrap();
		let text_size = node.size().y / 2f32;
		// if text_size == 0f32
		// {
		// 	println!("Text size is 0, skipping.");
		// 	resize_writer.send(TextResizeEvent);
		// 	continue;
		// }
		for section in text.sections.iter_mut()
		{
			section.style.font_size = text_size;
		}
	}
}
pub struct TextLabel<U>
	where U: Component + Default
{
	pub container: Container<U>,
	pub theme: Theme,
	pub label: TextBundle,
	pub custom_font: Option<Handle<Font>>,
	/// If this is set, text size will no longer be automatically determined by the size of the container.
	pub fixed_text_size: Option<f32>
}

impl<U: Component + Default> TextLabel<U>
{
	pub fn new(text: impl Into<String>) -> Self
	{
		let text: String = text.into();
		let text = TextSection
		{
			value: text,
			..Default::default()
		};
		Self
		{
			container: Container::new()
				.with_direction(FlexDirection::Row)
				.with_align_content(AlignContent::Center),
			theme: Theme::Auto,
			label: TextBundle
			{
				text: Text
				{
					alignment: TextAlignment::Center,
					sections: vec![text],
					..Default::default()
				},
				..Default::default()
			},
			custom_font: None,
			fixed_text_size: None,
		}
	}

	pub fn with_font(mut self, font: Handle<Font>) -> Self
	{
		self.custom_font = Some(font);
		self
	}

	/// Sets the text size to a fixed value rather than automatically determining it from the size of the container.
	pub fn with_text_size(mut self, text_size: f32) -> Self
	{
		self.fixed_text_size = Some(text_size);
		self
	}

	pub fn with_theme(mut self, theme: Theme) -> Self
	{
		self.theme = theme;
		self
	}

}

impl<U: Component + Default> Widget for TextLabel<U>
{
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
		{ self.container = self.container.with_colour(background, foreground); self }
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

impl<U: Component + Default> ThemeApplicator for TextLabel<U>
{
	fn apply_theme(&mut self, parent_theme: Theme, theme_data: &ThemeData)
	{
		// Apply theme's font.
		if let Some(font) = &theme_data.default_font
		{
			for section in self.label.text.sections.iter_mut()
			{
				section.style.font = font.clone();
			}
		}
		// Apply theme's text colour.
		for section in self.label.text.sections.iter_mut()
		{
			section.style.color = parent_theme.get_foreground_container(theme_data).into();
		}

		// Apply theme colour.
		self.container.apply_theme(parent_theme, theme_data);
	}
}

pub fn clone_text_bundle(text_bundle: &TextBundle) -> TextBundle
{
	TextBundle
	{
		node: text_bundle.node.clone(),
		style: text_bundle.style.clone(),
		text: text_bundle.text.clone(),
		text_layout_info: text_bundle.text_layout_info.clone(),
		text_flags: text_bundle.text_flags.clone(),
		calculated_size: Default::default(), // This is the only field that is not cloned.
		focus_policy: text_bundle.focus_policy.clone(),
		transform: text_bundle.transform.clone(),
		global_transform: text_bundle.global_transform.clone(),
		visibility: text_bundle.visibility.clone(),
		computed_visibility: text_bundle.computed_visibility.clone(),
		z_index: text_bundle.z_index.clone(),
		background_color: text_bundle.background_color.clone(),
	}
}

impl<U: Component + Default> Into<Box<dyn WidgetBuilder<U>>> for TextLabel<U>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}

impl<U: Component + Default> WidgetBuilder<U> for TextLabel<U>
{
	fn build(&mut self, theme_data: &ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		let parent_theme = if self.theme == Theme::Auto { parent_theme } else { self.theme };
		self.apply_theme(parent_theme, theme_data);

		// Apply font size.
		let font_size = self.fixed_text_size.unwrap_or(BASE_TEXT_SIZE);
		for section in self.label.text.sections.iter_mut()
		{
			section.style.font_size = font_size;
		}

		let container = self.container.build(theme_data, parent_theme, commands);

		let container = commands.entity(container).insert(AutoSizedText).id();
		let label = commands
			.spawn(clone_text_bundle(&self.label))
			.insert(U::default())
			.id();
		commands.entity(container).add_child(label);
		container
	}
}
