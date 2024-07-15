#![feature(trivial_bounds)]
use std::{any::TypeId, collections::HashMap, marker::PhantomData, sync::Mutex, sync::Arc};
use bevy::{ ecs::{schedule::SystemConfigs, system::BoxedSystem}, prelude::* };
use bevy_alt_ui_navigation_lite::prelude::*;

pub mod prelude;
pub mod widgets;
pub mod theme;
pub mod test;

pub struct UIEventsPlugin;

impl Plugin for UIEventsPlugin
{
	fn build(&self, app: &mut App)
	{
		app
			.add_event::<widgets::AspectRatioEvent>()
			.add_systems
			(
				Update,
				widgets::ensure_aspect_ratio,
			)
			.add_event::<widgets::text_label::TextResizeEvent>()
			.add_systems
			(
				Update,
				(
					widgets::text_label::resize_text,
					widgets::resize_on_window_resize,
					// widgets::resize_on_window_change // System no longer works.
				)
			)
			.add_systems(Update, widgets::base_button::send_pressed_on_keyboard)
			.add_systems
			(
				Update,
				(
					widgets::text_input::handle_text_input,
					widgets::text_input::update_text_sections,
					widgets::checkbox::toggle_checkbox,
					widgets::checkbox::handle_checkbox_toggle
				)
			)
			.add_plugins(DefaultNavigationPlugins)
			.insert_resource
			(
				bevy_alt_ui_navigation_lite::systems::InputMapping
				{
					keyboard_navigation: true,
					focus_follows_mouse: true,
					..Default::default()
				}
			)
			;
	}
}

// This resource describes the UI tree of named elements.

#[derive(Resource)]
pub struct UIHierarchy<U: Component>(pub Arc<Mutex<indextree::Arena<TypeId>>>, pub std::marker::PhantomData<U>);

// unsafe impl<U: Component> Send for UIHierarchy<U> {}
// unsafe impl<U: Component> Sync for UIHierarchy<U> {}

// This component describes the closest named element to the entity.
#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct UIOwner(pub TypeId);

impl From<TypeId> for UIOwner
{
	fn from(type_id: TypeId) -> Self
	{
		Self(type_id)
	}
}

pub struct UIBuilderPlugin<D: Component, S: States>
{
	pub theme: theme::ThemeData,
	pub builders: Mutex<HashMap<TypeId, SystemConfigs>>,
	pub change_detectors: HashMap<TypeId, Vec<BoxedSystem>>,
	pub root_builder: Mutex<Option<SystemConfigs>>,
	pub state: S,
	_d: std::marker::PhantomData<D>,
}

impl<D: Component, S: States> UIBuilderPlugin<D, S>
{
	pub fn new(state: S) -> Self
	{
		let result = Self
		{
			theme: theme::ThemeData::default(),
			builders: Default::default(),
			change_detectors: Default::default(),
			root_builder: None.into(),
			state: state,
			_d: std::marker::PhantomData,
		};
		return result;
	}

	pub fn with_theme(mut self, theme: theme::ThemeData) -> Self
	{
		self.theme = theme;
		self
	}


	pub fn register_builder<C: Component + Default, M>(self, builder: impl IntoSystemConfigs<M>) -> Self
	{
		// let builder = Box::new(IntoSystem::into_system(builder));
		use std::any::Any;
		self.builders.lock().unwrap().insert(C::default().type_id(), builder.into_configs());
		self
	}

	pub fn register_root_builder<M>(self, builder: impl IntoSystemConfigs<M>) -> Self
	{
		// let builder = Box::new(IntoSystem::into_system(builder));
		let mut unlocked_root_builder = self.root_builder.lock().unwrap();
		*unlocked_root_builder = Some(builder.into_configs());
		drop(unlocked_root_builder);
		// self.builders.lock().unwrap().insert(TypeId::of::<D>(), builder.into_configs());
		self
	}

	pub fn update_on<C: Component + Default, M>(mut self, updater: impl IntoSystem<(), (), M>) -> Self
	{
		use std::any::Any;
		let updater = Box::new(IntoSystem::<(), (), M>::into_system(updater));
		let updaters = self.change_detectors.entry(C::default().type_id()).or_insert_with(|| Vec::new());
		updaters.push(updater);
		self
	}

	/// This is a system, not an actual method.
	fn destroy_ui_on_exit(mut commands: Commands, mut query: Query<Entity, With<D>>)
	{
		for entity in query.iter_mut()
		{
			commands.entity(entity).despawn_recursive();
		}
	}

}
impl<D: Component + Default + std::any::Any, S: States> Plugin for UIBuilderPlugin<D, S>
{
	fn build(&self, app: &mut App)
	{
		// use std::any::Any;
		// let root_component_id = D::default().type_id();
		// Unsafe cast to &mut self
		// #[allow(mutable_transmutes)]
		// let self_mut = unsafe { std::mem::transmute::<&UIBuilderPlugin<D, S>, &mut UIBuilderPlugin<D, S>>(self) };
		// #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
		// pub enum BuildSet
		// {
		// 	Building,
		// 	Built,
		// }
		#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Resource)]
		pub struct ResizeLocal<T>(pub u8, pub PhantomData<T>);
		// let root_builder = self_mut.builders.remove(&root_component_id).unwrap();
		let mut unlocked_builders = self.builders.lock().unwrap();
		// let root_builder = unlocked_builders.remove(&root_component_id).unwrap();
		let root_builder = self.root_builder.lock().unwrap().take().unwrap();
		let mut ui_tree = indextree::Arena::new();
		ui_tree.new_node(D::default().type_id());
		app
			.add_systems(OnEnter(self.state.clone()), root_builder.into_configs())
			// Insert the UIHierarchy resource.
			.insert_resource(UIHierarchy::<D>(Arc::new(Mutex::new(ui_tree)), PhantomData))
			.add_systems
			(
				OnEnter(self.state.clone()),
				| mut commands: Commands|
				{
					commands.insert_resource(ResizeLocal::<D>(0, PhantomData));
				}
			)
			.add_systems
			(
				Update,
				(
					| mut aspect_writer: EventWriter<widgets::AspectRatioEvent>, mut resize_writer: ResMut<Events<widgets::TextResizeEvent>>, mut commands: Commands, mut resize_local: ResMut<ResizeLocal<D>>|
					{
						// Run 2 times to ensure that the text is resized.
						if resize_local.0 >= 1
						{
							commands.remove_resource::<ResizeLocal<D>>();
						}
						aspect_writer.send(widgets::AspectRatioEvent);
						resize_writer.send(widgets::TextResizeEvent);
						resize_local.0 += 1;
					}
				)
					.run_if(resource_exists::<ResizeLocal::<D>>)
			)
			.add_systems
			(
				Update,
				(
					widgets::base_button::style_button_on_focus::<D>,
					widgets::base_button::style_button_on_pressed::<D>,
				)
					.run_if(in_state(self.state.clone()))
					.after(NavRequestSystem)
			)
			.add_systems(OnExit(self.state.clone()), Self::destroy_ui_on_exit)
			.insert_resource(theme::CurrentThemeData::<D>(self.theme.clone(), PhantomData))
			;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	/// By simulating a UI Builder system that inserts a resource, we can check if that resource is inserted.
	/// This means that the .build(...) method is still able to mutably access the stored systems. (Via a mutex now)
	#[test]
	fn adding_systems_works()
	{
		#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
		pub enum TestApplicationState
		{
			#[default]
			Startup,
		}
		let mut app = App::new();
		test::PretendWindowPlugin.build(&mut app); // This is so we don't get unrelated panics
		bevy::state::app::StatesPlugin.build(&mut app); // States are no longer a default part of bevy, so we need to add it manually (bevy 0.14)
		app.init_state::<TestApplicationState>();
		#[derive(Default, Component)]
		pub struct TestUI;
		#[derive(Default, Resource)]
		pub struct TestResource(u8);
		const MAGIC_NUMBER: u8 = 42;
		fn test_insert_resource(mut commands: Commands)
		{
			commands.insert_resource(TestResource(MAGIC_NUMBER));
		}
		let plugin = UIBuilderPlugin::<TestUI, _>::new(TestApplicationState::Startup)
			.register_root_builder(test_insert_resource);
		plugin.build(&mut app);
		UIEventsPlugin.build(&mut app);
		app.update();
		let test_resource = app.world_mut().get_resource::<TestResource>().expect("TestResource not inserted");
		assert_eq!(test_resource.0, MAGIC_NUMBER);
	}
}
