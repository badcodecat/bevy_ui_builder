// This simple checkbox implementation is just a TextButton with a border, and a aspect ratio of 1f32.

use super::*;

#[derive(Component, Default)]
pub struct CheckBoxState
{
	pub checked: bool,
}

pub fn toggle_checkbox
(
	mut query: Query<(&mut CheckBoxState, &Interaction), Changed<Interaction>>,
)
{
	for (mut state, interaction) in query.iter_mut()
	{
		if *interaction != Interaction::Pressed
			{ continue; }
		state.checked = !state.checked;
	}
}

pub fn handle_checkbox_toggle
(
	query: Query<(&CheckBoxState, Entity), Changed<CheckBoxState>>,
	children_query: Query<&Children>,
	mut text_query: Query<&mut Text>
)
{
	for (state, entity) in query.iter()
	{
		// The hierarchy is: TextButton/BaseButton -> Container -> TextBundle. (-> Text Component)
		let child = children_query.get(entity).unwrap()[0]; // Get the TextButton.
		let child = children_query.get(child).unwrap()[0]; // Get the Container.
		let mut text = text_query.get_mut(child).unwrap(); // Get the Text Component.

		text.sections[0].value = if state.checked { "X" } else { " " }.to_string();
	}
}

pub struct CheckBox<U, M = ()>
	where U: Component + Default, M: UIOptionalUniqueIdentifier
{
	pub text_button: TextButton<U, M>,
	pub initial_checked_state: bool,
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> CheckBox<U, M>
{
	pub fn new() -> Self
	{
		// ! FIXME: Border is not working.
		let mut text_button = TextButton::new(" ")
			.with_aspect_ratio(1f32)
			.with_paint_mode(PaintMode::BackgroundContainer)
			;
		text_button = text_button.with_border(crate::theme::dimensions::LARGE);
		Self
		{
			text_button,
			initial_checked_state: false,
		}
	}

	pub fn with_checked(mut self, checked: bool) -> Self
		{ self.initial_checked_state = checked; self }
}


impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Widget for CheckBox<U, M>
{
	fn with_paint_mode(mut self, paint_mode: PaintMode) -> Self
		{ self.text_button = self.text_button.with_paint_mode(paint_mode); self }
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
		{ self.text_button = self.text_button.with_colour(background, foreground); self }
	fn with_border(mut self, border: UiRect) -> Self
		{ self.text_button = self.text_button.with_border(border); self }
	fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self
		{ self.text_button = self.text_button.with_aspect_ratio(aspect_ratio); self }
	fn with_direction(mut self, direction: FlexDirection) -> Self
		{ self.text_button = self.text_button.with_direction(direction); self }
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
		{ self.text_button = self.text_button.with_wrap(wrap); self }
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
		{ self.text_button = self.text_button.with_align_self(align_self); self }
	fn with_align_content(mut self, align_content: AlignContent) -> Self
		{ self.text_button = self.text_button.with_align_content(align_content); self }
	fn with_padding(mut self, padding: UiRect) -> Self
		{ self.text_button = self.text_button.with_padding(padding); self }
	fn with_margin(mut self, margin: UiRect) -> Self
		{ self.text_button = self.text_button.with_margin(margin); self }
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
		{ self.text_button = self.text_button.with_fill_portion(fill_portion); self }
	fn with_theme(mut self, theme: Theme) -> Self
		{ self.text_button = self.text_button.with_theme(theme); self }
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> WidgetBuilder<U> for CheckBox<U, M>
{
	fn build(&mut self, theme_data: &crate::theme::ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		// Apply the initial checked state.
		// TODO: This code is ugly, can pretty?
		self.text_button.label.label.text.sections[0].value = if self.initial_checked_state { "X" } else { " " }.to_string();

		// Build the button.
		let button_entity = self.text_button.build(theme_data, parent_data, commands);

		// Add the checkbox state.
		commands.entity(button_entity)
			.insert(CheckBoxState
			{
				checked: self.initial_checked_state,
			});
		button_entity
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Into<Box<dyn WidgetBuilder<U>>> for CheckBox<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
