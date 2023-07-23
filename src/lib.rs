use std::{collections::HashMap, any::TypeId, marker::PhantomData};
use bevy::{ prelude::*, ecs::system::BoxedSystem };

pub mod prelude;
pub mod widgets;
pub mod theme;

pub struct UIStylePlugin;

impl Plugin for UIStylePlugin
{
	fn build(&self, app: &mut App)
	{
		// fn needs_initialization() -> impl Condition<()>
		// {
		// 	IntoSystem::into_system
		// 	(
		// 		|mut commands: Commands, mut entity: Query<(Entity, &NeedsInitialization)>|
		// 		{
		// 			let result = entity.iter().any(|(_, needs_initialization)| needs_initialization.0);
		// 			for (entity, needs_initialization) in entity.iter_mut()
		// 			{
		// 				if needs_initialization.0
		// 				{
		// 					commands.entity(entity).remove::<NeedsInitialization>();
		// 				}
		// 			}
		// 			if result
		// 				{ println!("Needs initialization."); }
		// 			result
		// 		}
		// 	)
		// }
		app
			.add_event::<widgets::label::TextResizeEvent>()
			.add_systems(Update, widgets::label::resize_text)
			.add_systems(Update, widgets::label::resize_text_on_window_resize)
			;
	}
}


pub struct UIBuilderPlugin<D: Component, S: States>
{
	pub theme: theme::ThemeData,
	pub builders: HashMap<TypeId, BoxedSystem>,
	pub updaters: HashMap<TypeId, Vec<BoxedSystem>>,
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
			updaters: Default::default(),
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


	pub fn register_builder<C: Component + Default, M>(mut self, builder: impl IntoSystem<(), (), M>) -> Self
	{
		let builder = Box::new(IntoSystem::into_system(builder));
		use std::any::Any;
		self.builders.insert(C::default().type_id(), builder);
		self
	}

	pub fn register_root_builder<M>(mut self, builder: impl IntoSystem<(), (), M>) -> Self
	{
		let builder = Box::new(IntoSystem::into_system(builder));
		self.builders.insert(TypeId::of::<D>(), builder);
		self
	}

	pub fn update_on<C: Component + Default, M>(mut self, updater: impl IntoSystem<(), (), M>) -> Self
	{

		use std::any::Any;
		let updater = Box::new(IntoSystem::<(), (), M>::into_system(updater));
		let updaters = self.updaters.entry(C::default().type_id()).or_insert_with(|| Vec::new());
		updaters.push(updater);
		self
	}
	// This is a system
	fn destroy_ui_on_exit(mut commands: Commands, mut query: Query<Entity, With<D>>)
	{
		for entity in query.iter_mut()
		{
			commands.entity(entity).despawn_recursive();
		}
	}

}
impl<D: Component + Default, S: States> Plugin for UIBuilderPlugin<D, S>
{
	fn build(&self, app: &mut App)
	{
		use std::any::Any;
		let root_component_id = D::default().type_id();
		// Unsafe cast to &mut self
		#[allow(mutable_transmutes)]
		let self_mut = unsafe { std::mem::transmute::<&UIBuilderPlugin<D, S>, &mut UIBuilderPlugin<D, S>>(self) };
		let root_builder = self_mut.builders.remove(&root_component_id).unwrap();
		app
			.add_systems(OnEnter(self.state.clone()), root_builder)
			.add_systems(OnEnter(self.state.clone()), | mut resize_writer: ResMut<Events<widgets::TextResizeEvent>> | resize_writer.send(widgets::TextResizeEvent))
			.add_systems(OnExit(self.state.clone()), Self::destroy_ui_on_exit)
			.insert_resource(theme::CurrentTheme::<D>(self.theme.clone(), PhantomData))
			;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	/// By simulating a UI Builder system that inserts a resource, we can check if that resource is inserted.
	/// This means that the .build(...) method is still able to mutably access the stored systems.
	#[test]
	fn unsafe_mut_transmute_still_works()
	{
		#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
		pub enum TestApplicationState
		{
			#[default]
			Startup,
		}
		let mut app = App::new();
		app.add_state::<TestApplicationState>();
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
			.register_builder::<TestUI, _>(test_insert_resource);
		plugin.build(&mut app);
		UIStylePlugin.build(&mut app);
		app.add_event::<bevy::window::WindowResized>(); // This is required for the resize_text_on_window_resize system to run.
		app.update();
		let test_resource = app.world.get_resource::<TestResource>().expect("TestResource not inserted");
		assert_eq!(test_resource.0, MAGIC_NUMBER);
	}
}
