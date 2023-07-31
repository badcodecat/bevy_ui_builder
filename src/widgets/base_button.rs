use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;

use super::*;
use crate::{theme::{ThemeData, ThemeApplicator, CurrentTheme, ShiftColour}, prelude::CurrentThemeData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
/// Indicates that this button should have effects applied to it when hovered over or pressed.
pub struct AutoStyledButton;

pub fn style_button_on_focus<U: Component + Default>

(
	mut button_query: Query<(&mut BackgroundColor, &CurrentTheme<U>, &Focusable), (With<AutoStyledButton>, Changed<Focusable>)>,
	theme_data: Res<CurrentThemeData<U>>,
)
{
	let theme_data = &theme_data.0;
	for (mut background_colour, current_theme, focus) in button_query.iter_mut()
	{
		let current_theme = current_theme.0;
		let current_background_colour = current_theme.get_background(&theme_data);
		match focus.state()
		{
			FocusState::Focused => *background_colour = current_background_colour.lighten(0.1).into(),
			FocusState::Active => *background_colour = current_background_colour.lighten(0.25).into(),
			_ => *background_colour = current_background_colour.into(),
		}
	}
}

pub fn style_button_on_pressed<U: Component + Default>

(
	mut button_query: Query<(&mut BackgroundColor, &CurrentTheme<U>, &Interaction, &mut Focusable), (With<AutoStyledButton>, Changed<Interaction>)>,
	theme_data: Res<CurrentThemeData<U>>,
)
{
	let theme_data = &theme_data.0;
	for (mut background_colour, current_theme, interaction, mut focus) in button_query.iter_mut()
	{
		let current_theme = current_theme.0;
		let current_background_colour = current_theme.get_background(&theme_data);
		match *interaction
		{
			Interaction::Pressed => *background_colour = current_background_colour.darken(0.1).into(),
			_ => *focus = focus.clone(), // Other interactions are handled by style_button_on_focus, so tell it to update.
		}
	}
}

// !FIXME: The button's colour stays too long when being pressed this way.
pub fn send_pressed_on_keyboard

(
	mut button_query: Query<(&Focusable, &mut Interaction)>,
	keyboard_input: Res<Input<KeyCode>>,
)
{
	for (focus, mut interaction) in button_query.iter_mut()
	{
		if focus.state() != FocusState::Focused
			{ continue; }
		if keyboard_input.just_pressed(KeyCode::Return)
		{
			*interaction = Interaction::Pressed;
		}
	}
}

pub struct BaseButton<U, M>
	where U: Component + Default, M: Component + Default
{
	pub button_bundle: ButtonBundle,
	pub theme: Theme,
	/// Determines if AutoStyledButton should be added to this button.
	pub auto_style: bool,

	pub custom_padding: Option<UiRect>,
	pub custom_margin: Option<UiRect>,

	pub children: Vec<Box<dyn WidgetBuilder<U>>>,
	phantom: std::marker::PhantomData<M>,
}

impl<U: Component + Default, M: Component + Default> BaseButton<U, M>
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
					overflow: Overflow::clip(),
					..Default::default()
				},
				..Default::default()
			},
			theme: Theme::Auto,
			auto_style: false,

			custom_padding: None,
			custom_margin: None,

			children: Vec::new(),
			phantom: std::marker::PhantomData,
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

impl<U: Component + Default, M: Component + Default> super::Widget for BaseButton<U, M>
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

	fn with_padding(mut self, padding: UiRect) -> Self
	{
		self.custom_padding = Some(padding);
		self.button_bundle.style.padding = padding;
		self
	}

	fn with_margin(mut self, margin: UiRect) -> Self
	{
		self.custom_margin = Some(margin);
		self.button_bundle.style.margin = margin;
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

impl<U: Component + Default, M: Component + Default> ThemeApplicator for BaseButton<U, M>
{
	fn apply_theme(&mut self, parent_theme: Theme, theme_data: &ThemeData)
	{
		// Apply padding and margin.
		if let Some(padding) = self.custom_padding
		{
			self.button_bundle.style.padding = padding;
		}
		else
		{
			self.button_bundle.style.padding = theme_data.default_padding;
		}
		if let Some(margin) = self.custom_margin
		{
			self.button_bundle.style.margin = margin;
		}
		else
		{
			self.button_bundle.style.margin = theme_data.default_margin;
		}

		dbg!(parent_theme);
		dbg!(self.theme);

		self.theme = match self.theme
		{
			Theme::Auto => parent_theme,
			_ => self.theme,
		};
		dbg!(self.theme);

		self.button_bundle.background_color = self.theme.get_background(theme_data).into();
	}
}

impl<U: Component + Default, M: Component + Default> WidgetBuilder<U> for BaseButton<U, M>
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
			.insert(M::default())
			.insert(AutoStyledButton)
			.insert(CurrentTheme(self.theme, std::marker::PhantomData::<U>))
			.insert(Focusable::default())
			.push_children(&children)
			;
		if self.auto_style
			{ button.insert(AutoStyledButton); }
		button.id()
	}
}

impl<U: Component + Default, M: Component + Default> Into<Box<dyn WidgetBuilder<U>>> for BaseButton<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
