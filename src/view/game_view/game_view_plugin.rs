use super::{despawn_screen, GameState, level_view::LevelControlButtonType};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use super::level_view::LevelViewPlugin;


pub struct GameViewPlugin;

const BLOCK_TYPE_BUTTON_HEIGHT: f32 = 25.0;

#[derive(Component)]
struct GameViewScreen;

impl Plugin for GameViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<GameViewScreen>),
            )
            .add_plugin(LevelViewPlugin)
            .add_system(mouse_scroll)
            .add_system(level_control_button_system);
    }
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Menu display
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Percent(100.0),
                },
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Blocks",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Px((BLOCK_TYPE_BUTTON_HEIGHT + 10.0) * 4.0),
                        },
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|block_node| {
                    let block_types: [&str; 4] = ["Movement", "Flow Control", "Numbers", "Logic"];
                    for str in block_types {
                        block_node
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Percent(90.0),
                                        height: Val::Px(BLOCK_TYPE_BUTTON_HEIGHT),
                                    },
                                    margin: UiRect {
                                        left: Val::Px(5.0),
                                        right: Val::Px(5.0),
                                        top: Val::Px(5.0),
                                        bottom: Val::Px(5.0),
                                    },
                                    ..default()
                                },
                                background_color: Color::AQUAMARINE.into(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(
                                    (TextBundle::from_section(
                                        str,
                                        TextStyle {
                                            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ))
                                    .with_text_alignment(TextAlignment::CENTER),
                                );
                            });
                    }
                });
            parent.spawn((TextBundle::from_section(
                "Variables",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(30.0),
                        },
                        overflow: Overflow::Hidden,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0,
                                    max_size: Size::UNDEFINED,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ScrollingList::default(),
                        ))
                        .with_children(|parent| {
                            for i in 0..30 {
                                parent.spawn(
                                    TextBundle::from_section(
                                        format!("Variable {i}"),
                                        TextStyle {
                                            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        flex_shrink: 0.,
                                        size: Size::new(Val::Undefined, Val::Px(20.)),
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            ..default()
                                        },
                                        ..default()
                                    }),
                                );
                            }
                        });
                });
            parent.spawn((TextBundle::from_section(
                "Menu",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),));
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Save",
                        TextStyle {
                            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER),));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Options",
                        TextStyle {
                            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER),));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Back to Menu",
                        TextStyle {
                            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER),));
                });
        });
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

pub fn level_control_button_system(
    mut interaction_query: Query<
        (&Interaction, &LevelControlButtonType),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match *button_type {
                LevelControlButtonType::Play => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::StepBack => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::StepForward => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::Pause => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::Stop => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
            },
            Interaction::Hovered => match *button_type {
                LevelControlButtonType::Play => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::StepBack => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::StepForward => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::Pause => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::Stop => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
            },
            Interaction::None => match *button_type {
                LevelControlButtonType::Play => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::StepBack => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::StepForward => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::Pause => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
                LevelControlButtonType::Stop => {info!("Button: {:?}, Action: {:?}", button_type, interaction);}
            },
        }
    }
}