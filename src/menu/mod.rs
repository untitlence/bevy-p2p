use crate::shared::prelude::*;
use bevy::prelude::*;
use prelude::*;
use std::any::Any;

mod prelude {
	pub use super::components::*;
	pub use super::systems::*;
	pub use super::ui::*;
}

mod components;
mod systems;
mod ui;

#[derive(Debug)]
pub struct MenuPlugin<T> {
	on_state: T,
}

impl<T> MenuPlugin<T> {
	pub fn new(on_state: T) -> Self {
		Self { on_state }
	}
}

impl<T> Plugin for MenuPlugin<T>
where
	T: Any + Send + Sync + Copy + States,
{
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ClearColor(CLEAR_COLOR))
			.add_systems(OnEnter(self.on_state), setup)
			.add_systems(OnExit(self.on_state), despawn::<Menu>)
			.add_systems(Update, button_flow);
	}
}
