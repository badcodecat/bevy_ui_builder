use bevy::prelude::*;
use bevy::ui::FlexDirection;

fn main()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.run();
}

fn setup
(
	mut commands: Commands,
)
{
	commands.spawn(Camera2dBundle::default());

	let root_column = commands.spawn
	(
		NodeBundle
		{
			style: Style
			{
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
		..Default::default()
		}
	).id();

	let node1 = commands.spawn
	(
		NodeBundle
		{
			style: Style
			{
				width: Val::Percent(50.0),
				height: Val::Percent(25.0),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			background_color: Color::RED.into(),
			..Default::default()
		}
	).id();

	let node2 = commands.spawn
	(
		NodeBundle
		{
			style: Style
			{
				width: Val::Percent(50.0),
				height: Val::Percent(25.0),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			background_color: Color::GREEN.into(),
			..Default::default()
		}
	).id();

	commands.entity(root_column)
		.push_children(&[node1, node2]);
}
