use bevy::{asset::LoadState, prelude::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

#[derive(Resource)]
pub struct LoadingProgress  {
    total: usize,
    loaded: usize,
}

pub fn setup_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let player_mesh: Handle<Mesh> = asset_server.load("models/character.glb#Mesh0/Primitive0");
    let bgm: Handle<AudioSource> = asset_server.load("audio/bgm.ogg");
    let font: Handle<Font> = asset_server.load("fonts/NotoSans.ttf");

    loading.0.push(player_mesh.untyped());
    loading.0.push(bgm.untyped());
    loading.0.push(font.untyped());

    commands.insert_resource(LoadingProgress {
        total: loading.0.len(),
        loaded: 0,
    });
}

pub fn check_assets_ready(
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut progress: ResMut<LoadingProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let total = loading.0.len();
    if total == 0 {
        return; // 没有资产需要加载
    }
    
    // 逐个检查每个资产的加载状态
    let loaded_count = loading.0.iter()
        .filter(|handle| {
            matches!(server.get_load_state(*handle), Some(LoadState::Loaded))
        })
        .count();
    
    progress.total = total;
    progress.loaded = loaded_count;
    
    // 检查是否有加载失败的资产
    let has_failed = loading.0.iter().any(|handle| {
        matches!(server.get_load_state(handle.id()), Some(LoadState::Failed(_)))
    });
    
    if has_failed {
        eprintln!("Some assets failed to load!");
        // 可以选择重试或继续
        return;
    }
    
    // 所有资产加载完成
    if loaded_count == total {
        next_state.set(GameState::Menu);
    }
}