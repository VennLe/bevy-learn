<!-- 支持格式：TTF、OTF -->

// src/ui/plugin.rs
use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 加载字体
    let font_handle: Handle<Font> = asset_server.load("fonts/NotoSansSC-Regular.ttf");
    
    // 创建文本实体
    commands.spawn((
        Text("Hello World"),
        TextFont {
            font: font_handle,
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}