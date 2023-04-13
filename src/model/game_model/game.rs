use crate::{view::game_view::game_view_plugin::RedrawPuzzle, MAX_LEVEL_HEIGHT, MAX_LEVEL_WIDTH};
use bevy::prelude::*;
use simple_matrix::Matrix;
use std::{cmp::min, f32::consts::PI};

#[derive(PartialEq, Eq)]
pub enum GameCompleted {
    Yes,
    No,
}

#[derive(Default, Clone, Copy)]
pub struct LevelCell {
    pub letter: char,
    pub angle: f32,
    pub image_size_x: f32,
    pub image_size_y: f32,
    pub extra_move_x: f32,
    pub extra_move_y: f32,
    pub cell_entity: Option<Entity>,
}

#[derive(Resource)]
pub struct Game {
    pub level_id: i32,
    pub level_matrix: Matrix<LevelCell>,
    pub fen: String,
    pub rows: u32,
    pub columns: u32,
    pub collected_perls: u32,
    pub required_perls: u32,
    pub puzzle: Vec<Entity>,
    pub redraw_cond: RedrawPuzzle,
    pub selected_puzzle_piece: i32,
    pub game_completed: GameCompleted,
    pub solution_steps: i32,
    pub solution: i32
}

impl Default for Game {
    fn default() -> Self {
        Game::init_from_fen("5 5 ZZZZZ/Z1C1Z/Z1Z1Z/Z1CPZ/ZZZZZ 2".to_string(), 0)
    }
}

impl Game {
    pub fn increment_per_counter(&mut self) {
        self.collected_perls += 1;
    }

    pub fn decrement_perl_counter(&mut self) {
        self.collected_perls -= 1;
    }
}

impl Game {
    pub fn init_from_fen(fen: String, id: i32) -> Game {
        let mut iter = fen.split_whitespace();
        let num_of_rows: u32 = iter.next().unwrap().parse().unwrap();
        let num_of_columns: u32 = iter.next().unwrap().parse().unwrap();
        let binding = String::from(iter.next().unwrap());
        let goal: u32 = iter.next().unwrap().parse().unwrap();
        let mut level_iter = binding.split('/').peekable();
        let mut matrix: Matrix<LevelCell> = Matrix::new(
            num_of_rows.try_into().unwrap(),
            num_of_columns.try_into().unwrap(),
        );
        let image_size = min(
            MAX_LEVEL_WIDTH as u32 / num_of_columns,
            MAX_LEVEL_HEIGHT as u32 / num_of_rows,
        ) as f32;
        for i in 0..num_of_rows {
            let mut line_chars = level_iter.next().unwrap().chars().peekable();
            let mut col_counter: u32 = 0;
            while Option::is_some(&line_chars.peek()) {
                let c: char = line_chars.next().unwrap();
                let mut cell: LevelCell;
                if c.is_alphabetic() {
                    match c {
                        'Z' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'Q' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'W' => {
                            cell = create_level_cell(c, PI / 2.0, image_size, image_size, 0.0, 0.0);
                        }
                        'E' => {
                            cell = create_level_cell(c, PI, image_size, image_size, 0.0, 0.0);
                        }
                        'R' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size,
                                0.0,
                                0.0,
                            );
                        }
                        'T' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'Y' => {
                            cell = create_level_cell(c, PI / 2.0, image_size, image_size, 0.0, 0.0);
                        }
                        'U' => {
                            cell = create_level_cell(c, PI, image_size, image_size, 0.0, 0.0);
                        }
                        'I' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size,
                                0.0,
                                0.0,
                            );
                        }
                        'A' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'S' => {
                            cell = create_level_cell(c, PI / 2.0, image_size, image_size, 0.0, 0.0);
                        }
                        'D' => {
                            cell = create_level_cell(c, PI, image_size, image_size, 0.0, 0.0);
                        }
                        'F' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size,
                                0.0,
                                0.0,
                            );
                        }
                        'G' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'H' => {
                            cell = create_level_cell(c, PI / 2.0, image_size, image_size, 0.0, 0.0);
                        }
                        'J' => {
                            cell = create_level_cell(c, PI, image_size, image_size, 0.0, 0.0);
                        }
                        'K' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size,
                                0.0,
                                0.0,
                            );
                        }
                        'z' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.5,
                                0.0,
                            );
                        }
                        'x' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                            );
                        }
                        'c' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                            );
                        }
                        'v' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'q' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'w' => {
                            cell = create_level_cell(
                                c,
                                PI * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                                0.0,
                            );
                        }
                        'e' => {
                            cell = create_level_cell(
                                c,
                                PI,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                            );
                        }
                        'r' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                            );
                        }
                        't' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size * 0.5,
                            );
                        }
                        'y' => {
                            cell = create_level_cell(
                                c,
                                PI / 2.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'u' => {
                            cell = create_level_cell(
                                c,
                                PI,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                                0.0,
                            );
                        }
                        'i' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'a' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        's' => {
                            cell = create_level_cell(
                                c,
                                PI / 2.0,
                                image_size,
                                image_size * 0.5,
                                -image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'd' => {
                            cell = create_level_cell(c, PI, image_size, image_size * 0.5, 0.0, 0.0);
                        }
                        'f' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size * 0.5,
                                image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'g' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'h' => {
                            cell = create_level_cell(
                                c,
                                PI * 0.5,
                                image_size,
                                image_size * 0.5,
                                -image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'j' => {
                            cell = create_level_cell(
                                c,
                                PI,
                                image_size,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'k' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size * 0.5,
                                image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'b' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'n' => {
                            cell = create_level_cell(
                                c,
                                PI * 0.5,
                                image_size,
                                image_size * 0.5,
                                -image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'm' => {
                            cell = create_level_cell(c, PI, image_size, image_size * 0.5, 0.0, 0.0);
                        }
                        'l' => {
                            cell = create_level_cell(
                                c,
                                PI * 3.0 / 2.0,
                                image_size,
                                image_size * 0.5,
                                image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'B' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size,
                                image_size / 4.0,
                                0.0,
                            );
                        }
                        'N' => {
                            cell = create_level_cell(
                                c,
                                PI * 0.5,
                                image_size * 0.5,
                                image_size,
                                image_size / 4.0,
                                0.0,
                            );
                        }
                        'p' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'P' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'X' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'o' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size,
                                image_size * 0.5,
                                0.0,
                                image_size * 0.5,
                            );
                        }
                        'O' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        'C' => {
                            cell = create_level_cell(
                                c,
                                0.0,
                                image_size * 0.5,
                                image_size * 0.5,
                                image_size / 4.0,
                                image_size / 4.0,
                            );
                        }
                        'V' => {
                            cell = create_level_cell(c, 0.0, image_size, image_size, 0.0, 0.0);
                        }
                        _ => {
                            error!("empty cell");
                            continue;
                        }
                    }
                    matrix.set(i.try_into().unwrap(), col_counter.try_into().unwrap(), cell);
                    col_counter += 1;
                } else if c.is_numeric() {
                    let num = c.to_digit(10).unwrap();
                    for _n in 1..num + 1 {
                        cell = create_level_cell('_', 0.0, image_size, image_size, 0.0, 0.0);
                        matrix.set(i.try_into().unwrap(), col_counter.try_into().unwrap(), cell);
                        col_counter += 1;
                    }
                }
            }
        }
        Game {
            level_id: id,
            level_matrix: matrix,
            fen,
            rows: num_of_rows,
            columns: num_of_columns,
            collected_perls: 0,
            required_perls: goal,
            puzzle: Vec::new(),
            redraw_cond: RedrawPuzzle::No,
            selected_puzzle_piece: -1,
            game_completed: GameCompleted::No,
            solution_steps: 0,
            solution: 0
        }
    }
}

fn create_level_cell(
    letter: char,
    angle: f32,
    image_size_x: f32,
    image_size_y: f32,
    extra_move_x: f32,
    extra_move_y: f32,
) -> LevelCell {
    LevelCell {
        letter,
        angle,
        image_size_x,
        image_size_y,
        extra_move_x,
        extra_move_y,
        cell_entity: None,
    }
}
