	use bevy::prelude::*;

	pub const NONE: UiRect = UiRect::new(Val::Px(0f32), Val::Px(0f32), Val::Px(0f32), Val::Px(0f32));
	pub const SMALL: UiRect = UiRect::new(Val::Percent(0.5f32), Val::Percent(0.5f32), Val::Percent(0.5f32), Val::Percent(0.5f32));
	pub const LARGE: UiRect = UiRect::new(Val::Percent(10f32), Val::Percent(10f32), Val::Percent(10f32), Val::Percent(10f32));
