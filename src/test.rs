// Tests and utilities for testing the library.

use bevy::prelude::*;

pub struct PretendWindowPlugin;

impl Plugin for PretendWindowPlugin
{
	fn build(&self, app: &mut App)
	{
		app
			.add_event::<bevy::window::WindowResized>() // This is required for the resize_text_on_window_resize system to run.
			.add_event::<bevy::window::CursorMoved>()
			.add_event::<bevy::input::touch::TouchInput>()
			.add_event::<bevy::input::mouse::MouseButtonInput>()
			.init_resource::<bevy::input::Input<bevy::input::keyboard::KeyCode>>()
			.init_resource::<bevy::input::Input<bevy::input::mouse::MouseButton>>()
			.init_resource::<bevy::input::Input<bevy::input::gamepad::GamepadButton>>()
			.init_resource::<bevy::input::Axis<bevy::input::gamepad::GamepadAxis>>()
			.init_resource::<bevy::ecs::event::Events<bevy::window::ReceivedCharacter>>()
			.init_resource::<bevy::ui::UiStack>()
			;
	}
}
