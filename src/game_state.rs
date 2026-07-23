use bevy::{asset::LoadState, prelude::*, ui::update};

use crate::ui::main_menu::{self, menu_button_system};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
    Paused,
    GameOver,
}

// 用于跟踪正在加载的资产列表
#[derive(Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

// 加载进度跟踪
#[derive(Resource)]
pub struct LoadingProgress  {
    total: usize,
    loaded: usize,
}

// 检查资产是否全部加载完成
pub fn setup_loading(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    // let player_mesh: Handle<Mesh> = asset_server.load("models/character.glb#Mesh0/Primitive0");
    // let bgm: Handle<AudioSource> = asset_server.load("audio/bgm.ogg");
    // 加载字体
    let font: Handle<Font> = asset_server.load("font/NotoSerifCJKsc/SimplifiedChinese/NotoSerifCJKsc-Regular.otf");
    // 加载菜单背景图片
    let main_menu: Handle<Image> = asset_server.load("ui/m1.png");

    // loading.0.push(player_mesh.untyped());
    // loading.0.push(bgm.untyped());
    loading.0.push(font.untyped());
    loading.0.push(main_menu.untyped());

    cmd.insert_resource(LoadingProgress {
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

pub struct InitGamePlugin;

impl Plugin for InitGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_loading, main_menu::setup_menu))
            .add_systems(Update, check_assets_ready.run_if(in_state(GameState::Loading)))
            .add_systems(Update, menu_button_system.run_if(in_state(GameState::Menu)));
    }
}