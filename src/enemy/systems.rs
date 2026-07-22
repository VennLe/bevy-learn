use bevy::prelude::*;
use super::components::Enemy;

pub fn spawn_enemies(mut commands: Commands) {
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 3.0;
        commands.spawn((
            Enemy::default(),
            Transform::from_xyz(x, 0.5, -8.0),
        ));
    }
}

pub fn enemy_move(
    time: Res<Time>,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    for (enemy, mut transform) in query.iter_mut() {
        transform.translation.z += enemy.speed * time.delta_secs();
        if transform.translation.z > 8.0 {
            transform.translation.z = -8.0;
        }
    }
}