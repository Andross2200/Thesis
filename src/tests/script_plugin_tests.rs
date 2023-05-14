use std::borrow::BorrowMut;

use bevy::{
    prelude::{App, ImageBundle, Quat, Query, ResMut, Transform, With, Without, World},
    ui::{PositionType, Size, Style, UiRect, Val},
};

use crate::{
    model::game_model::game::{create_level_cell, Game, GameMode, LevelCell},
    utilities::script_plugin::*,
    view::game_view::level_view::{
        CellCollider, CellMovable, GreenPawn, OrangePawn, Perl, ShellType,
    },
};

const SHIFT_TO_RIGHT: f32 = 7.0;
const SHIFT_DOWN: f32 = 20.0;

#[test]
fn script_string_empty_nothing_should_happen() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;

    app.update();
    assert_eq!(
        app.world.resource::<ScriptRes>().run_status,
        ScriptRunStatus::Reset,
        "script run status should be set to Reset"
    );

    assert_eq!(
        app.world.resource::<ScriptRes>().run_index,
        0,
        "run index should stay at 0"
    );
}

#[test]
fn script_string_has_invalid_step_nothing_should_happen() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("invalid".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    app.update();
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(actual.position, expected);
}

#[test]
fn script_string_has_valid_step_for_nonexisting_pawn_nothing_should_happen() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mor".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    app.update();
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(actual.position, expected);
}

#[test]
fn script_string_has_valid_step_for_existing_pawn() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col + 1) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    app.update();
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(actual.position, expected);
}

#[test]
fn script_string_has_several_valid_steps_run_all_script_should_modify_pawn_position() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col + 3) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    for _ in 0..3 {
        app.update();
    }
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(actual.position, expected);
    assert_eq!(
        app.world.resource::<ScriptRes>().run_status,
        ScriptRunStatus::Reset,
        "script run status should be set to Reset"
    );
}

#[test]
fn script_string_has_several_valid_steps_run_one_step_forward_should_modify_pawn_position() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::ForwardOnce;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col + 1) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    app.update();
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(actual.position, expected);
    assert_eq!(
        app.world.resource::<ScriptRes>().run_status,
        ScriptRunStatus::Paused,
        "script run status should be set to Paused"
    );
}

#[test]
fn script_string_has_several_valid_steps_run_one_step_back_should_return_pawn_to_prev_position() {
    let mut app = App::new();
    app.add_system(run_script);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::ForwardOnce;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    app.update();
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::BackwardOnce;
    app.update();
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(actual.position, expected);
    assert_eq!(
        app.world.resource::<ScriptRes>().run_status,
        ScriptRunStatus::Paused,
        "script run status should be set to Paused"
    );
}

#[test]
fn pause_running_script_should_change_run_status_to_pause_and_move_pawn_by_one() {
    let mut app = App::new();
    app.add_system(run_script);
    app.add_system(pause_system);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    app.world
        .resource_mut::<ScriptRes>()
        .script
        .push("mgr".to_string());
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());

    let expected = UiRect {
        left: Val::Px(image_size * (pawn_col + 1) as f32 + 0.0 + SHIFT_TO_RIGHT),
        top: Val::Px(image_size * (pawn_row) as f32 + 0.0 + SHIFT_DOWN),
        ..Default::default()
    };
    for _ in 0..3 {
        app.update();
    }
    let actual = app
        .world
        .get::<Style>(pawn_cell_data.cell_entity.expect("Entity should exist"))
        .expect("Enity should have style");
    assert_eq!(
        actual.position, expected,
        "top should be increased by {image_size}"
    );
    assert_eq!(
        app.world.resource::<ScriptRes>().run_status,
        ScriptRunStatus::Paused,
        "script run status should be set to Paused"
    );
}

#[test]
fn reset_level_should_reset_vars_and_set_status_to_reset() {
    let mut app = App::new();
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());
    app.world.resource_mut::<ScriptRes>().run_index = 5;
    app.world.resource_mut::<ScriptRes>().run_status = ScriptRunStatus::Running;
    app.world.resource_mut::<Game>().collected_perls = 3;
    app.world.resource_mut::<Game>().solution_steps = 5;

    app.add_system(system_to_test_reset_level);
    app.update();
    assert_eq!(
        app.world.resource_mut::<ScriptRes>().run_index,
        0,
        "run_index should be set to 0"
    );
    assert_eq!(
        app.world.resource_mut::<ScriptRes>().run_status,
        ScriptRunStatus::Reset,
        "run_status should be set to Reset"
    );
    assert_eq!(
        app.world.resource_mut::<Game>().collected_perls,
        0,
        "collected_perls should be set to 0"
    );
    assert_eq!(
        app.world.resource_mut::<Game>().solution_steps,
        0,
        "solution_steps should be set to 0"
    );
}

#[test]
fn move_pawn_up_should_update_top_position_by_subtracting_image_size() {
    let mut app = App::new();
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());
    let pawn_id = pawn_cell_data.cell_entity.unwrap();

    let mut expect_binding = app.world.entity_mut(pawn_id);
    let expect_pawn = expect_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let expect = expect_pawn
        .position
        .top
        .try_sub(Val::Px(image_size))
        .expect("Subtraction should be successful");

    move_pawn(expect_pawn, Direction::Up, image_size);

    let mut actual_binding = app.world.entity_mut(pawn_id);
    let actual_pawn = actual_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let actual = actual_pawn.position.top;
    assert!(
        expect.eq(&actual),
        "top value should change; Now expect: {:?}, actual: {:?}",
        expect,
        actual
    );
}

#[test]
fn move_pawn_down_should_update_top_position_by_subtracting_image_size() {
    let mut app = App::new();
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());
    let pawn_id = pawn_cell_data.cell_entity.unwrap();

    let mut expect_binding = app.world.entity_mut(pawn_id);
    let expect_pawn = expect_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let expect = expect_pawn
        .position
        .top
        .try_add(Val::Px(image_size))
        .expect("Subtraction should be successful");

    move_pawn(expect_pawn, Direction::Down, image_size);

    let mut actual_binding = app.world.entity_mut(pawn_id);
    let actual_pawn = actual_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let actual = actual_pawn.position.top;
    assert!(
        expect.eq(&actual),
        "top value should change; Now expect: {:?}, actual: {:?}",
        expect,
        actual
    );
}

#[test]
fn move_pawn_left_should_update_left_position_by_subtracting_image_size() {
    let mut app = App::new();
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());
    let pawn_id = pawn_cell_data.cell_entity.unwrap();

    let mut expect_binding = app.world.entity_mut(pawn_id);
    let expect_pawn = expect_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let expect = expect_pawn
        .position
        .left
        .try_sub(Val::Px(image_size))
        .expect("Subtraction should be successful");

    move_pawn(expect_pawn, Direction::Left, image_size);

    let mut actual_binding = app.world.entity_mut(pawn_id);
    let actual_pawn = actual_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let actual = actual_pawn.position.left;
    assert!(
        expect.eq(&actual),
        "top value should change; Now expect: {:?}, actual: {:?}",
        expect,
        actual
    );
}

#[test]
fn move_pawn_right_should_update_left_position_by_subtracting_image_size() {
    let mut app = App::new();
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );
    assert!(pawn_cell_data.cell_entity.is_some());
    let pawn_id = pawn_cell_data.cell_entity.unwrap();

    let mut expect_binding = app.world.entity_mut(pawn_id);
    let expect_pawn = expect_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let expect = expect_pawn
        .position
        .left
        .try_add(Val::Px(image_size))
        .expect("Subtraction should be successful");

    move_pawn(expect_pawn, Direction::Right, image_size);

    let mut actual_binding = app.world.entity_mut(pawn_id);
    let actual_pawn = actual_binding
        .get_mut::<Style>()
        .expect("Pawn should exist");
    let actual = actual_pawn.position.left;
    assert!(
        expect.eq(&actual),
        "top value should change; Now expect: {:?}, actual: {:?}",
        expect,
        actual
    );
}

#[test]
fn check_location_should_give_true_to_right_and_false_to_up() {
    let mut app = App::new();
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );

    let wall_row = 2;
    let wall_col = 4;
    let mut wall_cell_data = create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut wall_cell_data,
        &mut app.world,
        image_size,
        wall_row as u32,
        wall_col as u32,
    );
    app.add_system(system_to_test_checking_location);
    app.update();
}

#[test]
fn reset_images_should_reset_position_to_initial() {
    let mut app = App::new();
    app.add_system(reset_images);
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    app.insert_resource(ScriptRes::new());

    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );

    let pawn_id = pawn_cell_data.cell_entity.unwrap();
    let mut pawn_binding = app.world.entity_mut(pawn_id);
    let mut pawn = pawn_binding.get_mut::<Style>().expect("Pawn should exist");
    pawn.position
        .left
        .try_add_assign(Val::Px(image_size))
        .expect("Addition should be successful");

    app.update();

    assert_eq!(
        Val::Px(307.0),
        app.world
            .entity_mut(pawn_id)
            .get_mut::<Style>()
            .expect("Pawn should exist")
            .position
            .left,
        "the pawn should return to initial position of 307"
    );
}

#[test]
fn get_perl_at_pawn_no_perl_should_change_nothing() {
    let mut app = App::new();
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );

    let perl_row = 2;
    let perl_col = 4;
    let mut perl_cell_data = create_level_cell(
        'C',
        0.0,
        image_size * 0.5,
        image_size * 0.5,
        image_size / 4.0,
        image_size / 4.0,
    );
    create_cell(
        &mut perl_cell_data,
        &mut app.world,
        image_size,
        perl_row as u32,
        perl_col as u32,
    );

    app.add_system(system_to_test_get_perl_at_pawn);
    app.update();

    let perl_id = perl_cell_data.cell_entity.unwrap();
    let perl_binding = app.world.entity_mut(perl_id);
    let status = perl_binding.get::<Perl>().expect("Perl should exist");
    assert_eq!(
        Perl::NotCollected,
        *status,
        "status of perl should not change"
    );
}

#[test]
fn get_perl_at_pawn_perl_present_should_change_to_collected() {
    let mut app = App::new();
    app.insert_resource(Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1CpZ/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    ));
    let image_size = 100.0;
    let pawn_row = 2;
    let pawn_col = 3;
    let mut pawn_cell_data = create_level_cell('p', 0.0, image_size, image_size, 0.0, 0.0);
    create_cell(
        &mut pawn_cell_data,
        &mut app.world,
        image_size,
        pawn_row as u32,
        pawn_col as u32,
    );

    let perl_row = 2;
    let perl_col = 3;
    let mut perl_cell_data = create_level_cell(
        'C',
        0.0,
        image_size * 0.5,
        image_size * 0.5,
        image_size / 4.0,
        image_size / 4.0,
    );
    create_cell(
        &mut perl_cell_data,
        &mut app.world,
        image_size,
        perl_row as u32,
        perl_col as u32,
    );

    app.add_system(system_to_test_get_perl_at_pawn);
    app.update();

    let perl_id = perl_cell_data.cell_entity.unwrap();
    let perl_binding = app.world.entity_mut(perl_id);
    let status = perl_binding.get::<Perl>().expect("Perl should exist");
    assert_eq!(Perl::Collected, *status, "status of perl should not change");
}

fn system_to_test_get_perl_at_pawn(
    mut gpawn: Query<
        &mut Style,
        (
            With<GreenPawn>,
            Without<CellMovable>,
            Without<Perl>,
            Without<OrangePawn>,
        ),
    >,
    mut perls: Query<(&mut Style, &mut Perl), With<Perl>>,
    mut game: ResMut<Game>,
) {
    for pawn in &mut gpawn {
        get_perl_at_pawn(&pawn, &mut perls, game.borrow_mut(), 100.0, false);
    }
}

fn system_to_test_checking_location(
    mut gpawn: Query<
        &mut Style,
        (
            With<GreenPawn>,
            Without<CellMovable>,
            Without<Perl>,
            Without<OrangePawn>,
        ),
    >,
    mut walls: Query<
        &mut Style,
        (
            With<CellCollider>,
            Without<CellMovable>,
            Without<Perl>,
            Without<GreenPawn>,
            Without<OrangePawn>,
        ),
    >,
) {
    let image_size = 100.0;
    for pawn in &mut gpawn {
        let check_right = check_location(&pawn, &Direction::Right, &mut walls, image_size);
        assert!(check_right, "position to right of pawn is occupied");
        let check_up = check_location(&pawn, &Direction::Up, &mut walls, image_size);
        assert!(!check_up, "position to top of pawn is free");
    }
}

fn system_to_test_reset_level(mut script_res: ResMut<ScriptRes>, mut game: ResMut<Game>) {
    reset_level(&mut script_res, &mut game);
}

fn pause_system(mut script_res: ResMut<ScriptRes>) {
    if script_res.run_index == 1 {
        script_res.run_status = ScriptRunStatus::Paused;
    }
}

fn create_cell(cell_data: &mut LevelCell, world: &mut World, image_size: f32, i: u32, j: u32) {
    let cell = world
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
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_z(cell_data.angle)),
            ..Default::default()
        })
        .id();
    if "pP".to_string().contains(cell_data.letter) {
        if "p".to_string().contains(cell_data.letter) {
            world.entity_mut(cell).insert(GreenPawn);
        } else {
            world.entity_mut(cell).insert(OrangePawn);
        }
    } else if "XV".to_string().contains(cell_data.letter) {
        world.entity_mut(cell).insert(CellMovable);
    } else if "o".to_string().contains(cell_data.letter) {
        world.entity_mut(cell).insert(ShellType::Closed);
    } else if "O".to_string().contains(cell_data.letter) {
        world.entity_mut(cell).insert(ShellType::Open);
    } else if "C".to_string().contains(cell_data.letter) {
        world.entity_mut(cell).insert(Perl::NotCollected);
    } else {
        world.entity_mut(cell).insert(CellCollider);
    }
    cell_data.cell_entity = Some(cell);
}
