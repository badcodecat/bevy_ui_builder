use std::{collections::HashMap, any::TypeId, marker::PhantomData};
use bevy::{prelude::*, app::SystemAppConfig, ecs::system::{BoxedSystem, FunctionSystem}};

pub mod prelude;
pub mod widgets;
pub mod theme;


pub struct UIBuilderPlugin<D: Component>
{
	pub theme: theme::ThemePallete,
	pub systems: HashMap<TypeId, BoxedSystem>,
	_d: std::marker::PhantomData<D>,
}

struct SystemWrapper
{
	system: BoxedSystem,
}

impl<D: Component> UIBuilderPlugin<D>
{
	pub fn new() -> Self
	{
		let mut result = Self
		{
			theme: theme::ThemePallete::default(),
			systems: Default::default(),
			_d: std::marker::PhantomData,
		};
		return result;
	}

	pub fn with_theme(mut self, theme: theme::ThemePallete) -> Self
	{
		self.theme = theme;
		self
	}


	pub fn register_builder<C: Component + Default>(mut self, builder: BoxedSystem) -> Self
	{
        use std::any::Any;
		self.systems.insert(C::default().type_id(), builder);
		self
	}

}
	fn test_system(mut commands: Commands)
	{
		unimplemented!()
	}
impl<D: Component + Default> Plugin for UIBuilderPlugin<D>
{
	fn build(&self, app: &mut App)
	{
		use std::any::Any;
		let root_component_id = D::default().type_id();
		// Unsafe cast to &mut self
		#[allow(mutable_transmutes)]
		let mut self_mut = unsafe { std::mem::transmute::<&UIBuilderPlugin<D>, &mut UIBuilderPlugin<D>>(self) };
		let root_builder = self_mut.systems.remove(&root_component_id).unwrap();
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
}
