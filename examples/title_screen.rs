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
		.add_plugins(UIStylePlugin)
		.add_plugins
		(
			UIBuilderPlugin::<MyUI, _>::new(ApplicationState::Startup)
				.register_root_builder(build_root)
		)
		.run();
}

fn build_root(mut commands: Commands, theme: Res<CurrentTheme<MyUI>>)
{
	commands.spawn(Camera2dBundle::default())
		.insert(MyUI);
	let column = bevy_ui_builder::widgets::Column::<MyUI>::new()
		.with_fill_portion(3f32);
	let title = bevy_ui_builder::widgets::TextLabel::<MyUI>::new("My Awesome Game")
		;
	let space = bevy_ui_builder::widgets::create_space(4f32);
	let column = column
		.push(title)
		.push(space)
		;
	bevy_ui_builder::widgets::Row::new()
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(column)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.build(&theme.0, Theme::Auto, &mut commands)
		;
}