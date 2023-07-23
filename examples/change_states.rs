use bevy::prelude::*;
use bevy_ui_builder::{prelude::*, theme::CurrentTheme};

#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationState
{
	#[default]
	Startup,
	Other
}

#[derive(Default, Component)]
pub struct MyUI;

fn main()
{
	println!("Press space to switch states");
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state::<ApplicationState>()
		.add_plugins
		(
			bevy_ui_builder::UIBuilderPlugin::<MyUI, _>::new(ApplicationState::Startup)
				.register_root_builder(build_root)
		)
		.add_systems(Update, change_state_to_other.run_if(in_state(ApplicationState::Startup)))
		.add_systems(Update, change_state_to_startup.run_if(in_state(ApplicationState::Other)))
		.run();
}

fn change_state_to_other(mut state: ResMut<NextState<ApplicationState>>, keyboard_input: ResMut<Input<KeyCode>>)
{
	if keyboard_input.just_pressed(KeyCode::Space)
	{
		state.set(ApplicationState::Other);
		println!("Switched to other state")
	}
}

fn change_state_to_startup(mut state: ResMut<NextState<ApplicationState>>, keyboard_input: ResMut<Input<KeyCode>>)
{
	if keyboard_input.just_pressed(KeyCode::Space)
	{
		state.set(ApplicationState::Startup);
		println!("Switched to startup state")
	}
}

fn build_root(mut commands: Commands, theme: Res<CurrentTheme<MyUI>>)
{
	commands.spawn(Camera2dBundle::default())
		.insert(MyUI);
	let column = bevy_ui_builder::widgets::Column::<MyUI>::new();
	let node1 = bevy_ui_builder::widgets::Column::new()
		.with_colour(Color::RED);
	let node2 = bevy_ui_builder::widgets::Column::new()
		.with_colour(Color::GREEN);
	let column = column
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(node1)
		.push(node2)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.with_fill_portion(2f32);
	bevy_ui_builder::widgets::Row::new()
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(column)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.build(&theme.0, Theme::Background, &mut commands)
		;
}
