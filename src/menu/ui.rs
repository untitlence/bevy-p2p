use bevy::prelude::*;

pub const CLEAR_COLOR: Color = Color::rgb(0.067, 0.071, 0.075);
pub const BORDER_COLOR: Color = Color::rgb(0.314, 0.318, 0.322);
pub const PAPER_COLOR: Color = Color::rgb(0.125, 0.129, 0.137);
pub const HOVERED_COLOR: Color = Color::rgb(0.251, 0.255, 0.259);

pub fn screen() -> NodeBundle {
	NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			row_gap: Val::Px(8.),
			flex_direction: FlexDirection::Column,
			..default()
		},
		..default()
	}
}

pub fn button() -> ButtonBundle {
	ButtonBundle {
		style: Style {
			border: UiRect::all(Val::Px(2.)),
			height: Val::Px(30.),
			padding: UiRect::axes(Val::Px(8.), Val::Px(2.)),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			..default()
		},
		border_color: BORDER_COLOR.into(),
		background_color: PAPER_COLOR.into(),
		..default()
	}
}

pub fn text(text: &str, font_size: f32) -> TextBundle {
	TextBundle::from_section(
		text,
		TextStyle {
			font_size,
			..default()
		}
	)
}
