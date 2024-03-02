use super::prelude::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
	// Camera
	commands.spawn((Camera2dBundle::default(), Menu));
	// Screen
	commands.spawn((screen(), Menu)).with_children(|parent| {
		parent.spawn(text("Welcome", 32.));
		parent.spawn(button()).with_children(|parent| {
			parent.spawn(text("Start game", 16.));
		});
	});
}

pub fn button_flow(
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>),
	>,
) {
	for (interaction, mut color) in &mut interaction_query {
		use Interaction::*;
		*color = match *interaction {
			Hovered | Pressed => HOVERED_COLOR.into(),
			None => PAPER_COLOR.into(),
		}
	}
}
