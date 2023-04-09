use bevy::prelude::*;
use std::collections::HashMap;

pub struct ImageHandlerPlugin;

#[derive(Resource)]
pub struct ImageMap(
    pub HashMap<String, UiImage>,
    pub Vec<UiImage>,
    pub Vec<Handle<Font>>,
);

impl Plugin for ImageHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_images);
    }
}

fn load_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cells
    let mut image_map: HashMap<String, UiImage> = HashMap::new();

    // Big blocks
    image_map.insert(
        "Z".to_string(),
        UiImage(asset_server.load("blocks/big_block.png")),
    );
    image_map.insert(
        "QWER".to_string(),
        UiImage(asset_server.load("blocks/big_left_triangle.png")),
    );
    image_map.insert(
        "TYUI".to_string(),
        UiImage(asset_server.load("blocks/big_right_triangle.png")),
    );
    image_map.insert(
        "ASDF".to_string(),
        UiImage(asset_server.load("blocks/big_left_half_slope.png")),
    );
    image_map.insert(
        "GHJK".to_string(),
        UiImage(asset_server.load("blocks/big_right_half_slope.png")),
    );

    // Small blocks
    image_map.insert(
        "zxcv".to_string(),
        UiImage(asset_server.load("blocks/small_block.png")),
    );
    image_map.insert(
        "qwer".to_string(),
        UiImage(asset_server.load("blocks/small_left_triangle.png")),
    );
    image_map.insert(
        "tyui".to_string(),
        UiImage(asset_server.load("blocks/small_right_triangle.png")),
    );
    image_map.insert(
        "asdf".to_string(),
        UiImage(asset_server.load("blocks/small_left_half_slope.png")),
    );
    image_map.insert(
        "ghjk".to_string(),
        UiImage(asset_server.load("blocks/small_right_half_slope.png")),
    );
    image_map.insert(
        "bnml".to_string(),
        UiImage(asset_server.load("blocks/lower_half_block.png")),
    );
    image_map.insert(
        "BN".to_string(),
        UiImage(asset_server.load("blocks/center_half_block.png")),
    );

    // Pawns
    image_map.insert(
        "p".to_string(),
        UiImage(asset_server.load("pawns/green.png")),
    );
    image_map.insert(
        "P".to_string(),
        UiImage(asset_server.load("pawns/orange.png")),
    );

    // Extra
    image_map.insert(
        "X".to_string(),
        UiImage(asset_server.load("extra/stone.png")),
    );
    image_map.insert(
        "o".to_string(),
        UiImage(asset_server.load("extra/closed_shell.png")),
    );
    image_map.insert(
        "O".to_string(),
        UiImage(asset_server.load("extra/open_shell.png")),
    );
    image_map.insert(
        "C".to_string(),
        UiImage(asset_server.load("extra/perl.png")),
    );
    image_map.insert(
        "V".to_string(),
        UiImage(asset_server.load("extra/hexagon.png")),
    );

    // Buttons
    let mut buttons: Vec<UiImage> = Vec::new();
    buttons.push(UiImage(asset_server.load("buttons/start.png")));
    buttons.push(UiImage(asset_server.load("buttons/step_back.png")));
    buttons.push(UiImage(asset_server.load("buttons/step_forward.png")));
    buttons.push(UiImage(asset_server.load("buttons/pause.png")));
    buttons.push(UiImage(asset_server.load("buttons/stop.png")));

    // Fonts
    let mut fonts: Vec<Handle<Font>> = Vec::new();
    fonts.push(asset_server.load("fonts/NotoSans-Regular.ttf"));

    commands.insert_resource(ImageMap(image_map, buttons, fonts));
}
