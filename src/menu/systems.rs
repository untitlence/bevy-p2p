use super::prelude::*;
use crate::shared::prelude::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
	// Camera
	commands.spawn((Camera2dBundle::default(), Menu));
	// Screen
	commands
		.spawn((ui::screen(), Menu))
		.with_children(|parent| {
			parent.spawn(ui::text("Welcome", 32.));
			parent
				.spawn((ui::button(), MenuAction::StartGame))
				.with_children(|parent| {
					parent.spawn(ui::text("Start game", 16.));
				});
		});
}

pub fn menu_action<GameState>(
	interaction_query: Query<
		(&Interaction, &MenuAction),
		(Changed<Interaction>, With<Button>),
	>,
	active_game_state: Res<ActiveGameState<GameState>>,
	mut game_state: ResMut<NextState<GameState>>,
) where
	GameState: States + Copy,
{
	for (interaction, menu_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_action {
				MenuAction::StartGame => {
					game_state.set(active_game_state.0);
				}
			}
		}
	}
}
