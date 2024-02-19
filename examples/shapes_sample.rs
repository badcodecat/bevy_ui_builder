use bevy::prelude::*;
use bevy_ui_builder::prelude::*;

#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationState
{
	#[default]
	Startup,
}

#[derive(Default, Component)]
pub struct MyUI;

fn main()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state::<ApplicationState>()
        .add_plugins(UIEventsPlugin)
		.add_plugins
		(
			bevy_ui_builder::UIBuilderPlugin::<MyUI, _>::new(ApplicationState::Startup)
				.register_root_builder(build_root)
		)
		.run();
}

fn build_root(mut commands: Commands, theme: Res<CurrentThemeData<MyUI>>)
{
	commands.spawn(Camera2dBundle::default())
		.insert(MyUI);
	let column = bevy_ui_builder::widgets::Column::<MyUI, ()>::new();
	let node1 = bevy_ui_builder::widgets::Column::<_, ()>::new()
		.with_colour(Color::RED, Color::NONE);
	let node2 = bevy_ui_builder::widgets::Column::<_, ()>::new()
		.with_colour(Color::GREEN, Color::NONE);
	let column = column
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(node1)
		.push(node2)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.with_fill_portion(2f32);
	bevy_ui_builder::widgets::Row::<_, ()>::new()
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(column)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.build(&theme.0, ParentData::default(), &mut commands)
		;
}
