use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorOptions};
use bevy_rapier3d::prelude::*;
use super::components::{Player, PlayerCamera, PlayerLook, AttachedToPlayer};

/// 生成玩家 + 第一人称相机（独立实体）
pub fn spawn_player(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. 生成玩家身体（物理身体，用于碰撞和被敌人看见）
    let player_entity = cmd.spawn((
        Player::default(),
        PlayerLook::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),

        // 物理身体组件
        RigidBody::Dynamic,  // 动态刚体，受物理影响
        LockedAxes::ROTATION_LOCKED,  // 锁定旋转，防止玩家摔倒

         // 碰撞体：胶囊体更适合人形角色
        Collider::capsule_y(0.9, 0.3),  // 高度 1.8，半径 0.3
        
        // 物理材质
        Friction::new(0.5),
        Restitution::new(0.0),  // 无弹性
        
        // 速度控制
        Velocity::default(),

        // 渲染实体
        Mesh3d(meshes.add(Cylinder::new(0.6, 1.8))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.6, 0.9))),
    )).id();
    
    // 2. 生成相机（独立实体，不父子绑定）
    cmd.spawn((
        Camera3d::default(),
        PlayerCamera,
        PlayerLook::default(),
        Transform::from_xyz(0.0, 1.8, 0.0),
        AttachedToPlayer { target: player_entity },
    ));
}

/// 合并：同步相机位置到玩家头部 + 同步玩家 Yaw 跟随相机
pub fn sync_player_and_camera(
    mut camera_query: Query<(&AttachedToPlayer, &mut Transform), (With<PlayerCamera>, Without<Player>)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    // 先处理相机位置同步
    for (attachment, mut camera_transform) in camera_query.iter_mut() {
        if let Ok(player_transform) = player_query.get(attachment.target) {
            // 相机位置 = 玩家位置 + 头部偏移（玩家身高 1.8，相机在 1.6 高度）
            camera_transform.translation = player_transform.translation + Vec3::new(0.0, 1.6, 0.0);
        }
    }
    
    // 再处理玩家 Yaw 同步
    let camera_yaw = camera_query.iter().next()
        .map(|(_, transform)| transform.rotation.to_euler(EulerRot::YXZ).0);
    
    if let Some(yaw) = camera_yaw {
        for mut player_transform in player_query.iter_mut() {
            player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
        }
    }
}

/// 鼠标视角控制 - 直接控制相机旋转
pub fn player_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut PlayerLook, &mut Transform), With<PlayerCamera>>,
    mut cursors: Query<&CursorOptions> // 查询窗口状态
) {
      // 检查鼠标是否锁定
    let mouse_locked = cursors.iter().next()
        .map(|cursor| cursor.grab_mode != bevy::window::CursorGrabMode::None)
        .unwrap_or(false);
    
    // 鼠标没锁定，不处理视角控制
    if !mouse_locked {
        // 但仍然需要消耗鼠标事件，防止事件堆积
        for _ in mouse_events.read() {}
        return;
    }



    let count = query.iter().len();
    if count == 0 {
        return;
    }
    
    for (mut look, mut transform) in query.iter_mut() {
        for event in mouse_events.read() {
            // 鼠标水平移动 → 左右转头（Yaw）
            look.yaw -= event.delta.x * 0.002;
            // 鼠标垂直移动 → 上下抬头（Pitch），限制角度防止翻转
            look.pitch -= event.delta.y * 0.002;
            look.pitch = look.pitch.clamp(-1.5, 1.5); // 约 ±85°

            // 应用旋转：先 Yaw（绕世界 Y 轴），再 Pitch（绕局部 X 轴）
            transform.rotation = Quat::from_axis_angle(Vec3::Y, look.yaw)
                * Quat::from_axis_angle(Vec3::X, look.pitch);
        }
    }
}

/// 键盘移动 - 基于相机朝向
pub fn player_move(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    // mut player_query: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let camera_transform = match camera_query.iter().next() {
        Some(cam) => cam,
        None => return,
    };
    
    let forward = -camera_transform.forward().as_vec3();
    let right = camera_transform.right().as_vec3();
    
    let forward_horizontal = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right_horizontal = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();
    
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction -= forward_horizontal;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += forward_horizontal;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= right_horizontal;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += right_horizontal;
    }

    let target_velocity = if direction != Vec3::ZERO {
        direction.normalize() * 5.0
    } else {
        Vec3::ZERO
    };
    
    for mut velocity in player_query.iter_mut() {
        velocity.linear = target_velocity;
    }
}

/// 鼠标锁定控制
pub fn toggle_mouse_lock(
    mut cursors: Query<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    for mut cursor in cursors.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            println!("鼠标锁定");
            cursor.visible = false;
            cursor.grab_mode = bevy::window::CursorGrabMode::Confined;
        }
        if key.just_pressed(KeyCode::Escape) {
            println!("鼠标释放");
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
