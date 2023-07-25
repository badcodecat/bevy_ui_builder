use bevy::prelude::*;

use super::*;
use crate::{theme::{ThemeData, ThemeApplicator, CurrentTheme, ShiftColour}, prelude::CurrentThemeData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
/// Indicates that this button should have effects applied to it when hovered over or pressed.
pub struct AutoStyledButton;

pub fn style_button_on_hover<U: Component + Default>

(
	mut button_query: Query<(&mut BackgroundColor, &CurrentTheme<U>, &Interaction), (With<AutoStyledButton>, Changed<Interaction>)>,
	theme_data: Res<CurrentThemeData<U>>,
)
{
	let theme_data = &theme_data.0;
	for (mut background_colour, current_theme, interaction) in button_query.iter_mut()
	{
		let current_theme = current_theme.0;
		let current_background_colour = current_theme.get_background(&theme_data);
		match *interaction
		{
			Interaction::Hovered => *background_colour = current_background_colour.lighten(0.1).into(),
			Interaction::Pressed => *background_colour = current_background_colour.darken(0.1).into(),
			Interaction::None => *background_colour = current_background_colour.into(),
		}
	}
}

pub struct BaseButton<U>
	where U: Component + Default
{
	pub button_bundle: ButtonBundle,
	pub theme: Theme,
	pub auto_style: bool,

	pub children: Vec<Box<dyn WidgetBuilder<U>>>,
}

impl<U: Component + Default> BaseButton<U>
{
	pub fn new() -> Self
	{
		Self
		{
			button_bundle: ButtonBundle
			{
				style: Style
				{
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..Default::default()
				},
				..Default::default()
			},
			theme: Theme::Auto,
			auto_style: false,

			children: Vec::new(),
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn WidgetBuilder<U>>>) -> Self
	{
		self.children.push(child.into());
		self
	}

	pub fn with_auto_style(mut self, should_auto_style: bool) -> Self
	{
		self.auto_style = should_auto_style;
		self
	}
}

impl<U: Component + Default> super::Widget for BaseButton<U>
{
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
	{
		self.theme = Theme::Custom(background, foreground);
		self
	}

	fn with_border(mut self, border: UiRect) -> Self
	{
		self.button_bundle.style.border = border;
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

impl<U: Component + Default> ThemeApplicator for BaseButton<U>
{
	fn apply_theme(&mut self, parent_theme: Theme, theme_data: &ThemeData)
	{
		self.theme = match self.theme
		{
			Theme::Auto => parent_theme.get_next_layer(),
			_ => self.theme,
		};

		self.button_bundle.background_color = self.theme.get_background(theme_data).into();
	}
}

impl<U: Component + Default> WidgetBuilder<U> for BaseButton<U>
{
	fn build(&mut self, theme: &crate::theme::ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		// Apply theming.
		self.apply_theme(parent_theme, theme);

		// Build children.
		let children: Vec<Entity> = self.children.iter_mut().map(|child| child.build(theme, self.theme, commands)).collect();

		let mut button = commands.spawn(self.button_bundle.clone());
		button
			.insert(U::default())
			.insert(AutoStyledButton)
			.insert(CurrentTheme(self.theme, std::marker::PhantomData::<U>))
			.push_children(&children)
			;
		if self.auto_style
			{ button.insert(AutoStyledButton); }
		button.id()
	}
}

impl<U: Component + Default> Into<Box<dyn WidgetBuilder<U>>> for BaseButton<U>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
