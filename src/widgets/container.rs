use bevy::prelude::*;

use super::{WidgetBuilder, ParentData};
use crate::theme::{Theme, ThemeApplicator, CurrentTheme, PaintMode};

// A container is just a NodeBundle with extra steps. You should use other widgets (Column, Row, etc.) instead of this.
pub struct Container<U, M>
	where U: Component + Default, M: Default
{
	pub children: Vec<Box<dyn WidgetBuilder<U>>>,
	pub node_bundle: NodeBundle,
	pub theme: Theme,
	pub custom_padding: Option<UiRect>,
	pub custom_margin: Option<UiRect>,
	pub aspect_ratio: Option<f32>,
	pub paint_mode: PaintMode,
	phantom: std::marker::PhantomData<M>
}

impl<U: Component + Default, M: Default> Container<U, M>
{
	pub fn new() -> Self
	{
		Self
		{
			children: Vec::new(),
			node_bundle: NodeBundle
			{
				style: Style
				{
					display: Display::Flex,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					overflow: Overflow::clip(),
					..Default::default()
				},
				focus_policy: bevy::ui::FocusPolicy::Pass,
				..Default::default()
			},
			theme: Theme::Auto,
			custom_padding: None,
			custom_margin: None,
			aspect_ratio: None,
			paint_mode: PaintMode::BackgroundContainer,
			phantom: std::marker::PhantomData
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn WidgetBuilder<U>>>) -> Self
	{
		self.children.push(child.into());
		self
	}

	pub fn with_size(mut self, width: Val, height: Val) -> Self
	{
		self.node_bundle.style.width = width;
		self.node_bundle.style.height = height;
		self
	}
}


impl<U: Component + Default, M: Default> super::Widget for Container<U, M>
{
	fn with_paint_mode(mut self, paint_mode: PaintMode) -> Self
	{
		self.paint_mode = paint_mode;
		self
	}
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
	{
		self.theme = Theme::Custom(background, foreground);
		self
	}

	fn with_border(mut self, border: UiRect) -> Self
	{
		self.node_bundle.style.border = border;
		self
	}

	fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self
	{
		// It's important to remember that bevy's aspect ratio is completely different from what we want.
		self.aspect_ratio = Some(aspect_ratio);
		self
	}

	fn with_direction(mut self, direction: FlexDirection) -> Self
	{
		self.node_bundle.style.flex_direction = direction;
		self
	}

	fn with_wrap(mut self, wrap: FlexWrap) -> Self
	{
		self.node_bundle.style.flex_wrap = wrap;
		self
	}

	// Sets the alignment of this element, WARNING: this will override the alignment options of the parent.
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
	{
		self.node_bundle.style.align_self = align_self;
		self
	}

	fn with_align_content(mut self, align_content: AlignContent) -> Self
	{
		self.node_bundle.style.align_content = align_content;
		self
	}

	fn with_padding(mut self, padding: UiRect) -> Self
	{
		self.custom_padding = Some(padding);
		self.node_bundle.style.padding = padding;
		self
	}

	fn with_margin(mut self, margin: UiRect) -> Self
	{
		self.custom_margin = Some(margin);
		self.node_bundle.style.margin = margin;
		self
	}

	fn with_fill_portion(mut self, fill_portion: f32) -> Self
	{
		self.node_bundle.style.flex_basis = Val::Percent(fill_portion * 100.0);
		self
	}

	fn with_theme(mut self, theme: Theme) -> Self
	{
		self.theme = theme;
		self
	}

}

impl<U: Component + Default, M: Default> ThemeApplicator for Container<U, M>
{
	fn apply_theme(&mut self, parent_theme: Theme, theme_data: &crate::theme::ThemeData)
	{
		// Apply padding & margin.
		if let Some(padding) = self.custom_padding
		{
			self.node_bundle.style.padding = padding;
		}

		let theme = match self.theme
		{
			Theme::Auto => parent_theme,
			_ => self.theme
		};

		match self.paint_mode
		{
			PaintMode::Background =>
				self.node_bundle.background_color = theme.get_background(theme_data).into(),
			PaintMode::BackgroundContainer =>
				self.node_bundle.background_color = theme.get_background_container(theme_data).into(),
			PaintMode::Invisible =>
				self.node_bundle.background_color = Color::NONE.into(),
		}

		match self.paint_mode
		{
			PaintMode::Background =>
				self.node_bundle.border_color = theme.get_background_container(theme_data).into(),
			PaintMode::BackgroundContainer =>
				self.node_bundle.border_color = theme.get_background(theme_data).into(),
			PaintMode::Invisible =>
				self.node_bundle.border_color = Color::NONE.into(),
		}
	}
}

impl<U: Component + Default, M: std::any::Any + Default> WidgetBuilder<U> for Container<U, M>
{
	fn build(&mut self, theme_data: &crate::theme::ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		self.apply_theme(parent_data.resolve_theme(), theme_data);

		let new_parent_data = parent_data.from_current(self.theme);

		let children: Vec<Entity> = self.children.iter_mut().map(|child| child.build(theme_data, new_parent_data, commands)).collect();
		let mut this_container = commands.spawn(self.node_bundle.clone()); // TODO: See if we can avoid cloning the node bundle.

		if let Some(aspect_ratio) = self.aspect_ratio
		{
			this_container.insert(super::AspectRatio(aspect_ratio));
		}

		// Check if M is a Component, and if so, insert it.
		use std::any::Any;
		let m_any: Box<dyn Any> = Box::new(M::default());
		let m_any_component_check: Box<dyn Any> = Box::new(M::default());
		if m_any_component_check.downcast::<Box<dyn Component<Storage = bevy::ecs::storage::Table>>>().is_ok()
		{
			if let Ok(m_component) = m_any.downcast::<Box<dyn Reflect>>()
			{
				let m_component = *m_component;
				use bevy::ecs::reflect::ReflectCommandExt;
				this_container.insert_reflect(m_component);
			}
		}

		this_container
			.insert(U::default())
			.insert(CurrentTheme(self.theme, std::marker::PhantomData::<U>))
			.push_children(&children)
			.id()
	}
}

impl<U: Component + Default, M: Default + 'static> From<Container<U, M>> for Box<dyn WidgetBuilder<U>>
{
	fn from(container: Container<U, M>) -> Self
	{
		Box::new(container)
	}
}
