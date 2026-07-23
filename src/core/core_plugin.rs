use bevy::{ prelude::*};
use crate::game_state::GameState;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_world)
           .add_systems(Update, despawn_on_game_over);
    }
}

fn setup_world(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // 地面
    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(20.0, 0.5, 20.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -0.25, 0.0),
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