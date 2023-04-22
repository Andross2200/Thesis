#![allow(clippy::type_complexity)]

use self::LevelControlButtonType::*;
use super::despawn_screen;
use crate::{
    model::game_model::game::{Game, LevelCell},
    utilities::{script_plugin::{reset_level, ScriptRes, ScriptRunStatus}, language_plugin::LanguageResource},
    view::{image_handler::ImageMap, GameState},
    MAX_LEVEL_HEIGHT, MAX_LEVEL_WIDTH, SHIFT_DOWN, SHIFT_TO_RIGHT,
};
use bevy::prelude::*;
use std::cmp::min;
use std::slice::Iter;

const LEVEL_DISPLAY_BUTTON_SIZE: f32 = 50.0;
const LEVEL_DISPLAY_BUTTON_MARGIN: f32 = 5.0;

#[derive(Component)]
pub struct LevelView;

pub struct LevelViewPlugin;

#[derive(Component)]
pub struct CellCollider;

#[derive(Component)]
pub struct GreenPawn;

#[derive(Component)]
pub struct OrangePawn;

#[derive(Component, PartialEq, Eq)]
pub enum ShellType {
    Closed,
    Open,
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Perl {
    Collected,
    NotCollected,
}

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

#[derive(Component)]
pub struct ScoreText;

impl LevelControlButtonType {
    pub fn iterator() -> Iter<'static, LevelControlButtonType> {
        static BUTTONTYPES: [LevelControlButtonType; 5] =
            [Play, StepBack, StepForward, Pause, Stop];
        BUTTONTYPES.iter()
    }
}

impl Plugin for LevelViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(create_panel))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<LevelView>),
            )
            .add_system(level_control_button_system);
    }
}

fn create_panel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    image_map: Res<ImageMap>,
    language: Res<LanguageResource>
) {
    let image_size = min(
        MAX_LEVEL_WIDTH as u32 / game.columns,
        MAX_LEVEL_HEIGHT as u32 / game.rows,
    ) as f32;
    let background = commands
        .spawn((ImageBundle {
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
        },))
        .id();
    commands.entity(background).insert(LevelView);
    let mut walls_shells: Vec<Entity> = Vec::new();
    let mut pawns_stones: Vec<Entity> = Vec::new();
    let mut perls: Vec<Entity> = Vec::new();
    commands.entity(background).insert(Name::new("Level"));
    for i in 0..game.rows {
        for j in 0..game.columns {
            let cell_data = *game
                .as_ref()
                .level_matrix
                .get(i.try_into().unwrap(), j.try_into().unwrap())
                .unwrap();
            if cell_data.letter != '_' {
                let cell = create_cell(cell_data, &image_map, &mut commands, image_size, i, j);
                if "ZQWERTYUIASDFGHJKzxcvqwertyuiasdfghjkbnmlBNoO"
                    .to_string()
                    .contains(cell_data.letter)
                {
                    walls_shells.push(cell);
                } else if "pPXV".to_string().contains(cell_data.letter) {
                    pawns_stones.push(cell);
                } else if cell_data.letter == 'C' {
                    perls.push(cell);
                }
            }
        }
    }
    if !walls_shells.is_empty() {
        commands.entity(background).push_children(&walls_shells);
    }
    if !pawns_stones.is_empty() {
        commands.entity(background).push_children(&pawns_stones);
    }
    if !perls.is_empty() {
        commands.entity(background).push_children(&perls);
    }
    let button_panel = create_button_panel(&mut commands, &image_map);
    let info_panel = create_info_panel(&mut commands, &image_map, language.game.perls_score_label.clone());
    commands
        .entity(background)
        .insert(Name::new("Level"))
        .add_child(button_panel)
        .add_child(info_panel);
}

fn create_cell(
    mut cell_data: LevelCell,
    image_map: &ImageMap,
    commands: &mut Commands,
    image_size: f32,
    i: u32,
    j: u32,
) -> Entity {
    let img = get_image(cell_data.letter, image_map);
    let cell = commands
        .spawn(ImageBundle {
            style: Style {
                size: Size {
                    width: Val::Px(cell_data.image_size_x),
                    height: Val::Px(cell_data.image_size_y),
                },
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(
                        image_size * (j) as f32 + cell_data.extra_move_x + SHIFT_TO_RIGHT,
                    ),
                    top: Val::Px(image_size * (i) as f32 + cell_data.extra_move_y + SHIFT_DOWN),
                    ..default()
                },
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_z(cell_data.angle)),
            image: img,
            ..Default::default()
        })
        .id();
    if "pP".to_string().contains(cell_data.letter) {
        if "p".to_string().contains(cell_data.letter) {
            commands.entity(cell).insert(GreenPawn);
        } else {
            commands.entity(cell).insert(OrangePawn);
        }
    } else if "XV".to_string().contains(cell_data.letter) {
        commands.entity(cell).insert(CellMovable);
    } else if "o".to_string().contains(cell_data.letter) {
        commands.entity(cell).insert(ShellType::Closed);
    } else if "O".to_string().contains(cell_data.letter) {
        commands.entity(cell).insert(ShellType::Open);
    } else if "C".to_string().contains(cell_data.letter) {
        commands.entity(cell).insert(Perl::NotCollected);
    } else {
        commands.entity(cell).insert(CellCollider);
    }
    cell_data.cell_entity = Some(cell);
    cell
}

fn get_image(letter: char, image_map: &ImageMap) -> UiImage {
    for key in (image_map).0.keys() {
        if key.contains(letter) {
            return (*image_map.0.get(key).unwrap()).clone();
        }
    }
    UiImage::default()
}

fn create_button_panel(commands: &mut Commands, image_map: &ImageMap) -> Entity {
    let mut buttons: Vec<Entity> = Vec::new();
    let mut button_type_iterator = LevelControlButtonType::iterator().peekable();
    let mut image_ind = 0;
    while button_type_iterator.peek().is_some() {
        let button: Entity = create_button(
            commands,
            image_map.1.get(image_ind).unwrap().clone(),
            *button_type_iterator.next().unwrap(),
        );
        image_ind += 1;
        buttons.push(button);
    }
    let panel = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                size: Size {
                    width: Val::Px(
                        5.0 * (LEVEL_DISPLAY_BUTTON_MARGIN * 2.0 + LEVEL_DISPLAY_BUTTON_SIZE),
                    ),
                    height: Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN * 2.0 + LEVEL_DISPLAY_BUTTON_SIZE),
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .id();
    commands.entity(panel).push_children(&buttons);
    panel
}

fn create_button(
    commands: &mut Commands,
    img: UiImage,
    button_type: LevelControlButtonType,
) -> Entity {
    return commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                    height: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                },
                margin: UiRect::all(Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)),
                ..Default::default()
            },
            image: img,
            ..Default::default()
        })
        .insert(button_type)
        .id();
}

fn create_info_panel(commands: &mut Commands, image_map: &ImageMap, perl_label: String) -> Entity {
    commands
        .spawn((
            TextBundle::from_sections([
                TextSection::new(
                    format!("{perl_label}: "),
                    TextStyle {
                        font: image_map.2.get(0).unwrap().clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: image_map.2.get(0).unwrap().clone(),
                    font_size: 40.0,
                    color: Color::BLACK,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ScoreText,
        ))
        .id()
}

fn level_control_button_system(
    mut interaction_query: Query<
        (&Interaction, &LevelControlButtonType, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut script_res: ResMut<ScriptRes>,
    mut game: ResMut<Game>,
) {
    for (interaction, button_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match *button_type {
                    LevelControlButtonType::Play => {
                        script_res.set_run_status(ScriptRunStatus::Running);
                    }
                    LevelControlButtonType::StepBack => {
                        script_res.set_run_status(ScriptRunStatus::BackwardOnce);
                    }
                    LevelControlButtonType::StepForward => {
                        script_res.set_run_status(ScriptRunStatus::ForwardOnce);
                    }
                    LevelControlButtonType::Pause => {
                        script_res.set_run_status(ScriptRunStatus::Paused);
                    }
                    LevelControlButtonType::Stop => {
                        reset_level(&mut script_res, &mut game);
                    }
                };
                *color = BackgroundColor(Color::YELLOW);
            }
            Interaction::Hovered => {
                match *button_type {
                    LevelControlButtonType::Play => {
                        info!("Button: {:?}, Action: {:?}", button_type, interaction);
                    }
                    LevelControlButtonType::StepBack => {
                        info!("Button: {:?}, Action: {:?}", button_type, interaction);
                    }
                    LevelControlButtonType::StepForward => {
                        info!("Button: {:?}, Action: {:?}", button_type, interaction);
                    }
                    LevelControlButtonType::Pause => {
                        info!("Button: {:?}, Action: {:?}", button_type, interaction);
                    }
                    LevelControlButtonType::Stop => {
                        info!("Button: {:?}, Action: {:?}", button_type, interaction);
                    }
                };
                *color = BackgroundColor(Color::AQUAMARINE);
            }
            Interaction::None => {
                *color = BackgroundColor::default();
            }
        }
    }
}
