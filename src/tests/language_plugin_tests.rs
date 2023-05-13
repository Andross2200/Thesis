use bevy::prelude::App;

use crate::utilities::{database_plugin::ConfigResource, language_plugin::*};

#[test]
#[should_panic]
fn file_missing_should_panic() {
    let mut app = App::new();
    app.init_resource::<ConfigResource>();
    app.add_system(setup_language);
    let mut config = app.world.resource_mut::<ConfigResource>();
    config.languages.push("French".to_string());
    config.selected_language = 2;
    app.update();
}

#[test]
fn file_present_and_valid_should_create_language_resource() {
    let mut app = App::new();
    app.init_resource::<ConfigResource>();
    app.add_system(setup_language);
    app.update();
    assert!(app.world.contains_resource::<LanguageResource>());
}
