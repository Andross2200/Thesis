use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, NodeBundle, Plugin, Res, SystemSet, TextBundle, ButtonBundle, Query, Changed, With, Button, EventWriter, ResMut, State,
    },
    text::TextStyle,
    ui::{AlignItems, BackgroundColor, FlexDirection, PositionType, Size, Style, UiRect, Val, JustifyContent, Interaction}, app::AppExit,
};

use crate::view::{despawn_screen, image_handler::ImageMap, GameState};

const BUTTON_MARGIN: f32 = 20.0;

#[derive(Debug, Component)]
enum MenuButtonAction {
    Tutorial,
    Challenge,
    Multiplayer,
    LanguageBack,
    LanguageForward,
    PlayerBack,
    PlayerForward,
    Quit
}

#[derive(Debug, Component)]
struct MainMenuView;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(init_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(despawn_screen::<MainMenuView>),
            )
            .add_system(menu_actions);
    }
}

fn init_setup(mut commands: Commands, image_handler: Res<ImageMap>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(10.0),
                    left: Val::Percent(25.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(50.0),
                    height: Val::Percent(80.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BEIGE),
            ..Default::default()
        })
        .insert(MainMenuView)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Title",
                TextStyle {
                    font: image_handler.2.get(1).unwrap().clone(),
                    font_size: 80.0,
                    color: Color::BLACK,
                },
            ).with_style(Style {
                margin: UiRect { top: Val::Px(40.0), bottom: Val::Px(50.0), ..Default::default() },
                ..Default::default()
            }));

            // Tutorial mode Button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0)
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Tutorial",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 50.0,
                        color: Color::BLACK
                    }
                ));
            }).insert(MenuButtonAction::Tutorial);

            // Challenge mode Button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0)
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Challenge",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 50.0,
                        color: Color::BLACK
                    }
                ));
            }).insert(MenuButtonAction::Challenge);

            // Multiplayer mode Button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0)
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Multiplayer",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 50.0,
                        color: Color::BLACK
                    }
                ));
            }).insert(MenuButtonAction::Multiplayer);

            // Change language panel
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(330.0),
                        height: Val::Px(60.0)
                    },
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            }).with_children(|node| {
                node.spawn(ButtonBundle {
                    style: Style {
                        size : Size::new(Val::Px(50.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: image_handler.1.get(7).unwrap().clone(),
                    ..Default::default()
                }).insert(MenuButtonAction::LanguageBack);
                node.spawn(TextBundle::from_section(
                    "Language: English",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 30.0,
                        color: Color::BLACK
                    }
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                }));
                node.spawn(ButtonBundle {
                    style: Style {
                        size : Size::new(Val::Px(50.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: image_handler.1.get(8).unwrap().clone(),
                    ..Default::default()
                }).insert(MenuButtonAction::LanguageForward);
            });

            // Change player panel
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(330.0),
                        height: Val::Px(60.0)
                    },
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            }).with_children(|node| {
                node.spawn(ButtonBundle {
                    style: Style {
                        size : Size::new(Val::Px(50.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: image_handler.1.get(7).unwrap().clone(),
                    ..Default::default()
                }).insert(MenuButtonAction::PlayerBack);
                node.spawn(TextBundle::from_section(
                    "Player: Bobby436",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 30.0,
                        color: Color::BLACK
                    }
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                }));
                node.spawn(ButtonBundle {
                    style: Style {
                        size : Size::new(Val::Px(50.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: image_handler.1.get(8).unwrap().clone(),
                    ..Default::default()
                }).insert(MenuButtonAction::PlayerForward);
            });

            // Exit game button
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0)
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Exit to Desktop",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 35.0,
                        color: Color::BLACK
                    }
                ));
            }).insert(MenuButtonAction::Quit);
        });
}

fn menu_actions(
    mut interaction_query: Query<(&Interaction, &MenuButtonAction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>
) {
    for (interaction, button_action, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match *button_action {
                    MenuButtonAction::Tutorial => {
                        game_state.set(GameState::LevelSelector).unwrap();
                    },
                    MenuButtonAction::Challenge => {},
                    MenuButtonAction::Multiplayer => {},
                    MenuButtonAction::LanguageBack => {},
                    MenuButtonAction::LanguageForward => {},
                    MenuButtonAction::PlayerBack => {},
                    MenuButtonAction::PlayerForward => {},
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    },
                }
            },
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            },
            Interaction::None => {
                *back_color = BackgroundColor(Color::WHITE);
            },
        }
    }
}