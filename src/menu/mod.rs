use crate::shared::prelude::*;
use bevy::prelude::*;
use prelude::*;
use std::any::Any;

mod prelude {
	pub use super::components::*;
	pub use super::resources::*;
	pub use super::systems::*;
}

mod components;
mod resources;
mod systems;

#[derive(Debug)]
pub struct MenuPlugin<T, A> {
	menu_state: T,
	game_state: A,
}

impl<T, A> MenuPlugin<T, A> {
	pub fn new(menu_state: T, game_state: A) -> Self {
		Self {
			menu_state,
			game_state,
		}
	}
}

impl<T, A> Plugin for MenuPlugin<T, A>
where
	T: Any + Send + Sync + Copy + States,
	A: Any + Send + Sync + Copy + States,
{
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ClearColor(ui::CLEAR_COLOR))
			.insert_resource(ActiveGameState(self.game_state))
			.add_systems(OnEnter(self.menu_state), setup)
			.add_systems(OnExit(self.menu_state), despawn::<Menu>)
			.add_systems(
				Update,
				(ui::button_flow, menu_action::<A>).run_if(in_state(self.menu_state)),
			);
	}
}
