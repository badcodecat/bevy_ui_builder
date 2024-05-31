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
		.add_plugins
		(
			DefaultPlugins
			// Optionally change image filtering mode
			.set(ImagePlugin::default_nearest())
		)
		.init_state::<ApplicationState>()
		.add_plugins(UIEventsPlugin)
		.add_plugins
		(
			bevy_ui_builder::UIBuilderPlugin::<MyUI, _>::new(ApplicationState::Startup)
				.register_root_builder(build_root)
		)
		.register_type::<ExampleTextureButton>()
		.run();
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ExampleTextureButton;

fn build_root
(
	mut commands: Commands,
	mut ui_tree: ResMut<UIHierarchy<MyUI>>,
	theme: Res<CurrentThemeData<MyUI>>,
	asset_server: Res<AssetServer>,
)
{
	commands.spawn(Camera2dBundle::default())
		.insert(MyUI);
	let column = bevy_ui_builder::widgets::Column::<MyUI>::new();

	let example_texture = asset_server.load("Example.png");
	let example_button = bevy_ui_builder::widgets::base_button::BaseButton::<MyUI, ExampleTextureButton>::new()
		.with_image(example_texture.into())
		// Optionally disable auto styling
		// .with_auto_style(false)
		// Or add your own styling
		// .with_focused_image(...)
		// .with_active_image(...)
		;

	let column = column
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(example_button)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.with_fill_portion(2f32);
	bevy_ui_builder::widgets::Row::<_>::new()
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.push(column)
		.push(bevy_ui_builder::widgets::create_space(1f32))
		.build(&mut ui_tree, &theme.0, ParentData::default(), &mut commands)
		;
}
