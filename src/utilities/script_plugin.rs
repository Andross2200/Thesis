#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::{
    borrow::{Borrow, BorrowMut},
    cmp::min,
    collections::HashMap,
};

use bevy::time::FixedTimestep;
use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{
    model::game_model::game::Game,
    view::game_view::level_view::{CellCollider, CellMovable, GreenPawn, OrangePawn, Perl},
    MAX_LEVEL_HEIGHT, MAX_LEVEL_WIDTH,
};
use crate::{model::game_model::game::GameCompleted, view::game_view::level_view::ScoreText};
use crate::{SHIFT_DOWN, SHIFT_TO_RIGHT};

const TIMESTEP_1_PER_SECOND: f64 = 1.0;

#[derive(PartialEq, Eq, Debug)]
pub enum ScriptRunStatus {
    Stopped,
    Running,
    Paused,
    Reset,
    ForwardOnce,
    BackwardOnce,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
pub struct ScriptRes {
    pub vars: HashMap<String, String>,
    pub script: Vec<String>,
    pub run_status: ScriptRunStatus,
    pub run_index: usize,
}

impl ScriptRes {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ScriptRes {
            vars: HashMap::new(),
            script: Vec::new(),
            run_status: ScriptRunStatus::Stopped,
            run_index: 0,
        }
    }

    pub fn set_run_status(&mut self, new_status: ScriptRunStatus) {
        self.run_status = new_status;
    }
}

pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScriptRes::new())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                    .with_system(run_script),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(reset_images_cond)
                    .with_system(reset_images),
            );
    }
}

pub fn run_script(
    mut gpawn: Query<
        &mut Style,
        (
            With<GreenPawn>,
            Without<CellMovable>,
            Without<Perl>,
            Without<OrangePawn>,
        ),
    >,
    mut opawn: Query<
        &mut Style,
        (
            With<OrangePawn>,
            Without<CellMovable>,
            Without<Perl>,
            Without<GreenPawn>,
        ),
    >,
    mut perls: Query<(&mut Style, &mut Perl), With<Perl>>,
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
    mut script_res: ResMut<ScriptRes>,
    mut game: ResMut<Game>,
    mut score_text: Query<&mut Text, With<ScoreText>>,
) {
    let image_size = min(
        MAX_LEVEL_WIDTH as u32 / game.columns,
        MAX_LEVEL_HEIGHT as u32 / game.rows,
    ) as f32;

    if script_res.run_index < script_res.script.len()
        && (script_res.run_status == ScriptRunStatus::Running
            || script_res.run_status == ScriptRunStatus::ForwardOnce
            || script_res.run_status == ScriptRunStatus::BackwardOnce)
    {
        let run_backwards = script_res.run_status == ScriptRunStatus::BackwardOnce;
        if run_backwards {
            script_res.run_index -= 1;
        }
        for mut green_pawn in &mut gpawn {
            match script_res
                .script
                .get(script_res.run_index)
                .unwrap()
                .as_str()
            {
                "mgd" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Down
                    } else {
                        Direction::Up
                    };
                    if !check_location(
                        green_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(green_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "mgu" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Up
                    } else {
                        Direction::Down
                    };
                    if !check_location(
                        green_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(green_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "mgl" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Left
                    } else {
                        Direction::Right
                    };
                    if !check_location(
                        green_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(green_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "mgr" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Right
                    } else {
                        Direction::Left
                    };
                    if !check_location(
                        green_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(green_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "cgp" => {
                    get_perl_at_pawn(
                        green_pawn.borrow_mut(),
                        perls.borrow_mut(),
                        &mut game,
                        image_size,
                        run_backwards,
                    );
                    game.solution_steps += 1;
                }
                _ => {}
            }
        }
        for mut orange_pawn in &mut opawn {
            match script_res
                .script
                .get(script_res.run_index)
                .unwrap()
                .as_str()
            {
                "mod" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Down
                    } else {
                        Direction::Up
                    };
                    if !check_location(
                        orange_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(orange_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "mou" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Up
                    } else {
                        Direction::Down
                    };
                    if !check_location(
                        orange_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(orange_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "mol" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Left
                    } else {
                        Direction::Right
                    };
                    if !check_location(
                        orange_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(orange_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "mor" => {
                    let direction: Direction = if !run_backwards {
                        Direction::Right
                    } else {
                        Direction::Left
                    };
                    if !check_location(
                        orange_pawn.borrow_mut(),
                        direction.borrow(),
                        walls.borrow_mut(),
                        image_size,
                    ) {
                        move_pawn(orange_pawn, direction, image_size);
                        game.solution_steps += 1;
                    } else {
                        reset_level(&mut script_res, &mut game);
                    }
                }
                "cop" => {
                    get_perl_at_pawn(
                        orange_pawn.borrow_mut(),
                        perls.borrow_mut(),
                        &mut game,
                        image_size,
                        run_backwards,
                    );
                    game.solution_steps += 1;
                }
                _ => {}
            }
        }
        if !run_backwards {
            script_res.run_index += 1;
        }
        if script_res.run_status == ScriptRunStatus::ForwardOnce
            || script_res.run_status == ScriptRunStatus::BackwardOnce
        {
            script_res.set_run_status(ScriptRunStatus::Paused);
        }
    }
    for mut text in &mut score_text {
        text.sections[1].value = format!(
            "{}/{}",
            game.borrow().collected_perls,
            game.borrow().required_perls
        );
    }
    if script_res.run_index >= script_res.script.len()
        && (script_res.run_status == ScriptRunStatus::Running
            || script_res.run_status == ScriptRunStatus::ForwardOnce
            || script_res.run_status == ScriptRunStatus::BackwardOnce)
    {
        if game.collected_perls == game.required_perls {
            game.game_completed = GameCompleted::Yes;
            game.solution = game.solution_steps;
            reset_level(script_res.borrow_mut(), game.borrow_mut())
        } else {
            reset_level(script_res.borrow_mut(), game.borrow_mut());
            game.game_completed = GameCompleted::No;
        }
    }
}

pub fn reset_level(script_res: &mut ResMut<ScriptRes>, game: &mut ResMut<Game>) {
    script_res.run_status = ScriptRunStatus::Reset;
    script_res.run_index = 0;
    game.collected_perls = 0;
    game.solution_steps = 0;
}

pub fn move_pawn(mut pawn: Mut<Style>, direction: Direction, image_size: f32) {
    let old_pos = pawn.position;
    match direction {
        Direction::Up => {
            pawn.position = UiRect {
                left: old_pos.left,
                top: old_pos.top.try_sub(Val::Px(image_size)).unwrap(),
                ..Default::default()
            };
        }
        Direction::Down => {
            pawn.position = UiRect {
                left: old_pos.left,
                top: old_pos.top.try_add(Val::Px(image_size)).unwrap(),
                ..Default::default()
            };
        }
        Direction::Left => {
            pawn.position = UiRect {
                left: old_pos.left.try_sub(Val::Px(image_size)).unwrap(),
                top: old_pos.top,
                ..Default::default()
            };
        }
        Direction::Right => {
            pawn.position = UiRect {
                left: old_pos.left.try_add(Val::Px(image_size)).unwrap(),
                top: old_pos.top,
                ..Default::default()
            };
        }
    }
}

pub fn check_location(
    pawn: &Mut<Style>,
    direction: &Direction,
    walls: &mut Query<
        &mut Style,
        (
            With<CellCollider>,
            Without<CellMovable>,
            Without<Perl>,
            Without<GreenPawn>,
            Without<OrangePawn>,
        ),
    >,
    image_size: f32,
) -> bool {
    #[allow(unused_assignments)]
    let mut pos_to_check = UiRect::default();
    match direction {
        Direction::Up => {
            pos_to_check = UiRect {
                left: pawn.position.left,
                top: pawn.position.top.try_sub(Val::Px(image_size)).unwrap(),
                ..Default::default()
            };
        }
        Direction::Down => {
            pos_to_check = UiRect {
                left: pawn.position.left,
                top: pawn.position.top.try_add(Val::Px(image_size)).unwrap(),
                ..Default::default()
            };
        }
        Direction::Left => {
            pos_to_check = UiRect {
                left: pawn.position.left.try_sub(Val::Px(image_size)).unwrap(),
                top: pawn.position.top,
                ..Default::default()
            };
        }
        Direction::Right => {
            pos_to_check = UiRect {
                left: pawn.position.left.try_add(Val::Px(image_size)).unwrap(),
                top: pawn.position.top,
                ..Default::default()
            };
        }
    }
    for wall in walls {
        if wall
            .position
            .left
            .reflect_partial_eq(&pos_to_check.left)
            .unwrap()
            && wall
                .position
                .top
                .reflect_partial_eq(&pos_to_check.top)
                .unwrap()
        {
            return true;
        }
    }
    false
}

pub fn reset_images_cond(script_res: ResMut<ScriptRes>) -> ShouldRun {
    if script_res.run_status == ScriptRunStatus::Reset {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[allow(unused_assignments)]
pub fn reset_images(
    game: Res<Game>,
    mut gpawn: Query<
        &mut Style,
        (
            With<GreenPawn>,
            Without<CellMovable>,
            Without<Perl>,
            Without<OrangePawn>,
        ),
    >,
    mut opawn: Query<
        &mut Style,
        (
            With<OrangePawn>,
            Without<CellMovable>,
            Without<Perl>,
            Without<GreenPawn>,
        ),
    >,
    mut perls: Query<(&mut Style, &Perl), With<Perl>>,
    mut script_res: ResMut<ScriptRes>,
) {
    let image_size = min(
        MAX_LEVEL_WIDTH as u32 / game.columns,
        MAX_LEVEL_HEIGHT as u32 / game.rows,
    ) as f32;
    for (mut perl_style, mut perl_cond) in &mut perls {
        if *perl_cond == Perl::Collected {
            perl_cond = &Perl::NotCollected;
            perl_style.display = Display::Flex;
        }
    }
    for i in 0..game.rows {
        for j in 0..game.columns {
            let cell_data = game
                .level_matrix
                .get(i.try_into().unwrap(), j.try_into().unwrap())
                .unwrap();
            if "p".to_string().contains(cell_data.letter) {
                gpawn.get_single_mut().unwrap().position = UiRect {
                    left: Val::Px(
                        image_size * (j) as f32 + cell_data.extra_move_x + SHIFT_TO_RIGHT,
                    ),
                    top: Val::Px(image_size * (i) as f32 + cell_data.extra_move_y + SHIFT_DOWN),
                    ..Default::default()
                };
            }
            if "P".to_string().contains(cell_data.letter) {
                opawn.get_single_mut().unwrap().position = UiRect {
                    left: Val::Px(
                        image_size * (j) as f32 + cell_data.extra_move_x + SHIFT_TO_RIGHT,
                    ),
                    top: Val::Px(image_size * (i) as f32 + cell_data.extra_move_y + SHIFT_DOWN),
                    ..Default::default()
                };
            }
        }
    }
    script_res.run_status = ScriptRunStatus::Stopped;
}

pub fn get_perl_at_pawn(
    pawn: &Mut<Style>,
    perls: &mut Query<(&mut Style, &mut Perl), With<Perl>>,
    game: &mut ResMut<Game>,
    image_size: f32,
    backwards: bool,
) {
    let search_for: Perl = if !backwards {
        Perl::NotCollected
    } else {
        Perl::Collected
    };
    let turn_to: Perl = if !backwards {
        Perl::Collected
    } else {
        Perl::NotCollected
    };
    let new_disp = if !backwards {
        Display::None
    } else {
        Display::Flex
    };
    for (mut style, mut perl_type) in perls {
        if *perl_type == search_for
            && style
                .position
                .left
                .reflect_partial_eq(
                    &pawn
                        .position
                        .left
                        .try_add(Val::Px(image_size / 4.0))
                        .unwrap(),
                )
                .unwrap()
            && style
                .position
                .top
                .reflect_partial_eq(
                    &pawn
                        .position
                        .top
                        .try_add(Val::Px(image_size / 4.0))
                        .unwrap(),
                )
                .unwrap()
        {
            style.display = new_disp;
            *perl_type = turn_to;
            if !backwards {
                game.increment_per_counter();
            } else {
                game.decrement_perl_counter();
            }
        }
    }
}
