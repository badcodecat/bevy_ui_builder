use bevy::prelude::*;
use bevy_ui_builder::prelude::*;

use crate::ApplicationState;
use crate::PlayerName;

pub struct MenuPlugin;

impl Plugin for MenuPlugin
{
	fn build(&self, app: &mut App)
	{
		app
		.add_plugins(UIEventsPlugin)
		.add_plugins
		(
			UIBuilderPlugin::<MyUI, _>::new(ApplicationState::Menu)
				.register_root_builder(build_root)
		)
		.add_systems
		(
			Update,
			(
				play_on_press,
				quit_on_press,
			)
				.run_if(in_state(ApplicationState::Menu))
		)
		;
	}
}


#[derive(Default, Component)]
pub struct MyUI;

#[derive(Component, Default)]
pub struct PlayButton;

#[derive(Component, Default)]
pub struct NameInput;

#[derive(Component, Default)]
pub struct QuitButton;


fn build_root(mut commands: Commands, theme: Res<CurrentThemeData<MyUI>>)
{
	commands.spawn(Camera2dBundle::default())
		.insert(MyUI);
	let column = bevy_ui_builder::widgets::Column::<MyUI>::new()
		.with_fill_portion(3f32);
	let title = bevy_ui_builder::widgets::TextLabel::new("My Awesome Game")
		;
	let space = bevy_ui_builder::widgets::create_space(1f32);
	let play_button = bevy_ui_builder::widgets::TextButton::<_, PlayButton>::new("Play");
	let name_input = bevy_ui_builder::widgets::TextInput::<MyUI, NameInput>::new("Enter your name".to_string().into());
	let quit_button = bevy_ui_builder::widgets::TextButton::<_, QuitButton>::new("Quit");
	let column = column
		.push(title)
		.push(space)
		.push(name_input)
		.push(play_button)
		.push(quit_button)
		.push(bevy_ui_builder::widgets::create_space(3f32))
		;
	bevy_ui_builder::widgets::Row::new()
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(column)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.build(&theme.0, ParentData::default(), &mut commands)
		;
}

fn play_on_press
(
	mut commands: Commands,
	interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
	name_query: Query<&EditableText, With<NameInput>>,
	mut state: ResMut<NextState<ApplicationState>>
)
{
	if let Ok(Interaction::Pressed) = interaction_query.get_single()
	{
		let name = name_query.single().text.clone();
		commands.insert_resource(PlayerName(name));
		state.set(ApplicationState::Game);
	}
}


fn quit_on_press(interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>)
{
	if let Ok(Interaction::Pressed) = interaction_query.get_single()
	{
		app_exit_events.send(bevy::app::AppExit);
	}
}
