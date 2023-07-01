use std::{collections::HashMap, any::TypeId, marker::PhantomData};
use bevy::{prelude::*, app::SystemAppConfig, ecs::{system::{BoxedSystem, FunctionSystem}, schedule::SystemConfig}};

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

fn boxed_system_from_intosystemconfig<M>(system: impl IntoSystemConfig<M>) -> BoxedSystem
{
	let system = IntoSystemConfig::<(), (), M>::into_system_config(system);
	let system = FunctionSystem::<(), (), M>::from(system);
	let system = BoxedSystem::new(system);
	system
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


	pub fn register_builder<C: Component + Default>(mut self, builder: impl IntoSystem<(), (), fn(&mut App)>) -> Self
	{
        use std::any::Any;
		let builder = Box::new(IntoSystem::<(), (), fn(&mut App)>::into_system(builder));
		self.builders.insert(C::default().type_id(), builder);
		self
	}

	pub fn update_on<C: Component + Default, X>(mut self, updater: impl IntoSystem<(), (), X>) -> Self
	{

        use std::any::Any;
		let updater = Box::new(IntoSystem::<(), (), X>::into_system(updater));
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
			.add_startup_system(root_builder)
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
		let mut plugin = UIBuilderPlugin::<TestUI>::new()
			.register_builder::<TestUI>(test_insert_resource);
		plugin.build(&mut app);
	}
}
