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

pub struct BaseButton<U, M = ()>
	where U: Component + Default, M: Default
{
	pub button_bundle: ButtonBundle,
	pub theme: Theme,
	pub paint_mode: PaintMode,
	/// Determines if AutoStyledButton should be added to this button.
	pub auto_style: bool,

	pub custom_padding: Option<UiRect>,
	pub custom_margin: Option<UiRect>,

	pub aspect_ratio: Option<f32>,

	pub children: Vec<Box<dyn WidgetBuilder<U>>>,
	phantom: std::marker::PhantomData<M>,
}

impl<U: Component + Default, M: Default> BaseButton<U, M>
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
			paint_mode: PaintMode::Background,
			auto_style: false,

			custom_padding: None,
			custom_margin: None,

			aspect_ratio: None,

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

impl<U: Component + Default, M: Default> super::Widget for BaseButton<U, M>
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
		self.button_bundle.style.border = border;
		self
	}

	fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self
	{
		self.aspect_ratio = Some(aspect_ratio);
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

impl<U: Component + Default, M: Default> ThemeApplicator for BaseButton<U, M>
{
	fn apply_theme(&mut self, parent_theme: Theme, theme_data: &ThemeData)
	{
		// Apply padding and margin.
		if let Some(padding) = self.custom_padding
		{
			self.button_bundle.style.padding = padding;
		}
		if let Some(margin) = self.custom_margin
		{
			self.button_bundle.style.margin = margin;
		}

		let theme = match self.theme
		{
			Theme::Auto => parent_theme,
			_ => self.theme
		};

		self.button_bundle.background_color = match self.paint_mode
		{
			PaintMode::Background =>
				theme.get_background(theme_data).into(),
			PaintMode::BackgroundContainer =>
				theme.get_background_container(theme_data).into(),
			PaintMode::Invisible =>
				Color::NONE.into(),
		};
		self.button_bundle.border_color = match self.paint_mode
		{
			PaintMode::Background =>
				theme.get_background_container(theme_data).into(),
			PaintMode::BackgroundContainer =>
				theme.get_background(theme_data).into(),
			PaintMode::Invisible =>
				Color::NONE.into(),
		};
	}
}

impl<U: Component + Default + std::any::Any, M: UIOptionalUniqueIdentifier> WidgetBuilder<U> for BaseButton<U, M>
{
	fn build(&mut self, ui_tree: &mut crate::UIHierarchy<U>, theme: &crate::theme::ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		// Check if M is a Component
		let m_component_check: Box<dyn Reflect> = Box::new(M::default());
		let m_component_check = !m_component_check.represents::<()>();
		let mut parent_data = parent_data;
		if m_component_check
		{
			let mut ui_tree = ui_tree.0.lock().unwrap();
			// Update the tree
			if parent_data.parent_ui_owner.is_none()
			{
				// If the parent UI Owner is None, then we need to add a new node to the tree.
				ui_tree.new_node(M::default().type_id());
				parent_data.parent_ui_owner = Some(U::default().type_id().into());
			}
			let parent_node_typeid = parent_data.parent_ui_owner.unwrap_or(U::default().type_id().into()).0;
			let parent_node = ui_tree
				.iter()
				.filter(|node| !node.is_removed())
				.find(|node| *node.get() == parent_node_typeid)
				.expect("Parent node not found in the UI Tree.");
			let parent_node = ui_tree.get_node_id(parent_node).expect("Parent node not found in the UI Tree.");
			let new_node = ui_tree.new_node(M::default().type_id());
			parent_node.append(new_node, &mut ui_tree);
// Update the ParentData
			parent_data.parent_ui_owner = crate::UIOwner(M::default().type_id()).into();
			println!("Parent UI Owner: {:?}", parent_data.parent_ui_owner);
		}
		// Apply theming.
		self.apply_theme(parent_data.resolve_theme(), theme);

		// Build children.
		let new_parent_data = parent_data.from_current(self.theme);
		let children: Vec<Entity> = self.children.iter_mut().map(|child| child.build(ui_tree, theme, new_parent_data, commands)).collect();

		let mut button = commands.spawn(self.button_bundle.clone());
		if let Some(aspect_ratio) = self.aspect_ratio
		{
			button.insert(AspectRatio(aspect_ratio));
		}
		button
			.insert(U::default())
			.insert(AutoStyledButton)
			.insert(CurrentTheme(self.theme, std::marker::PhantomData::<U>))
			.insert(Focusable::default())
			.push_children(&children)
			;

		if m_component_check
		{
				let m_component: Box<dyn Reflect> = Box::new(M::default());
				use bevy::ecs::reflect::ReflectCommandExt;
				button.insert_reflect(m_component);
				// Also insert it as an UIOwner
				let ui_owner = crate::UIOwner(M::default().type_id());
				button.insert(ui_owner);

			// else { panic!("M is a Component, but it's not a Reflect. This is not supported."); }
		}
		else
		{
			// Otherwise inherit the parent's UIOwner.
			let default_owner = crate::UIOwner(U::default().type_id());
			let owner = parent_data.parent_ui_owner.unwrap_or(default_owner);
			button.insert(owner);
			parent_data.parent_ui_owner = Some(owner);
		}


		if self.auto_style
			{ button.insert(AutoStyledButton); }
		button.id()
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Into<Box<dyn WidgetBuilder<U>>> for BaseButton<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
