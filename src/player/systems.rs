use bevy::prelude::*;
use super::components::{Player, Velocity};

pub fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Player::default(),
        Velocity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.4, 0.5))),
    ));
}

pub fn player_move(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * player.speed * time.delta_secs();
        }
    }
}

pub fn take_damage(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player, &Transform)>,
    enemy_query: Query<&Transform, (With<crate::enemy::components::Enemy>, Without<Player>)>,
) {
    for (player_entity, mut player, player_transform) in player_query.iter_mut() {
        // let player_pos = commands.get_entity(player_entity).unwrap().get::<Transform>().unwrap().translation;
        
        for enemy_transform in enemy_query.iter() {
            // let distance = 
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < 1.5 {
                player.hp -= 10.0 * 0.016; // 假设每帧扣血
                if player.hp <= 0.0 {
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }
}