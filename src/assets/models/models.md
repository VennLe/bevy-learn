# character.glb          # 主模型文件（包含网格、骨骼、动画、材质引用）
# character.gltf + .bin  # 或拆分成 JSON + 二进制数据

// 支持格式：glTF、OBJ（需插件）
// src/player/systems.rs
use bevy::prelude::*;

pub fn load_player_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 方式一：从 glTF 文件加载网格（推荐）
    // 注意：glTF 文件中的网格通过路径索引访问
    let mesh_handle: Handle<Mesh> = asset_server.load("models/character.glb#Mesh0/Primitive0");
   
    // 方式二：加载整个 glTF 场景（包含网格、材质等）
    let scene_handle: Handle<Scene> = asset_server.load("models/character.glb#Scene0");
    
    // 使用网格创建实体
    commands.spawn((
        Mesh3d(mesh_handle),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

<!-- 注意：骨骼和蒙皮数据内嵌在 glTF 文件中，不需要单独加载 -->

// src/player/systems.rs
use bevy::prelude::*;

pub fn load_skinned_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 加载包含骨骼的 glTF 场景
    // Bevy 会自动解析骨骼层级和蒙皮权重
    commands.spawn((
        SceneRoot(asset_server.load("models/skinned_character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// 查询已加载的骨骼信息
fn inspect_skeleton(
    skeleton_query: Query<&Skeleton>,
) {
    for skeleton in skeleton_query.iter() {
        // 骨骼层级信息
        println!("Skeleton joints: {}", skeleton.joints.len());
        for joint in &skeleton.joints {
            println!("Joint: {:?}", joint.name);
        }
    }
}