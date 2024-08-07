use std::marker::PhantomData;

// Really a TextInput is just a label with extra steps
use bevy_alt_ui_navigation_lite::prelude::*;

use crate::theme::ThemeData;

use super::*;

#[derive(Component, Default)]
pub struct EditableText
{
	pub text: String,
}

#[derive(Component, Default)]
pub struct EditCursor
{
	pub position: usize,
}

#[derive(Component, Default)]
pub struct PlaceholderText
{
	pub text: String,
}

#[derive(Component, Default)]
pub struct AllowsNewlines;

pub fn update_text_sections
(
	mut query: Query<(&Children, &EditableText, Option<&PlaceholderText>), Changed<EditableText>>,
	mut text_query: Query<&mut Text>,
)
{
	for (children, text, placeholder) in query.iter_mut()
	{
		let mut text_bundle = text_query.get_mut(children[0]).unwrap();
		text_bundle.sections[0].value = text.text.clone();
		if let Some(placeholder) = placeholder
		{
			if text.text.is_empty()
			{
				text_bundle.sections[0].value = placeholder.text.clone();
				continue;
			}
		}
	}
}

// TODO: Support IME
pub fn handle_text_input
(
	mut query: Query<(&mut EditableText, &mut EditCursor, &Focusable, Option<&AllowsNewlines>)>,
	mut text_input: EventReader<ReceivedCharacter>,
	keyboard_input: ResMut<ButtonInput<KeyCode>>,
)
{
	for (mut text, mut cursor, focusable, allows_newlines) in query.iter_mut()
	{
		if focusable.state() != FocusState::Focused
			{ continue; }
		if keyboard_input.just_pressed(KeyCode::Backspace)
		{
			if cursor.position > 0
			{
				text.text.remove(cursor.position - 1);
				cursor.position -= 1;
			}
		}
		else if keyboard_input.just_pressed(KeyCode::Delete)
		{
			if cursor.position < text.text.len()
			{
				text.text.remove(cursor.position);
			}
		}
		else if keyboard_input.just_pressed(KeyCode::ArrowLeft)
		{
			if cursor.position > 0
			{
				cursor.position -= 1;
			}
		}
		else if keyboard_input.just_pressed(KeyCode::ArrowRight)
		{
			if cursor.position < text.text.len()
			{
				cursor.position += 1;
			}
		}
		else if keyboard_input.just_pressed(KeyCode::Enter)
		{
			if allows_newlines.is_some()
			{
				text.text.insert(cursor.position, '\n');
				cursor.position += 1;
			}
		}
		else if keyboard_input.just_pressed(KeyCode::Home)
		{
			cursor.position = 0;
		}
		else if keyboard_input.just_pressed(KeyCode::End)
		{
			cursor.position = text.text.len();
		}
		for event in text_input.read()
		{
			// if event.char.chars().count() != 1
			// {
			// 	warn!("ReceivedCharacter event contained more than one character, skipping.");
			// 	continue;
			// }
			// let character : char = event.char.chars().next().expect("ReceivedCharacter event contained no characters.");
			for character in event.char.chars()
			{
				if character.is_control()
					{ continue; }
				text.text.insert(cursor.position, character);
				cursor.position += 1;
			}
		}
	}
}


pub struct TextInput<U: Component + Default, M: UIOptionalUniqueIdentifier = ()>
{
	pub label: TextLabel<U, M>,
	pub placeholder: Option<String>,
	pub allows_newlines: bool,
	phantom: PhantomData<M>,
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> TextInput<U, M>
{
	pub fn new(text: Option<String>) -> Self
	{
		Self
		{
			label: TextLabel::new(text.clone().unwrap_or_default())
				.with_border(UiRect::all(Val::Percent(3f32)))
				,
			placeholder: text,
			allows_newlines: false,
			phantom: PhantomData,
		}
	}

	pub fn allows_newlines(mut self, allows_newlines: bool) -> Self
	{
		self.allows_newlines = allows_newlines;
		self
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Widget for TextInput<U, M>
{
	fn with_paint_mode(mut self, paint_mode: PaintMode) -> Self
	{
		self.label = self.label.with_paint_mode(paint_mode);
		self
	}
	fn with_colour(mut self, background: Color, foreground: Color) -> Self
	{
		self.label = self.label.with_colour(background, foreground);
		self
	}
	fn with_border(mut self, border: UiRect) -> Self
	{
		self.label = self.label.with_border(border);
		self
	}
	fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self
	{
		self.label = self.label.with_aspect_ratio(aspect_ratio);
		self
	}
	fn with_direction(mut self, direction: FlexDirection) -> Self
	{
		self.label = self.label.with_direction(direction);
		self
	}
	fn with_wrap(mut self, wrap: FlexWrap) -> Self
	{
		self.label = self.label.with_wrap(wrap);
		self
	}
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
	{
		self.label = self.label.with_align_self(align_self);
		self
	}
	fn with_align_content(mut self, align_content: AlignContent) -> Self
	{
		self.label = self.label.with_align_content(align_content);
		self
	}
	fn with_padding(mut self, padding: UiRect) -> Self
	{
		self.label = self.label.with_padding(padding);
		self
	}
	fn with_margin(mut self, margin: UiRect) -> Self
	{
		self.label = self.label.with_margin(margin);
		self
	}
	fn with_fill_portion(mut self, fill_portion: f32) -> Self
	{
		self.label = self.label.with_fill_portion(fill_portion);
		self
	}
	fn with_theme(mut self, theme: Theme) -> Self
	{
		self.label = self.label.with_theme(theme);
		self
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> WidgetBuilder<U> for TextInput<U, M>
{
	fn build(&mut self, ui_tree: &mut crate::UIHierarchy<U>, theme_data: &ThemeData, parent_data: ParentData, commands: &mut Commands) -> Entity
	{
		let entity = self.label.build(ui_tree, theme_data, parent_data, commands);
		let mut entity = commands.entity(entity);
		entity
			.insert(Focusable::default())
			.insert(EditableText::default())
			.insert(EditCursor::default())
			;
		if let Some(placeholder) = &self.placeholder
		{ entity.insert(PlaceholderText { text: placeholder.clone() }); }
		if self.allows_newlines
		{ entity.insert(AllowsNewlines); }
		entity.id()
	}
}

impl<U: Component + Default, M: UIOptionalUniqueIdentifier> Into<Box<dyn WidgetBuilder<U>>> for TextInput<U, M>
{
	fn into(self) -> Box<dyn WidgetBuilder<U>>
	{
		Box::new(self)
	}
}
