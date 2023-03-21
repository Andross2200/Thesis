use std::f32::consts::PI;

use super::{despawn_screen, GameState};
use bevy::{prelude::*,input::mouse::{MouseScrollUnit, MouseWheel}, winit::WinitSettings};
use crate::model::game_model::game::{init_from_fen, Game};
use std::cmp::min;

pub struct GameViewPlugin;

const LEVEL_DISPLAY_BUTTON_SIZE: f32 = 50.0;
const LEVEL_DISPLAY_BUTTON_MARGIN: f32 = 5.0;
const MAX_LEVEL_WIDTH: f32 = 500.0;
const MAX_LEVEL_HEIGHT: f32 = 550.0;
const SHIFT_TO_RIGHT: f32 = 7.0;
const SHIFT_DOWN: f32 = 20.0;
const BLOCK_TYPE_BUTTON_HEIGHT: f32 = 25.0;

#[derive(Component)]
struct GameViewScreen;

impl Plugin for GameViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<GameViewScreen>),
            )
            .add_system(mouse_scroll);
    }
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut game = init_from_fen("5 10 ZQWERTYUIA/SDFGHJKzxc/vqwertyuia/sdfghjkbnm/lBNpPXoOCV".to_string());
    // Level Display
    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                size: Size {width: Val::Percent(40.0), height: Val::Percent(100.0)},
                ..default()
            },
            image: UiImage(asset_server.load("level_background.png")),
            ..default()
    },
    GameViewScreen,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect { right: Val::Px(0.0), bottom: Val::Px(0.0), ..default()},
                size: Size {
                    width: Val::Px(5.0*(LEVEL_DISPLAY_BUTTON_MARGIN*2.0+LEVEL_DISPLAY_BUTTON_SIZE)),
                    height: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN*2.0+LEVEL_DISPLAY_BUTTON_SIZE)
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            // Play button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE)
                    ),
                    margin: UiRect {
                        right: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        top: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        left: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        bottom: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)
                    },
                    ..default()
                },
                image: UiImage(asset_server.load("buttons/start.png")),
                ..default()
            });
            // Step back button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE)
                    ),
                    margin: UiRect {
                        right: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        top: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        left: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        bottom: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)
                    },
                    ..default()
                },
                image: UiImage(asset_server.load("buttons/step_back.png")),
                ..default()
            });
            // Step forward button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE)
                    ),
                    margin: UiRect {
                        right: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        top: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        left: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        bottom: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)
                    },
                    ..default()
                },
                image: UiImage(asset_server.load("buttons/step_forward.png")),
                ..default()
            });
            // Step pause button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE)
                    ),
                    margin: UiRect {
                        right: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        top: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        left: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        bottom: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)
                    },
                    ..default()
                },
                image: UiImage(asset_server.load("buttons/pause.png")),
                ..default()
            });
            // Stop button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                        Val::Px(LEVEL_DISPLAY_BUTTON_SIZE)
                    ),
                    margin: UiRect {
                        right: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        top: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        left: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN),
                        bottom: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)
                    },
                    ..default()
                },
                image: UiImage(asset_server.load("buttons/stop.png")),
                ..default()
            });
        });
        draw_level(parent, &asset_server, game);
    });
    // Menu display
    commands.spawn(NodeBundle {
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
            size: Size {width: Val::Percent(10.0), height: Val::Percent(100.0)},
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn((TextBundle::from_section(
            "Blocks",
            TextStyle {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 50.0,
                color: Color::BLACK },)
                .with_text_alignment(TextAlignment::CENTER),
        ));
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                size: Size {width: Val::Percent(100.0), height:Val::Px((BLOCK_TYPE_BUTTON_HEIGHT+10.0)*4.0)},
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|block_node| {
            let block_types: [&str; 4] = ["Movement", "Flow Control", "Numbers", "Logic"];
            for str in block_types {
                block_node.spawn(ButtonBundle {
                    style: Style {
                        size: Size {width: Val::Percent(90.0), height:Val::Px(BLOCK_TYPE_BUTTON_HEIGHT)},
                        margin: UiRect { left: Val::Px(5.0), right: Val::Px(5.0), top: Val::Px(5.0), bottom: Val::Px(5.0) },
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn((TextBundle::from_section(str, TextStyle {
                        font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK })).with_text_alignment(TextAlignment::CENTER));
                });
            }
        });
        parent.spawn((TextBundle::from_section(
            "Variables",
            TextStyle {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 40.0,
                color: Color::BLACK },)
                .with_text_alignment(TextAlignment::CENTER),
        ));
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                size: Size {width: Val::Percent(100.0), height:Val::Percent(30.0)},
                overflow: Overflow::Hidden,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((NodeBundle {
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
                    parent.spawn(TextBundle::from_section(
                        format!("Variable {i}"),
                        TextStyle {
                            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE }
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
                    }));
                }
            });
        });
        parent.spawn((TextBundle::from_section(
            "Menu",
            TextStyle {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 40.0,
                color: Color::BLACK },)
                .with_text_alignment(TextAlignment::CENTER),
        ));
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                ..default()
             },
             ..default()
        }).with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            });
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            });
        });
        parent.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: Color::AQUAMARINE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Save",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK },)
                    .with_text_alignment(TextAlignment::CENTER),
            ));
        });
        parent.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: Color::AQUAMARINE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Options",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK },)
                    .with_text_alignment(TextAlignment::CENTER),
            ));
        });
        parent.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: Color::AQUAMARINE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Back to Menu",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::BLACK },)
                    .with_text_alignment(TextAlignment::CENTER),
            ));
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

fn draw_level(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, game: Game) {
    let cell_image_size = min(MAX_LEVEL_WIDTH as u32/game.columns, MAX_LEVEL_HEIGHT as u32/game.rows) as  f32;
    for i in 0..game.rows {
        for j in 0..game.columns {
            let cell = game.level_matrix.get(i.try_into().unwrap(), j.try_into().unwrap()).unwrap();
            let mut img: UiImage = UiImage::default();
            let mut angle: f32 = 0.0;
            let mut size_image_coeff_x = 1.0;
            let mut size_image_coeff_y = 1.0;
            let mut extra_move_x: f32 = 0.0;
            let mut extra_move_y: f32 = 0.0;
            match cell {
                'Z'=>{
                    img = UiImage(asset_server.load("blocks/big_block.png"));
                }
                'Q'=>{
                    img = UiImage(asset_server.load("blocks/big_left_triangle.png"));
                }
                'W'=>{
                    img = UiImage(asset_server.load("blocks/big_left_triangle.png"));
                    angle = PI/2.0;
                }
                'E'=>{
                    img = UiImage(asset_server.load("blocks/big_left_triangle.png"));
                    angle = PI;
                }
                'R'=>{
                    img = UiImage(asset_server.load("blocks/big_left_triangle.png"));
                    angle = PI*3.0/2.0;
                }
                'T'=>{
                    img = UiImage(asset_server.load("blocks/big_right_triangle.png"));
                }
                'Y'=>{
                    img = UiImage(asset_server.load("blocks/big_right_triangle.png"));
                    angle = PI/2.0;
                }
                'U'=>{
                    img = UiImage(asset_server.load("blocks/big_right_triangle.png"));
                    angle = PI;
                }
                'I'=>{
                    img = UiImage(asset_server.load("blocks/big_right_triangle.png"));
                    angle = PI*3.0/2.0;
                }
                'A'=>{
                    img = UiImage(asset_server.load("blocks/big_left_half_slope.png"));
                }
                'S'=>{
                    img = UiImage(asset_server.load("blocks/big_left_half_slope.png"));
                    angle = PI/2.0;
                }
                'D'=>{
                    img = UiImage(asset_server.load("blocks/big_left_half_slope.png"));
                    angle = PI;
                }
                'F'=>{
                    img = UiImage(asset_server.load("blocks/big_left_half_slope.png"));
                    angle = PI*3.0/2.0;
                }
                'G'=>{
                    img = UiImage(asset_server.load("blocks/big_right_half_slope.png"));
                }
                'H'=>{
                    img = UiImage(asset_server.load("blocks/big_right_half_slope.png"));
                    angle = PI/2.0;
                }
                'J'=>{
                    img = UiImage(asset_server.load("blocks/big_right_half_slope.png"));
                    angle = PI;
                }
                'K'=>{
                    img = UiImage(asset_server.load("blocks/big_right_half_slope.png"));
                    angle = PI*3.0/2.0;
                }
                'z'=>{
                    img = UiImage(asset_server.load("blocks/small_block.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                }
                'x'=>{
                    img = UiImage(asset_server.load("blocks/small_block.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/2.0;
                }
                'c'=>{
                    img = UiImage(asset_server.load("blocks/small_block.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/2.0;
                    extra_move_y = cell_image_size/2.0;
                }
                'v'=>{
                    img = UiImage(asset_server.load("blocks/small_block.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                }
                'q'=>{
                    img = UiImage(asset_server.load("blocks/small_left_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                }
                'w'=>{
                    img = UiImage(asset_server.load("blocks/small_left_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    angle = PI/2.0;
                }
                'e'=>{
                    img = UiImage(asset_server.load("blocks/small_left_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/2.0;
                    angle = PI;
                }
                'r'=>{
                    img = UiImage(asset_server.load("blocks/small_left_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/2.0;
                    extra_move_y = cell_image_size/2.0;
                    angle = PI*3.0/2.0;
                }
                't'=>{
                    img = UiImage(asset_server.load("blocks/small_right_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/2.0;
                    extra_move_y = cell_image_size/2.0;
                }
                'y'=>{
                    img = UiImage(asset_server.load("blocks/small_right_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                    angle = PI/2.0;
                }
                'u'=>{
                    img = UiImage(asset_server.load("blocks/small_right_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    angle = PI;
                }
                'i'=>{
                    img = UiImage(asset_server.load("blocks/small_right_triangle.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/2.0;
                    angle = PI*3.0/2.0;
                }
                'a'=>{
                    img = UiImage(asset_server.load("blocks/small_left_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                }
                's'=>{
                    img = UiImage(asset_server.load("blocks/small_left_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_x = -cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                    angle = PI/2.0;
                }
                'd'=>{
                    img = UiImage(asset_server.load("blocks/small_left_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    angle = PI;
                }
                'f'=>{
                    img = UiImage(asset_server.load("blocks/small_left_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                    angle = PI*3.0/2.0;
                }
                'g'=>{
                    img = UiImage(asset_server.load("blocks/small_right_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                }
                'h'=>{
                    img = UiImage(asset_server.load("blocks/small_right_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_x = -cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                    angle = PI/2.0;
                }
                'j'=>{
                    img = UiImage(asset_server.load("blocks/small_right_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    angle = PI;
                }
                'k'=>{
                    img = UiImage(asset_server.load("blocks/small_right_half_slope.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                    angle = PI*3.0/2.0;
                }
                'b'=>{
                    img = UiImage(asset_server.load("blocks/lower_half_block.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                }
                'n'=>{
                    img = UiImage(asset_server.load("blocks/lower_half_block.png"));
                    size_image_coeff_y = 0.5;
                    angle = PI/2.0;
                    extra_move_x = -cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                }
                'm'=>{
                    img = UiImage(asset_server.load("blocks/lower_half_block.png"));
                    angle = PI;
                    size_image_coeff_y = 0.5;
                }
                'l'=>{
                    img = UiImage(asset_server.load("blocks/lower_half_block.png"));
                    angle = PI*3.0/2.0;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                }
                'B'=>{
                    img = UiImage(asset_server.load("blocks/center_half_block.png"));
                    size_image_coeff_x = 0.5;
                    extra_move_x = cell_image_size/4.0;
                }
                'N'=>{
                    img = UiImage(asset_server.load("blocks/center_half_block.png"));
                    angle = PI/2.0;
                    size_image_coeff_x = 0.5;
                    extra_move_x = cell_image_size/4.0;
                }
                'p'=>{
                    img = UiImage(asset_server.load("pawns/green.png"));
                }
                'P'=>{
                    img = UiImage(asset_server.load("pawns/orange.png"));
                }
                'X'=>{
                    img = UiImage(asset_server.load("extra/stone.png"));
                }
                'o'=>{
                    img = UiImage(asset_server.load("extra/closed_shell.png"));
                    size_image_coeff_y = 0.5;
                    extra_move_y = cell_image_size/2.0;
                }
                'O'=>{
                    img = UiImage(asset_server.load("extra/open_shell.png"));
                }
                'C'=>{
                    img = UiImage(asset_server.load("extra/perl.png"));
                    size_image_coeff_x = 0.5;
                    size_image_coeff_y = 0.5;
                    extra_move_x = cell_image_size/4.0;
                    extra_move_y = cell_image_size/4.0;
                }
                'V'=>{
                    img = UiImage(asset_server.load("extra/hexagon.png"));
                }
                _=>{
                    print!("empty cell");
                    continue;
                }
            }
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(cell_image_size*(j) as f32 + extra_move_x + SHIFT_TO_RIGHT), top: Val::Px(cell_image_size*(i) as f32 + extra_move_y + SHIFT_DOWN), ..default() },
                    size: Size {width: Val::Px(cell_image_size*size_image_coeff_x), height: Val::Px(cell_image_size*size_image_coeff_y)},
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_rotation_z(angle)),
                image: img,
                ..default()
            });
        }
    }
}