// This simple checkbox implementation is just a TextButton with a border, and a aspect ratio of 1f32.

use super::*;

#[derive(Component, Default)]
pub struct CheckBoxState
{
	pub checked: bool,
}

pub fn toggle_checkbox
(
	mut query: Query<(&mut CheckBoxState, &Interaction, Entity), Changed<Interaction>>,
	children_query: Query<&Children>,
	mut text_query: Query<&mut Text>
)
{
	for (mut state, interaction, entity) in query.iter_mut()
	{
		if *interaction != Interaction::Pressed
			{ continue; }
		state.checked = !state.checked;
		// The hierarchy is: This entity -> TextButton -> Container -> TextBundle. (-> Text Component)
		let child = children_query.get(entity).unwrap()[0]; // Get the TextButton.
		let child = children_query.get(child).unwrap()[0]; // Get the Container.
		// let child = children_query.get(child).unwrap()[0]; // Get the TextBundle.
		let mut text = text_query.get_mut(child).unwrap(); // Get the Text Component.

		// let text = text_query.get_many_mut::<1>(children.iter().map(|&entity| entity).collect::<Vec<_>>().as_slice().try_into().unwrap()).unwrap();
		// let [mut text] = text;
		text.sections[0].value = if state.checked { "X" } else { " " }.to_string();
		println!("Checkbox state: {}", state.checked);
	}
}

pub struct CheckBox<U, M>
	where U: Component + Default, M: Component + Default
{
	pub text_button: TextButton<U, M>,
	pub initial_checked_state: bool,
}

impl<U: Component + Default, M: Component + Default> CheckBox<U, M>
{
	pub fn new() -> Self
	{
		// ! FIXME: Border is not working.
		let mut text_button = TextButton::new(" ")
			.with_aspect_ratio(1f32)
			;
		text_button.label = text_button.label.with_border(crate::theme::dimensions::SMALL);
		Self
		{
			text_button,
			initial_checked_state: false,
		}
	}
}


impl<U: Component + Default, M: Component + Default> Widget for CheckBox<U, M>
{
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

impl<U: Component + Default, M: Component + Default> WidgetBuilder<U> for CheckBox<U, M>
{
	fn build(&mut self, theme_data: &crate::theme::ThemeData, parent_theme: Theme, commands: &mut Commands) -> Entity
	{
		// Apply the initial checked state.
		// TODO: This code is ugly, can pretty?
		self.text_button.label.label.text.sections[0].value = if self.initial_checked_state { "X" } else { " " }.to_string();

		// Build the button.
		let button_entity = self.text_button.build(theme_data, parent_theme, commands);

		// Add the checkbox state.
		commands.entity(button_entity)
			.insert(CheckBoxState
			{
				checked: self.initial_checked_state,
			});
		button_entity
	}
}

impl<U: Component + Default, M: Component + Default> Into<Box<dyn WidgetBuilder<U>>> for CheckBox<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
