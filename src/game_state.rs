use std::process;

use bevy::{asset::LoadState, gltf::gltf, prelude::*};

// 游戏全局状态
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
    Paused,
    GameOver,
}

// 集中管理所有游戏资产的Handle
#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub main_menu_bg: Handle<Image>,
    pub c6091_scene: Handle<Gltf>,
}

// 游戏加载进度跟踪
#[derive(Resource)]
pub struct LoadingProgress  {
    handles: Vec<UntypedHandle>,
    total: usize,
    loaded: usize,
}

// 统一的资产加载与检查系统
pub fn load_and_check_assets(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    progress: Option<ResMut<LoadingProgress>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut initialized: Local<bool>,
) {
    // 第一帧：注册所有资产并初始化
    if !*initialized {
        let c6091: Handle<Gltf> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/c6091.glb"));
        let font: Handle<Font> = asset_server.load("font/NotoSerifCJKsc/SimplifiedChinese/NotoSerifCJKsc-Regular.otf");
        let main_menu_bg: Handle<Image> = asset_server.load("ui/m1.png");

        // 保存所有 Handle 供后续使用
        cmd.insert_resource(GameAssets {
            font: font.clone(),
            main_menu_bg: main_menu_bg.clone(),
            c6091_scene: c6091,
        });

         // 把 Handle 列表也存到进度里
        let handles = vec![font.untyped(), main_menu_bg.untyped()];
        let total = handles.len();
        cmd.insert_resource(LoadingProgress {
            handles,
            total,
            loaded: 0,
        });

        *initialized = true;
        return;
    }

    let mut progress = match progress {
        Some(p) => p,
        None => return,
    };

    let total = progress.handles.len();
    if total == 0 {
          // 没有需要加载的资产，直接进入菜单
        next_state.set(GameState::Menu);
        return;
    }

    // 统计已加载数量
    let loaded_count = progress
        .handles
        .iter()
        .filter(|handle| {
            matches!(asset_server.get_load_state(*handle), Some(LoadState::Loaded))
        })
        .count();

    progress.total = total;
    progress.loaded = loaded_count;

    // 检查是否有加载失败的资产，有则直接终止程序
    for handle in &progress.handles {
        if let Some(LoadState::Failed(err)) = asset_server.get_load_state(handle.id()) {
            eprintln!("❌ 资产加载失败！");
            eprintln!("   原因: {}", err);
            process::exit(1);
        }
    }

    // 所有资产加载完成 → 切到 Menu
    if loaded_count == total {
        info!("✅ 所有资产加载完成，进入菜单");
        next_state.set(GameState::Menu);
    }
}

pub struct InitGamePlugin;

impl Plugin for InitGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_and_check_assets);
    }
}