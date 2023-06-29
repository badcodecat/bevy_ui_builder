use std::{collections::HashMap, any::TypeId};
use bevy::{prelude::*, app::SystemAppConfig, ecs::system::BoxedSystem};

pub mod prelude;
pub mod widgets;
pub mod theme;


pub struct UIBuilderPlugin<D: Component>
{
	pub theme: theme::ThemePallete,
	pub systems: HashMap<TypeId, BoxedSystem>,
	_d: std::marker::PhantomData<D>,
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

	pub fn register_builder<C: Component + Default, B>(mut self, builder: impl IntoSystemAppConfig<B>) -> Self
	{
        use std::any::Any;
		self.systems.insert(C::default().type_id(), builder);
		self
	}

}

impl<D: Component + Default> Plugin for UIBuilderPlugin<D>
{
	fn build(&self, app: &mut App)
	{
		use std::any::Any;
		let root_component_id = D::default().type_id();
		let root_builder = self.systems.get(&root_component_id).unwrap();
		app
			.add_startup_system(<&SystemAppConfig as Into<IntoSystemAppConfig>>::into(root_builder))
			;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
}
