use bevy::prelude::*;

const MARGIN: Val = Val::Px(5.);

// 标签类型
#[derive(PartialEq, Clone, Copy)]
enum TabType {
    Character,
    Weapon,
    Item,
    Story,
}   

#[derive(Resource)]
struct CharacterPanelState {
    is_open: bool,
    active_tab: TabType,
    nodes: Vec<Node>,
}

impl Default for CharacterPanelState {
    fn default() -> Self {
        Self {
            is_open: false,
            active_tab: TabType::Character,
        }
    }
}

// 面板组件标记
#[derive(Component)]
struct PanelRoot;

#[derive(Component)]
struct CloseButton;

#[derive(Component)]
struct TabButton(TabType);

#[derive(Component)]
struct ContentArea; 

fn character_panel_spawn(mut cmd: Commands) {
    cmd.spawn(Camera2d);

    cmd.spawn((
        Node{
            width: px(1270.),
            height: px(720.),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Transform::from_xyz(-1270. / 2.0, -720. / 2.0, 0.0),
        Visibility::Hidden,
        PanelRoot,
        Children::spawn(SpawnIter(
            nodes.into_iter().map(move |v|handle_node())
        ))
    ));
}

fn handle_node(){

}
