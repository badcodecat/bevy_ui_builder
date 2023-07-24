use bevy::prelude::*;

use super::*;
use crate::theme::{ThemeData, ThemeApplicator};

pub struct Button<U>
	where U: Component + Default
{
	pub container: Container<U>,
	pub button_bundle: ButtonBundle,
	pub custom_colour: Option<Color>,
	pub theme: Theme,

	pub children: Vec<Box<dyn WidgetBuilder<U>>>,
}

impl<U: Component + Default> Button<U>
{
	pub fn new() -> Self
	{
		Self
		{
			container: Container::new(),
			button_bundle: ButtonBundle::default(),
			custom_colour: None,
			theme: Theme::Auto,

			children: Vec::new(),
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn WidgetBuilder<U>>>) -> Self
	{
		self.children.push(child.into());
		self
	}
}

impl<U: Component + Default> super::Widget for Button<U>
{
	fn with_colour(mut self, colour: Color) -> Self
	{
		self.custom_colour = Some(colour);
		self
	}

	fn with_direction(mut self, direction: FlexDirection) -> Self
	{
		self.button_bundle.style.flex_direction = direction;
		self
	}

	fn with_wrap(mut self, wrap: FlexWrap) -> Self
	{
		self.button_bundle.style.flex_wrap = wrap;
		self
	}

	// Sets the alignment of this element, WARNING: this will override the alignment options of the parent.
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
	{
		self.button_bundle.style.align_self = align_self;
		self
	}

	fn with_align_content(mut self, align_content: AlignContent) -> Self
	{
		self.button_bundle.style.align_content = align_content;
		self
	}

	fn with_fill_portion(mut self, fill_portion: f32) -> Self
	{
		self.button_bundle.style.flex_basis = Val::Percent(fill_portion * 100.0);
		self
	}

	fn with_theme(mut self, theme: Theme) -> Self
	{
		self.theme = theme;
		self
	}
}

impl<U: Component + Default> ThemeApplicator for Button<U>
{
	fn apply_theme(&mut self, theme: Theme, theme_data: &ThemeData)
	{
		self.button_bundle.background_color = self.custom_colour.unwrap_or(theme.get_background(theme_data)).into();
	}
}

impl<U: Component + Default> WidgetBuilder<U> for Button<U>
{
	fn build(&mut self, theme: &crate::theme::ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		// Apply theming.
		self.apply_theme(self.theme, theme);

		// Build children.
		let parent_theme = if self.theme == Theme::Auto { parent_theme } else { self.theme };
		let children: Vec<Entity> = self.children.iter_mut().map(|child| child.build(theme, parent_theme, commands)).collect();

		commands.spawn(self.button_bundle.clone())
			.insert(U::default())
			.push_children(&children)
			.id()
	}
}
