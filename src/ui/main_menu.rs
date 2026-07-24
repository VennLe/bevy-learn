use bevy::prelude::*;

use crate::game_state::{GameAssets, GameState};

// 标记组件：标识属于菜单的实体
#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MenuButton {
    NewGame,
    Settings,
    Quit,
}

pub fn setup_menu(mut cmd: Commands, assets: Res<GameAssets>) {
    cmd.spawn(Camera2d);

    // 根节点：铺满窗口，带背景图
    cmd.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        MainMenu,
    ))
    .insert(ImageNode::new(assets.main_menu_bg.clone()))
    .with_children(|parent| {
        // 菜单容器 - 垂直排列三个按钮
        parent
            .spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },))
            .with_children(|parent| {
                // 新游戏按钮
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        MenuButton::NewGame,
                    ))
                    .with_child((
                        Text::new("新游戏"),
                        TextFont {
                            font: FontSource::Handle(assets.font.clone()),
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                // 设置按钮
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        MenuButton::Settings,
                    ))
                    .with_child((
                        Text::new("设置"),
                        TextFont {
                            font: FontSource::Handle(assets.font.clone()),
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                // 退出按钮
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // 红色突出退出
                        MenuButton::Quit,
                    ))
                    .with_child((
                        Text::new("退出"),
                        TextFont {
                            font: FontSource::Handle(assets.font.clone()),
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
            });
    });
}

pub fn menu_button_system(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_exit: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::NewGame => {
                    info!("点击了：新游戏");
                    next_state.set(GameState::Playing);
                }
                MenuButton::Settings => {
                    info!("点击了：设置");
                }
                MenuButton::Quit => {
                    info!("点击了：退出");
                    app_exit.write(AppExit::Success);
                }
            }
        }
    }
}

// 离开 Menu 时，清除所有带 MainMenu 标记的实体
pub fn close_main_menu(mut cmd: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_children();
        cmd.entity(entity).despawn();
    }
    info!("🧹 主菜单已清理");
}
