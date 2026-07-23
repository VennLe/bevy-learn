
use crate::player::components::{PlayerCamera, PlayerLook};

use super::components::{Player, Velocity};
use bevy::{ input::mouse::MouseMotion, prelude::*, window::CursorOptions};

// 生成玩家 + 第一人称相机
pub fn spawn_player(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    cmd.spawn((
        Player::default(),
        PlayerLook::default(),
        Velocity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.6, 1.8, 0.6)))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.6, 0.9))),
    ))
    .with_children(|parent|{
        // 第一人称相机（子节点，绑定在“头部”位置）
        parent.spawn((
            Camera3d::default(),
            PlayerCamera,
            Transform::from_xyz(0.0, 0.8, 0.0),  // 相机在玩家头顶上

            // projection 默认是透视投影，适合fps游戏
        ));
    });
}

// 键盘移动(WASD)
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
            // 移动速度(m/s)
            transform.translation += direction * player.speed * time.delta_secs();
        }
    }
}

// 鼠标视角控制（FPS风格）
pub fn player_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut PlayerLook, &mut Transform), With<PlayerCamera>>
) { 
    for (mut look, mut transform) in query.iter_mut() {
        for event in mouse_events.read() {
            // 鼠标 × 移动 -> 左右转头（Yaw）
            look.yaw -= event.delta.x * 0.002;
            // 鼠标 Y 移动 -> 上下抬头（Pitch），限制角度防止翻转
            look.pitch -= event.delta.y * 0.002;
            look.pitch = look.pitch.clamp(-1.5, 1.5); // 约±85°

            // 应用旋转：先yaw（绕世界Y轴）， 再Pitch（饶局部X轴）
            transform.rotation = Quat::from_axis_angle(Vec3::Y, look.yaw) * Quat::from_axis_angle(Vec3::X, look.pitch);
        }
    }
}

// 让玩家根实体的 Y 旋转跟随相机的 Yaw（这样 WASD 方向才正确） 
pub fn sync_player_yaw(
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    for cam_transform in camera_query.iter() {
        for mut player_transform in player_query.iter_mut() {
            // 只取相机的 Y 轴旋转，应用到玩家根实体
            let yaw = cam_transform.rotation.to_euler(EulerRot::YXZ).0;
            player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
        }
    }
}

/// 按 ESC 释放/锁定鼠标
pub fn toggle_mouse_lock(
    mut cursors: Query<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    for mut cursor in cursors.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            cursor.visible = false;
            cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
        }
        if key.just_pressed(KeyCode::Escape) {
            cursor.visible = true;
            cursor.grab_mode = bevy::window::CursorGrabMode::None;
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
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 1.5 {
                player.hp -= 10.0 * 0.016; // 假设每帧扣血
                if player.hp <= 0.0 {
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }
}
