// src/ui/hud.rs
use bevy::prelude::*;
use crate::player::components::Player;

/// 更新 HUD 文本显示
pub fn update_hud(
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text>,
) {
    if let Ok(player) = player_query.single() {
        for mut text in text_query.iter_mut() {
            text.0 = format!("HP: {:.0}/{}", player.hp, player.max_hp);
        }
    }
}

/// 用来标记 HUD 文本的轻量组件
#[derive(Component)]
pub struct HudTextMarker;

/// 使用传统 API 创建 HUD（不需要 bsn! 宏）
pub fn spawn_hud(mut commands: Commands) {
    // 父节点：全屏容器
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            // 标记组件，方便 update_hud 精准选中文本
            HudTextMarker,
        ))
        .with_children(|parent| {
            // 子节点：HP 文本
            parent.spawn((
                Text::new("HP: 100/100"),
                TextFont {
                    font_size: FontSize::Px(36.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                HudTextMarker,
            ));
        });
}