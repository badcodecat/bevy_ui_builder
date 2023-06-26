use bevy::prelude::*;
use bevy::ui::FlexDirection;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.run();
}

fn setup
(
	mut commands: Commands,
)
{
	commands.spawn(Camera2dBundle::default());

	let root = commands.spawn
	(
		NodeBundle
		{
			style: Style
			{
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
				size: Size::new(Val::Px(200.0), Val::Px(100.0)),
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
				size: Size::new(Val::Px(150.0), Val::Px(75.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
        background_color: Color::GREEN.into(),
		..Default::default()
		}
	).id();

	commands.entity(root)
		.push_children(&[node1, node2]);
}
