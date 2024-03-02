use bevy::prelude::*;
use game::*;
use menu::*;
use resources::*;

mod game;
mod menu;
mod resources;
mod shared;

fn main() {
	App::new()
		.init_state::<GameState>()
		.add_plugins((
			DefaultPlugins,
			MenuPlugin::new(GameState::Menu, GameState::Game),
			GamePlugin::new(GameState::Game, GameState::Menu),
		))
		.run();
}
