use bevy::prelude::*;
use bevy_ui_builder::prelude::*;

fn main()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.run();
}

fn setup(mut commands: Commands)
{
	commands.spawn(Camera2dBundle::default());
	let column = bevy_ui_builder::widgets::Column::new();
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
		.build(&mut commands);
}
