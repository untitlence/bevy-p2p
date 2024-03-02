use bevy::prelude::*;

#[derive(Resource)]
pub struct ActiveGameState<T>(pub T);
