use bevy::prelude::*;

#[derive(Component)]
struct WelcomeText;

fn setup(mut commands: Commands) {
	commands.insert_resource(ClearColor(Color::rgb_u8(0x10, 0x11, 0x12)));
	commands.spawn(Camera2dBundle::default());

	commands
		.spawn((NodeBundle {
			style: Style {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				flex_direction: FlexDirection::Column,
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				row_gap: Val::Px(20.),
				..default()
			},
			..default()
		},))
		.with_children(|parent| {
			parent.spawn((
				TextBundle::from_section(
					"Welcome",
					TextStyle {
						font_size: 20.,
						..default()
					},
				),
				WelcomeText,
			));
			parent
				.spawn(ButtonBundle {
					style: Style {
						width: Val::Px(160.),
						height: Val::Px(30.),
						border: UiRect::all(Val::Px(2.)),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						..default()
					},
					background_color: Color::rgb_u8(0x20, 0x21, 0x22).into(),
					border_color: Color::rgb_u8(0x50, 0x51, 0x52).into(),
					..default()
				})
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Create host game",
						TextStyle {
							color: Color::WHITE,
							font_size: 16.,
							..default()
						},
					));
				});
			parent
				.spawn(ButtonBundle {
					style: Style {
						width: Val::Px(160.),
						height: Val::Px(30.),
						border: UiRect::all(Val::Px(2.)),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						..default()
					},
					background_color: Color::rgb_u8(0x20, 0x21, 0x22).into(),
					border_color: Color::rgb_u8(0x50, 0x51, 0x52).into(),
					..default()
				})
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Join",
						TextStyle {
							color: Color::WHITE,
							font_size: 16.,
							..default()
						},
					));
				});
		});
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.run();
}
