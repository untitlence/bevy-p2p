use bevy::prelude::*;
use menu::*;
use resources::*;

mod menu;
mod resources;
mod shared;

fn main() {
	App::new()
		.init_state::<GameState>()
		.add_plugins((DefaultPlugins, MenuPlugin::new(GameState::Menu)))
		.run();
}
