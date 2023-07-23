use bevy::prelude::*;

use super::*;
use crate::theme::ThemeData;

pub struct Button<U>
	where U: Component + Default
{
	pub container: Container<U>,
	pub label: TextLabel<U>
}

