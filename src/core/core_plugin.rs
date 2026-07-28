use std::any::Any;

use crate::game_state::{GameAssets, GameState};
use bevy::{
    app::SceneSpawnerSystems::SceneSpawn,
    gltf::{GltfAssetLabel::Scene, GltfMesh, GltfSceneName},
    prelude::*,
    scene::ScenePatch,
};
use bevy_rapier3d::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_world)
            .add_systems(
                Update,
                despawn_on_game_over.run_if(in_state(GameState::GameOver)),
            );
    }
}

// 创建简易3d场景地图
fn setup_world(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    let wall = Mesh::from(Cuboid::from_size(Vec3::ONE));
    // 地面
    if let Some(terrain_material) = assets.textures.get("forest_leaves").cloned() {
        cmd.spawn((
            Mesh3d(meshes.add(wall.clone())),
            MeshMaterial3d(materials.add(terrain_material.clone())),
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(100.0, 0.2, 100.0)),
            // 碰撞检测
            RigidBody::Fixed,
            Collider::cuboid(50.0, 0.1, 50.0),
        ));
    }

    // ★ 从 Gltf 中提取场景
    if let Some(c6091) = assets.scenes.get("c6091_model").cloned() {
        cmd.spawn((
            WorldAssetRoot(c6091.into()),
            Transform::from_xyz(5.0, 2.0, -10.0).with_scale(Vec3::splat(10.0)),
            
            RigidBody::Dynamic,
        ));
    }
     if let Some(torch) = assets.scenes.get("torch").cloned() {
        cmd.spawn((
            WorldAssetRoot(torch.into()),
            Transform::from_xyz(-5.0, 6.0, -5.0).with_scale(Vec3::splat(10.0)),
            
            RigidBody::Dynamic,
        ));
    }
    if let Some(lemon) = assets.scenes.get("lemon").cloned() {
        cmd.spawn((
            WorldAssetRoot(lemon.into()),
            Transform::from_xyz(-3.0, 1.0, 2.0).with_scale(Vec3::splat(10.0)),
            
            RigidBody::Dynamic,
        ));
    }

    // 几面墙壁 (用长方体拼接成一个简易的迷宫/竞技场)
    if let Some(wall_material) = assets.textures.get("mossy_bricks") {
        // 北墙
        cmd.spawn((
            Mesh3d(meshes.add(wall.clone())),
            MeshMaterial3d(materials.add(wall_material.clone())),
            Transform::from_xyz(0.0, 2.0, 10.0).with_scale(Vec3::new(20.0, 4.0, 0.2)),
            // 碰撞检测
            RigidBody::Fixed,
            Collider::cuboid(10.0, 2.0, 0.1),
        ));

        // 西墙
        cmd.spawn((
            Mesh3d(meshes.add(wall.clone())),
            MeshMaterial3d(materials.add(wall_material.clone())),
            Transform::from_xyz(-10.1, 2.0, 0.0).with_scale(Vec3::new(0.2, 4.0, 20.0)),
            // 碰撞检测
            RigidBody::Fixed,
            Collider::cuboid(0.1, 2.0, 10.0),
        ));

        // 东墙
        cmd.spawn((
            Mesh3d(meshes.add(wall.clone())),
            MeshMaterial3d(materials.add(wall_material.clone())),
            Transform::from_xyz(10.1, 2.0, 0.0).with_scale(Vec3::new(0.2, 4.0, 20.0)),
            // 碰撞检测
            RigidBody::Fixed,
            Collider::cuboid(0.1, 2.0, 10.0),
        ));
    } 

    // 几个障碍物（箱子）
    let box_material = materials.add(Color::srgb(0.8, 0.5, 0.2)); // 木箱色

    let box_positions = [
        Vec3::new(3.0, 0.5, 2.0),
        Vec3::new(-4.0, 0.5, -3.0),
        Vec3::new(0.0, 1.0, 5.0),
        Vec3::new(-2.0, 0.5, 4.0),
        Vec3::new(5.0, 0.5, -5.0),
    ];

    for pos in box_positions {
        cmd.spawn((
            Mesh3d(meshes.add(wall.clone())),
            MeshMaterial3d(box_material.clone()),
            Transform::from_translation(pos),
            // 碰撞检测
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5, 0.5),
        ));
    }

    // ===== 环境光 + 平行光（模拟太阳）=====
    cmd.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            //  shadows_enabled: true,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 微弱环境光（避免背光面全黑）
    cmd.spawn((
        PointLight {
            intensity: 200.0,
            color: Color::srgb(0.7, 0.8, 1.0), // 偏冷色
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));
}

fn despawn_on_game_over(
    mut cmd: Commands,
    query: Query<Entity, With<DespawnOnGameOver>>,
    state: Res<State<GameState>>,
) {
    if *state.get() == GameState::GameOver {
        for entity in query.iter() {
            cmd.entity(entity).despawn();
        }
    }
}

/// 标记组件：游戏结束时自动销毁
#[derive(Component, Default)]
pub struct DespawnOnGameOver;
