use bevy::prelude::*;
use bevy_ui_builder::{prelude::*, theme::CurrentTheme};

#[derive(Default, Component)]
pub struct MyUI;

fn main()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins
		(
			bevy_ui_builder::UIBuilderPlugin::<MyUI>::new()
				.register_builder::<MyUI, _>(setup)
		)
		.run();
}

fn setup(mut commands: Commands, theme: Res<CurrentTheme<MyUI>>)
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
		.build(&theme.0, &mut commands)
		;
}
