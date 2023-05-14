use simple_matrix::Matrix;

use crate::model::game_model::game::*;

#[test]
#[should_panic]
fn fen_string_is_empty_should_panic() {
    Game::init_from_fen("".to_string(), 0, GameMode::Tutorial);
}
#[test]
#[should_panic]
fn num_of_rows_or_columns_missing_should_panic() {
    Game::init_from_fen(
        "5 ZZZZZ/Z3Z/Z1C1Z/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    );
}

#[test]
#[should_panic]
fn nonalphanumeric_char_should_panic() {
    Game::init_from_fen(
        "5 5 ZZZ@Z/Z3Z/Z1C1Z/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    );
}

#[test]
#[should_panic]
fn invalid_char_should_panic() {
    Game::init_from_fen(
        "5 5 ZZZLZ/Z3Z/Z1C1Z/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    );
}

#[test]
#[should_panic]
fn num_of_perls_missing_should_panic() {
    Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1C1Z/Z3Z/ZZZZZ".to_string(),
        0,
        GameMode::Tutorial,
    );
}

#[test]
fn valid_fen_should_create_new_struct() {
    let game = Game::init_from_fen(
        "5 5 ZZZZZ/Z3Z/Z1C1Z/Z3Z/ZZZZZ 1".to_string(),
        0,
        GameMode::Tutorial,
    );
    assert_eq!(
        game.rows, 5,
        "number of rows should equal to one in fen string"
    );
    assert_eq!(
        game.columns, 5,
        "number of columns should equal to one in fen string"
    );
    assert_eq!(
        game.required_perls, 1,
        "number of required perls should equal to one in fen string"
    );
    assert_eq!(game.level_id, 0, "id of level should be 1");
    assert_eq!(
        game.game_mode,
        GameMode::Tutorial,
        "game mode should be tutorial"
    );
    let expected_matrix = create_matrix();
    assert_eq!(
        game.level_matrix, expected_matrix,
        "content of matrix should be created according to fen"
    );
}

fn create_matrix() -> Matrix<LevelCell> {
    let image_size = 100.0;
    let mut matrix: Matrix<LevelCell> = Matrix::new(5, 5);
    for j in 0..5 {
        matrix.set(
            0,
            j,
            create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
        );
        matrix.set(
            4,
            j,
            create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
        );
    }
    matrix.set(
        1,
        0,
        create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
    );
    for j in 1..4 {
        matrix.set(
            1,
            j,
            create_level_cell('_', 0.0, image_size, image_size, 0.0, 0.0),
        );
    }
    matrix.set(
        1,
        4,
        create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
    );

    matrix.set(
        3,
        0,
        create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
    );
    for j in 1..4 {
        matrix.set(
            3,
            j,
            create_level_cell('_', 0.0, image_size, image_size, 0.0, 0.0),
        );
    }
    matrix.set(
        3,
        4,
        create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
    );

    matrix.set(
        2,
        0,
        create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
    );
    matrix.set(
        2,
        1,
        create_level_cell('_', 0.0, image_size, image_size, 0.0, 0.0),
    );
    matrix.set(
        2,
        2,
        create_level_cell(
            'C',
            0.0,
            image_size * 0.5,
            image_size * 0.5,
            image_size / 4.0,
            image_size / 4.0,
        ),
    );
    matrix.set(
        2,
        3,
        create_level_cell('_', 0.0, image_size, image_size, 0.0, 0.0),
    );
    matrix.set(
        2,
        4,
        create_level_cell('Z', 0.0, image_size, image_size, 0.0, 0.0),
    );
    matrix
}
