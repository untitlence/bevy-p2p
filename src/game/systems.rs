use super::prelude::*;
use crate::shared::prelude::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
	commands.spawn((Camera2dBundle::default(), Game));

	commands.spawn((ui::row(), Game)).with_children(|parent| {
		parent
			.spawn((ui::button(), ButtonAction::Leave))
			.with_children(|parent| {
				parent.spawn(ui::text("Leave", 16.));
			});
	});
}

pub fn button_action<GameState>(
	interaction_query: Query<
		(&Interaction, &ButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	menu_game_state: Res<MenuGameState<GameState>>,
	mut game_state: ResMut<NextState<GameState>>,
) where
	GameState: States + Copy,
{
	for (interaction, menu_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_action {
				ButtonAction::Leave => {
					game_state.set(menu_game_state.0);
				}
			}
		}
	}
}
