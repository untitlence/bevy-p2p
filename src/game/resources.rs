use bevy::prelude::*;

#[derive(Resource)]
pub struct MenuGameState<T>(pub T);
