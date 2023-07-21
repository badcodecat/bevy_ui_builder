use std::{collections::HashMap, any::TypeId, marker::PhantomData};
use bevy::{ prelude::*, ecs::system::BoxedSystem };

pub mod prelude;
pub mod widgets;
pub mod theme;


pub struct UIBuilderPlugin<D: Component>
{
	pub theme: theme::ThemePallete,
	pub builders: HashMap<TypeId, BoxedSystem>,
	pub updaters: HashMap<TypeId, Vec<BoxedSystem>>,
	_d: std::marker::PhantomData<D>,
}

impl<D: Component> UIBuilderPlugin<D>
{
	pub fn new() -> Self
	{
		let mut result = Self
		{
			theme: theme::ThemePallete::default(),
			builders: Default::default(),
			updaters: Default::default(),
			_d: std::marker::PhantomData,
		};
		return result;
	}

	pub fn with_theme(mut self, theme: theme::ThemePallete) -> Self
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

	pub fn update_on<C: Component + Default, M>(mut self, updater: impl IntoSystem<(), (), M>) -> Self
	{

		use std::any::Any;
		let updater = Box::new(IntoSystem::<(), (), M>::into_system(updater));
		let updaters = self.updaters.entry(C::default().type_id()).or_insert_with(|| Vec::new());
		updaters.push(updater);
		self
	}

}
impl<D: Component + Default> Plugin for UIBuilderPlugin<D>
{
	fn build(&self, app: &mut App)
	{
		use std::any::Any;
		let root_component_id = D::default().type_id();
		// Unsafe cast to &mut self
		#[allow(mutable_transmutes)]
		let self_mut = unsafe { std::mem::transmute::<&UIBuilderPlugin<D>, &mut UIBuilderPlugin<D>>(self) };
		let root_builder = self_mut.builders.remove(&root_component_id).unwrap();
		app
			.add_systems(Startup, root_builder)
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
		let mut app = App::new();
		#[derive(Default, Component)]
		pub struct TestUI;
		#[derive(Default, Resource)]
		pub struct TestResource(u8);
		const MAGIC_NUMBER: u8 = 42;
		fn test_insert_resource(mut commands: Commands)
		{
			commands.insert_resource(TestResource(MAGIC_NUMBER));
		}
		let plugin = UIBuilderPlugin::<TestUI>::new()
			.register_builder::<TestUI, _>(test_insert_resource);
		plugin.build(&mut app);
		app.update();
		let test_resource = app.world.get_resource::<TestResource>().expect("TestResource not inserted");
		assert_eq!(test_resource.0, MAGIC_NUMBER);
	}
}
