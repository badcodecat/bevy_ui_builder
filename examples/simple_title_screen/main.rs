use bevy::prelude::*;

pub mod menu;

#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationState
{
	#[default]
	Menu,
	Game
}

#[derive(Resource)]
pub struct PlayerName(pub String);

fn main()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.init_state::<ApplicationState>()
		.add_plugins(menu::MenuPlugin)
		.add_systems
		(
			OnEnter(ApplicationState::Game),
			start_game
		)
		.run();
}

/// Placeholder function for starting the game.
fn start_game(name: Res<PlayerName>)
{
	println!("Starting game with name: {}", name.0);
}
