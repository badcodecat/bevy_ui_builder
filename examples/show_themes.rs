use bevy::prelude::*;
use bevy_ui_builder::prelude::*;

#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationState
{
	#[default]
	Menu,
}

fn main()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state::<ApplicationState>()
		.add_plugins(UIEventsPlugin)
		.add_plugins
		(
			UIBuilderPlugin::<MyUI, _>::new(ApplicationState::Menu)
				// Change here to see the different themes
				.with_theme(bevy_ui_builder::theme::themes::LIGHT.clone())
				.register_root_builder(build_root)
		)
		.run();
}

#[derive(Default, Component)]
pub struct MyUI;

fn build_root(mut commands: Commands, theme: Res<CurrentThemeData<MyUI>>)
{
	commands.spawn(Camera2dBundle::default())
		.insert(MyUI);
	let root = bevy_ui_builder::widgets::Row::<MyUI>::new()
		.with_padding(bevy_ui_builder::theme::dimensions::LARGE);
	let column_primary = bevy_ui_builder::widgets::Column::new()
		.with_theme(Theme::Primary)
		.with_padding(bevy_ui_builder::theme::dimensions::LARGE)
		.push(build_sample_widgets())
		;
	let column_secondary = bevy_ui_builder::widgets::Column::new()
		.with_theme(Theme::Secondary)
		.with_padding(bevy_ui_builder::theme::dimensions::LARGE)
		.push(build_sample_widgets())
		;
	let column_tertiary = bevy_ui_builder::widgets::Column::new()
		.with_theme(Theme::Tertiary)
		.with_padding(bevy_ui_builder::theme::dimensions::LARGE)
		.push(build_sample_widgets())
		;
	let column_secondary = column_secondary
		.push(column_tertiary)
		;
	let mut root = root
		.push(build_sample_widgets())
		.push(column_primary)
		.push(column_secondary)
		;
	root.build(&theme.0, ParentData::default(), &mut commands);
}

fn build_sample_widgets() -> impl Into<Box<dyn WidgetBuilder<MyUI>>>
{
	let text_label = bevy_ui_builder::widgets::TextLabel::new("Text Label");
	let text_input = bevy_ui_builder::widgets::TextInput::<MyUI, MyUI>::new("Text Input".to_string().into());
	let text_button = bevy_ui_builder::widgets::TextButton::<MyUI, MyUI>::new("Text Button");
	let checkbox = bevy_ui_builder::widgets::CheckBox::<MyUI, MyUI>::new();
	bevy_ui_builder::widgets::Column::new()
		.push(text_label)
		.push(text_input)
		.push(text_button)
		.push(checkbox)
}
