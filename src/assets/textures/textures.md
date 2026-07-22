# character_albedo.png   # 漫反射贴图
# character_normal.png   # 法线贴图
# character_roughness.png # 粗糙度贴图
# character_ao.png       # AO 贴图

// 支持格式：PNG、JPG、KTX2、DDS、BMP 等

// src/player/systems.rs
use bevy::prelude::*;
use bevy::render::texture::Image;

pub fn load_player_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 加载纹理图片
    let albedo_texture: Handle<Image> = asset_server.load("textures/character_albedo.png");
    let normal_texture: Handle<Image> = asset_server.load("textures/character_normal.png");
    let roughness_texture: Handle<Image> = asset_server.load("textures/character_roughness.png");
    
    // 创建材质并应用纹理
    let material = StandardMaterial {
        base_color_texture: Some(albedo_texture),      // 漫反射贴图
        normal_map_texture: Some(normal_texture),      // 法线贴图
        perceptual_roughness: 0.5,                     // 基础粗糙度
        metallic: 0.0,                                 // 金属度
        ..default()
    };
    
    let material_handle = materials.add(material);
    
    // 应用到实体
    commands.spawn((
        Mesh3d(asset_server.load("models/character.glb#Mesh0/Primitive0")),
        MeshMaterial3d(material_handle),
        Transform::default(),
    ));
}