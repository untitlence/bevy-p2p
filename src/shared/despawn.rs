use crate::*;

pub fn despawn<T: Component>(
	to_despawn: Query<Entity, With<T>>,
	mut commands: Commands,
) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}
