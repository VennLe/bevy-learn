# character.material     # 材质定义文件（可选）

<!-- 支持格式：glTF 内嵌材质、代码创建 -->

// 方式一：使用 glTF 文件自带的材质
fn use_gltf_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // glTF 文件中的材质会自动加载
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::default(),
    ));
}

// 方式二：代码创建自定义材质
fn create_custom_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let custom_material = StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),       // 基础颜色
        base_color_texture: Some(asset_server.load("textures/armor.png")),
        emissive: Color::srgb(0.0, 0.0, 0.0),          // 自发光
        metallic: 0.8,                                  // 金属质感
        perceptual_roughness: 0.2,                      // 光滑表面
        reflectance: 0.5,                               // 反射率
        ..default()
    };
    
    let handle = materials.add(custom_material);
    
    commands.spawn((
        Mesh3d(asset_server.load("models/sword.glb#Mesh0/Primitive0")),
        MeshMaterial3d(handle),
    ));
}