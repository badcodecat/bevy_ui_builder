use bevy::prelude::*;

use super::*;
use crate::theme::PaintMode;
use crate::theme::Theme;
use crate::theme::ThemeApplicator;
use crate::theme::ThemeData;

#[derive(Clone, Copy, Component, PartialEq, Eq, Debug)]
pub struct AutoSizedText;

#[derive(Clone, Copy, Event, PartialEq, Eq, Debug)]
pub struct TextResizeEvent;



pub fn resize_text
(
	container_query: Query<(&Children, &Node, Option<&AspectRatio>), With<AutoSizedText>>,
	mut text_query: Query<&mut Text>,
	resize_reader: EventReader<TextResizeEvent>,
	// mut resize_writer: EventWriter<TextResizeEvent>,
)
{
	if resize_reader.is_empty()
		{ return; }
	for (children, node, aspect_ratio) in container_query.iter()
	{
		let mut text = text_query.get_mut(children[0]).unwrap();
		let size = node.size();
		let text_divisor = match aspect_ratio
		{
			Some(AspectRatio(aspect_ratio)) => 4f32 / aspect_ratio,
			None => 2.25, // Magic number for a assumed 16:9 aspect ratio.
		};
		// let text_divisor = 10.1f32; //Temporary number for testing.
		let text_size = size.y / text_divisor;

		for section in text.sections.iter_mut()
		{
			section.style.font_size = text_size;
		}
	}
}
pub struct TextLabel<U, M = ()>
	where U: Component + Default, M: Default + std::any::Any + Reflect
{
	pub container: Container<U, M>,
	pub theme: Theme,
	pub label: TextBundle,
	pub custom_font: Option<Handle<Font>>,
	/// If this is set, text size will no longer be automatically determined by the size of the container.
	pub fixed_text_size: Option<f32>
}

impl<U: Component + Default, M: Default + std::any::Any + Reflect> TextLabel<U, M>
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
					justify: JustifyText::Center,
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

impl<U: Component + Default, M: Default + std::any::Any + Reflect> Widget for TextLabel<U, M>
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

impl<U: Component + Default, M: Default + std::any::Any + Reflect> ThemeApplicator for TextLabel<U, M>
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

		if self.theme == Theme::Auto
		{
			self.theme = parent_theme;
		}

		// Apply background colour.
		match self.container.paint_mode
		{
			PaintMode::Background =>
				self.label.background_color = self.theme.get_background(theme_data).into(),
			PaintMode::BackgroundContainer =>
				self.label.background_color = self.theme.get_background_container(theme_data).into(),
			PaintMode::Invisible =>
				self.label.background_color = Color::NONE.into(),
		}

		// Apply theme's text colour.
		for section in self.label.text.sections.iter_mut()
		{
			match self.container.paint_mode
			{
				PaintMode::BackgroundContainer =>
					{ section.style.color = self.theme.get_foreground_container(theme_data).into(); }
				_ =>
					{ section.style.color = self.theme.get_foreground(theme_data).into(); }
			}
		}
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
		inherited_visibility: text_bundle.inherited_visibility.clone(),
		view_visibility: text_bundle.view_visibility.clone(),
		z_index: text_bundle.z_index.clone(),
		background_color: text_bundle.background_color.clone(),
	}
}

impl<U: Component + Default, M: Default + 'static + Reflect> Into<Box<dyn WidgetBuilder<U>>> for TextLabel<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}

impl<U: Component + Default, M: Default + 'static + Reflect> WidgetBuilder<U> for TextLabel<U, M>
{
	fn build(&mut self, ui_tree: &mut crate::UIHierarchy<U>, theme_data: &ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		self.apply_theme(parent_data.resolve_theme(), theme_data);

		// Apply font size.
		let font_size = self.fixed_text_size.unwrap_or(BASE_TEXT_SIZE);
		for section in self.label.text.sections.iter_mut()
		{
			section.style.font_size = font_size;
		}

		let container = self.container.build(ui_tree, theme_data, parent_data, commands);

		let mut container = commands.entity(container);
		if self.fixed_text_size.is_none()
			{ container.insert(AutoSizedText); }
		let container = container.id();
		let label = commands
			.spawn(clone_text_bundle(&self.label))
			.insert(U::default())
			.id();
		commands.entity(container).add_child(label);
		container
	}
}
