use std::{collections::HashMap, hash::Hash, process};

use bevy::{asset::LoadState, gltf::gltf_ext::scene, prelude::*};

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
    pub scenes: HashMap<&'static str, Handle<WorldAsset>>,
    // pub textures: HashMap<&'static str, StandardMaterial>,
}

// 游戏加载进度跟踪
#[derive(Resource)]
pub struct LoadingProgress {
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
        // 模型资产
        let c6091: Handle<WorldAsset> =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/c6091.glb"));
        // 手电筒
        let torch: Handle<WorldAsset> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/small_plastic_torch_2k.glb"));
        // 柠檬
        let lemon: Handle<WorldAsset> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/food_lime_01_2k.glb"));
        // object
        let object: Handle<WorldAsset> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/concrete_rock_path_2k.glb"));

        let font: Handle<Font> =
            asset_server.load("font/NotoSerifCJKsc/SimplifiedChinese/NotoSerifCJKsc-Regular.otf");
        let main_menu_bg: Handle<Image> = asset_server.load("ui/m1.png");
        // wall
        let diff_mossy_bricks: Handle<Image> =
            asset_server.load("textures/wall/mossy_brick_diff_2k.jpg");
        let nor_mossy_bricks: Handle<Image> =
            asset_server.load("textures/wall/mossy_brick_nor_gl_2k.exr");
        let rough_mossy_bricks: Handle<Image> =
            asset_server.load("textures/wall/mossy_brick_rough_2k.exr");
        // let wall_material = StandardMaterial {
        //     base_color_texture: Some(diff_mossy_bricks),
        //     normal_map_texture: Some(nor_mossy_bricks),
        //     metallic_roughness_texture: Some(rough_mossy_bricks),
        //     metallic: 0.0,
        //     ..default()
        // };

        // terrain
        // let diff_forest_leaves: Handle<Image> =
        //     asset_server.load("textures/terrain/forest_leaves_02_diffuse_2k.jpg");
        // let nor_forest_leaves: Handle<Image> =
        //     asset_server.load("textures/terrain/forest_leaves_02_nor_gl_2k.exr");
        // let rough_forest_leaves: Handle<Image> =
        //     asset_server.load("textures/terrain/forest_leaves_02_rough_2k.jpg");
        // let terrain_material = StandardMaterial {
        //     base_color_texture: Some(diff_forest_leaves),
        //     normal_map_texture: Some(nor_forest_leaves),
        //     metallic_roughness_texture: Some(rough_forest_leaves),
        //     metallic: 0.0,
        //     ..default()
        // };

        // 模型资产信息
        let mut scenes_map = HashMap::new();
        scenes_map.insert("c6091_model", c6091);
        scenes_map.insert("torch", torch);
        scenes_map.insert("lemon", lemon);
        scenes_map.insert("object", object);
        // // 纹理资产信息
        // let mut textures_map = HashMap::new();
        // textures_map.insert("mossy_bricks", wall_material);
        // textures_map.insert("forest_leaves", terrain_material);

        // 保存所有 Handle 供后续使用
        cmd.insert_resource(GameAssets {
            font: font.clone(),
            main_menu_bg: main_menu_bg.clone(),
            scenes: scenes_map,
            // textures: textures_map,
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
            matches!(
                asset_server.get_load_state(*handle),
                Some(LoadState::Loaded)
            )
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
