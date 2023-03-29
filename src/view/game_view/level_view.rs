use std::cmp::min;
use bevy::prelude::*;
use crate::{
    model::game_model::game::{Game, LevelCell},
    MAX_LEVEL_HEIGHT, MAX_LEVEL_WIDTH, SHIFT_DOWN, SHIFT_TO_RIGHT,
};
use super::image_handler::ImageMap;
use self::LevelControlButtonType::*;
use std::slice::Iter;

const LEVEL_DISPLAY_BUTTON_SIZE: f32 = 50.0;
const LEVEL_DISPLAY_BUTTON_MARGIN: f32 = 5.0;

pub struct LevelViewPlugin;

#[derive(Component)]
pub struct CellCollider;

#[derive(Component)]
pub struct Pawn;

#[derive(Component)]
pub struct CellMovable;

#[derive(Component, Clone, Copy, Debug)]
pub enum LevelControlButtonType {
    Play,
    StepBack,
    StepForward,
    Pause,
    Stop,
}

impl LevelControlButtonType {
    pub fn iterator() -> Iter<'static, LevelControlButtonType> {
        static BUTTONTYPES: [LevelControlButtonType; 5] = [Play, StepBack, StepForward, Pause, Stop];
        BUTTONTYPES.iter()
    }
}

impl Plugin for LevelViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_panel);
    }
}

fn create_panel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    image_map: Res<ImageMap>,
) {
    let image_size = min(
        MAX_LEVEL_WIDTH as u32 / game.columns,
        MAX_LEVEL_HEIGHT as u32 / game.rows,
    ) as f32;
    let background = commands.spawn((ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            margin: UiRect::all(Val::Auto),
            size: Size {
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
            },
            ..default()
        },
        image: UiImage(asset_server.load("level_background.png")),
        ..default()
    },)).id();
    info!("col: {}, rows: {}", game.columns, game.rows);

    info!("Image size: {}", image_size);
    let mut cells: Vec<Entity> = Vec::new();
    commands.entity(background).insert(Name::new("Level"));
    for i in 0..game.rows {
        for j in 0..game.columns {
            let cell_data = *game.as_ref().level_matrix.get(i.try_into().unwrap(), j.try_into().unwrap()).unwrap();
            if cell_data.letter != '_' {
                let cell = create_cell(
                    cell_data,
                    &image_map,
                    &mut commands,
                    image_size,
                    i,
                    j,
                );
                cells.push(cell);
            }
        }
        commands.entity(background).push_children(&cells);
        cells.clear();

    }
    let button_panel = create_button_panel(&mut commands, &image_map);
    commands.entity(background).insert(Name::new("Level")).add_child(button_panel);
}

fn create_cell(
    cell_data: LevelCell,
    image_map: &ImageMap,
    commands: &mut Commands,
    image_size: f32,
    i: u32,
    j: u32,
) -> Entity {
    info!("Scale x: {}, scale y: {}", 208.0/(cell_data.image_size_x*10.0), 208.0/(cell_data.image_size_y*10.0));
    let img = get_image(cell_data.letter, image_map);
    let cell = commands
        .spawn(ImageBundle {
            style: Style {
                size: Size { width: Val::Px(cell_data.image_size_x), height: Val::Px(cell_data.image_size_y) },
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(image_size * (j) as f32 + cell_data.extra_move_x + SHIFT_TO_RIGHT),
                    top: Val::Px(image_size * (i) as f32 + cell_data.extra_move_y + SHIFT_DOWN),
                    ..default()
                },
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_z(cell_data.angle)),
            // .with_scale(Vec3 { x: 208.0/(cell_data.image_size_x*100.0), y: 208.0/(cell_data.image_size_y*100.0), z: 1.0 }),
            image: img,
            ..Default::default()
        })
        .id();
    if "pP".to_string().contains(cell_data.letter) {
        commands.entity(cell).insert(Pawn);
    } else if "XV".to_string().contains(cell_data.letter) {
        commands.entity(cell).insert(CellMovable);
    } else {
        commands.entity(cell).insert(CellCollider);
    }
    return cell;
}

fn get_image(letter: char, image_map: &ImageMap) -> UiImage {
    for key in (*image_map).0.keys() {
        if key.contains(letter) {
            return (*image_map.0.get(key).unwrap()).clone();
        }
    }
    return UiImage::default();
}

fn create_button_panel(commands: &mut Commands, image_map: &ImageMap) -> Entity {
    let mut buttons: Vec<Entity> = Vec::new();
    let mut button_type_iterator = LevelControlButtonType::iterator();
    for img in image_map.1.clone() {
        let button: Entity = create_button(commands, img, *button_type_iterator.next().unwrap());
        buttons.push(button);
    }
    let panel = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            size: Size {
                width: Val::Px(
                    5.0 * (LEVEL_DISPLAY_BUTTON_MARGIN * 2.0
                        + LEVEL_DISPLAY_BUTTON_SIZE),
                ),
                height: Val::Px(
                    LEVEL_DISPLAY_BUTTON_MARGIN * 2.0 + LEVEL_DISPLAY_BUTTON_SIZE,
                ),
            },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        background_color: Color::GRAY.into(),
        ..default()
    }).id();
    commands.entity(panel).push_children(&buttons);
    return panel;
}

fn create_button(commands: &mut Commands, img: UiImage, button_type: LevelControlButtonType) -> Entity {
    return commands.spawn(ButtonBundle {
        style: Style {
            size: Size { width: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE), height: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE) },
            margin: UiRect::all(Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)),
            ..Default::default()
        },
        image: img,
        ..Default::default()
    }).insert(button_type).id();
}