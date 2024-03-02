use crate::shared::prelude::*;
use bevy::prelude::*;
use prelude::*;
use std::any::Any;

mod components;
mod resources;
mod systems;

mod prelude {
	pub use super::components::*;
	pub use super::resources::*;
	pub use super::systems::*;
}

#[derive(Debug)]
pub struct GamePlugin<T, A> {
	game_state: T,
	menu_state: A,
}

impl<T, A> GamePlugin<T, A> {
	pub fn new(game_state: T, menu_state: A) -> Self {
		Self {
			game_state,
			menu_state,
		}
	}
}

impl<T, A> Plugin for GamePlugin<T, A>
where
	T: Any + Send + Sync + Copy + States,
	A: Any + Send + Sync + Copy + States,
{
	fn build(&self, app: &mut App) {
		app
			.insert_resource(MenuGameState(self.menu_state))
			.add_systems(OnEnter(self.game_state), setup)
			.add_systems(OnExit(self.game_state), despawn::<Game>)
			.add_systems(
				Update,
				(ui::button_flow, button_action::<A>).run_if(in_state(self.game_state)),
			);
	}
}
