use bevy::prelude::Component;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub enum MenuAction {
	StartGame,
}
