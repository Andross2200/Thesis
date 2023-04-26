use std::fs;

use bevy::prelude::{Commands, Plugin, Res, Resource};
use serde::{Deserialize, Serialize};

use super::database_plugin::ConfigResource;

const LANGUAGE_FILE_FOLDER: &str = "assets/languages/";

#[derive(Debug, Serialize, Deserialize)]
pub struct MainMenu {
    pub tutorial_button: String,
    pub challenge_button: String,
    pub multiplayer_button: String,
    pub scoreboard_button: String,
    pub language_panel: String,
    pub player_panel: String,
    pub create_new_player_button: String,
    pub exit_button: String,
    pub reload_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelSelector {
    pub go_back_button: String,
    pub level_label: String,
    pub not_completed_label: String,
    pub completed_label: Vec<String>,
    pub selected_button: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuzzleButtonPanel {
    pub label: String,
    pub buttons: Vec<String>,
    pub close_button: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameText {
    pub blocks_button_panel: String,
    pub blocks_panels_selector_buttons: Vec<String>,
    pub pawn_action_panel: PuzzleButtonPanel,
    pub move_arrows_panel_label: String,
    pub menu_panel_label: String,
    pub complete_button: String,
    pub go_back_button: String,
    pub perls_score_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreboardText {
    pub title: String,
    pub challenge_scores_title: String,
    pub go_back_button: String,
}

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct LanguageResource {
    pub main_menu: MainMenu,
    pub level_selector: LevelSelector,
    pub game: GameText,
    pub scoreboard: ScoreboardText,
}
pub struct LanguagePlugin;

impl Plugin for LanguagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_language);
    }
}

fn setup_language(mut commands: Commands, config: Res<ConfigResource>) {
    let language_file_path = LANGUAGE_FILE_FOLDER.to_string()
        + &config.languages[config.selected_language as usize]
        + ".json";
    let language_resource_json = fs::read_to_string(language_file_path).unwrap_or_else(|_| {
        panic!(
            "json file with text in {} language should be in folder \"languages\"",
            config.languages[config.selected_language as usize].as_str()
        )
    });
    let language_resource: LanguageResource = serde_json::from_str(&language_resource_json)
        .expect("Structure of json string should be valid");
    commands.insert_resource(language_resource);
}
