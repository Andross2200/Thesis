use mysql::{prelude::Queryable, *};

use crate::utilities::database_plugin::*;

#[test]
fn table_for_all_levels_no_solutions_should_first_element_has_no_solution() {
    let mut db_conn = create_database_connection();

    let result_vector = get_all_levels_for_player(&mut db_conn, 3);
    assert!(result_vector[0].number_of_steps.is_none());
}

#[test]
fn table_for_all_levels_some_solutions_should_have_solution_for_all_solved() {
    let mut db_conn = create_database_connection();

    let result_vector = get_all_levels_for_player(&mut db_conn, 2);
    assert!(result_vector[0].number_of_steps.is_some());
    assert!(result_vector[1].number_of_steps.is_none());
}

#[test]
fn update_score_for_tutorial_level_with_no_prev_solution_should_create_new_row() {
    let mut db_conn = create_database_connection();
    let player_id = 9;
    let level_id = 2;

    let query = format!("SELECT number_of_steps FROM tutorial_level_solutions WHERE player_id = {player_id} AND level_id = {level_id}");

    let before: Option<i32> = db_conn
        .conn
        .query_first(query.clone())
        .expect("Query should be successful");
    assert!(
        before.is_none(),
        "row for player_id {player_id} and level_id {level_id} should not exist initially"
    );

    update_score_for_tutorial_level(&mut db_conn, player_id, level_id, 5);

    let after: Option<i32> = db_conn
        .conn
        .query_first(query)
        .expect("Query should be successful");
    assert!(
        after.is_some(),
        "row for player_id {player_id} and level_id {level_id} should exist"
    );
    assert_eq!(5, after.unwrap(), "number of steps should be saved");

    let delete_query = format!("DELETE FROM tutorial_level_solutions WHERE player_id = {player_id} AND level_id = {level_id}");
    db_conn
        .conn
        .query_drop(delete_query)
        .expect("Query should be successful");
}

#[test]
fn update_score_for_tutorial_levels_with_worse_prev_solutions() {
    let mut db_conn = create_database_connection();
    let player_id = 9;
    let level_id = 1;

    let query = format!("SELECT number_of_steps FROM tutorial_level_solutions WHERE player_id = {player_id} AND level_id = {level_id}");

    let before: Option<i32> = db_conn
        .conn
        .query_first(query.clone())
        .expect("Query should be successful");
    assert!(
        before.is_some(),
        "row for player_id {player_id} and level_id {level_id} should exist initially"
    );

    update_score_for_tutorial_level(&mut db_conn, player_id, level_id, 5);

    let after: Option<i32> = db_conn
        .conn
        .query_first(query)
        .expect("Query should be successful");
    assert!(
        after.is_some(),
        "row for player_id {player_id} and level_id {level_id} should exist"
    );
    assert_ne!(
        before.unwrap(),
        after.unwrap(),
        "number of steps should be changed"
    );

    let reset_query = format!("UPDATE tutorial_level_solutions SET number_of_steps = 3 WHERE player_id = {player_id} AND level_id = {level_id}");
    db_conn
        .conn
        .query_drop(reset_query)
        .expect("Query should be successful");
}

#[test]
fn update_score_for_tutorial_level_with_better_prev_solution() {
    let mut db_conn = create_database_connection();
    let player_id = 9;
    let level_id = 1;

    let query = format!("SELECT number_of_steps FROM tutorial_level_solutions WHERE player_id = {player_id} AND level_id = {level_id}");

    let before: Option<i32> = db_conn
        .conn
        .query_first(query.clone())
        .expect("Query should be successful");
    assert!(
        before.is_some(),
        "row for player_id {player_id} and level_id {level_id} should exist initially"
    );

    update_score_for_tutorial_level(&mut db_conn, player_id, level_id, 2);

    let after: Option<i32> = db_conn
        .conn
        .query_first(query)
        .expect("Query should be successful");
    assert!(
        after.is_some(),
        "row for player_id {player_id} and level_id {level_id} should exist"
    );
    assert_eq!(
        before.unwrap(),
        after.unwrap(),
        "number of steps should not be changed"
    );
}

#[test]
fn get_random_challenge_fen_should_produce_two_different_consecutive_fens() {
    let mut db_conn = create_database_connection();
    let (_, first_fen) = get_random_challenge_fen(&mut db_conn);
    let (_, second_fen) = get_random_challenge_fen(&mut db_conn);
    assert_ne!(first_fen, second_fen, "two fens should not be same");
}

#[test]
fn make_fen_from_prefab_should_not_have_unfilled_positions() {
    let prefab = "11 10 ZZZZZZZZZZ/Z1_2+1_1Z/ZZ1ZZZZZZZ/Z1_1+2_1Z/ZZZZ1ZZZZZ/Z1_2+1_1Z/ZZZZZZ1ZZZ/Z1_1+2_1Z/ZZZZZZZZ1Z/Z1_2+1_1Z/ZZZZZZZZZZ 1";
    let result = make_fen_from_prefab(prefab.to_string());
    assert!(!result.contains('_'), "fen should be filled");
    assert!(!result.contains('+'), "fen should be filled");
}

#[test]
fn save_challenge_result_should_create_new_row() {
    let mut db_conn = create_database_connection();
    let player_id = 2;
    let fen = String::new();
    let num_of_steps = 5;
    let level_id = 1;
    let check_query = format!("SELECT id FROM challenge_solutions WHERE player_id = {player_id} AND prefab_id = {level_id}");
    let before: Option<i32> = db_conn
        .conn
        .query_first(check_query.clone())
        .expect("Query should be successful");
    assert!(before.is_none());

    save_challenge_result(&mut db_conn, player_id, fen, num_of_steps, level_id);

    let after: Option<i32> = db_conn
        .conn
        .query_first(check_query)
        .expect("Query should be successful");
    assert!(after.is_some());

    let delete_query = format!(
        "DELETE FROM challenge_solutions WHERE player_id = {player_id} AND prefab_id = {level_id}"
    );
    db_conn
        .conn
        .query_drop(delete_query)
        .expect("Query should be successful");
}

#[test]
fn create_new_player_should_add_new_row_in_db_and_add_to_config() {
    let mut db_conn = create_database_connection();
    let mut config = ConfigResource::default();

    let count_rows_query = "SELECT COUNT(id) FROM players";
    let before_in_db: i32 = db_conn
        .conn
        .query_first(count_rows_query)
        .expect("Query should be successful")
        .unwrap();
    let before_in_config = config.local_players.len();
    let before_in_config_ind = config.selected_player_id;
    create_new_player(&mut db_conn, &mut config);
    let after_in_db: i32 = db_conn
        .conn
        .query_first(count_rows_query)
        .expect("Query should be successful")
        .unwrap();
    let after_in_config = config.local_players.len();
    assert_eq!(
        before_in_db + 1,
        after_in_db,
        "number of rows should increase by 1"
    );
    assert_eq!(
        before_in_config + 1,
        after_in_config,
        "number of local players in config should increase by 1"
    );
    assert_eq!(
        after_in_config - 1,
        config.selected_player_id as usize,
        "newly created player should be selected inconfig"
    );

    let new_player = config.local_players[config.selected_player_id as usize]
        .name
        .clone();
    let reset_query = format!("DELETE FROM players WHERE player_name = \"{new_player}\"");
    db_conn
        .conn
        .query_drop(reset_query)
        .expect("Query should be successful");
    config.local_players.pop();
    config.selected_player_id = before_in_config_ind;
    update_cofig_file(&mut config);
}

#[test]
fn get_best_ten_challenge_scores_for_player_with_no_solutions_should_be_empty() {
    let mut db_conn = create_database_connection();
    let result = get_best_ten_challenge_scores_for_player(&mut db_conn, 2);
    assert!(result.is_empty());
}

#[test]
fn get_best_ten_challenge_scores_for_player_with_less_than_ten_solutions_should_have_all_solutions()
{
    let mut db_conn = create_database_connection();
    let result = get_best_ten_challenge_scores_for_player(&mut db_conn, 1);
    assert_eq!(5, result.len(), "player with id 1 has 5 solutions");
}

#[test]
fn get_best_ten_challenge_scores_for_player_with_more_than_ten_solutions_should_return_ten() {
    let mut db_conn = create_database_connection();
    let result = get_best_ten_challenge_scores_for_player(&mut db_conn, 9);
    assert_eq!(
        10,
        result.len(),
        "player with id 9 has more than 10 solutions"
    );
}

#[test]
fn get_challenge_fen_at_ind_should_return_filled_prefab() {
    let mut db_conn = create_database_connection();
    let prefab_ind = 1;
    let query = format!("SELECT level_name FROM challenge_prefabs WHERE id = {}", 2);
    let name_of_prefab: String = db_conn
        .conn
        .query_first(query)
        .expect("Query should be successful")
        .unwrap();
    let (result_id, result_fen, result_name) = get_challenge_fen_at_ind(&mut db_conn, prefab_ind);
    assert_eq!(2, result_id, "should choose prefab with id 2");
    assert_eq!(
        name_of_prefab, result_name,
        "should return nam eof chosen prefab"
    );
    assert!(!result_fen.contains('_'), "fen should be filled");
    assert!(!result_fen.contains('+'), "fen should be filled");
}

#[test]
fn get_next_challenge_fen_should_return_prefab_after_given_ne() {
    let mut db_conn = create_database_connection();
    let prefab_ind = 1;
    let query = format!("SELECT level_name FROM challenge_prefabs WHERE id = {}", 3);
    let name_of_prefab: String = db_conn
        .conn
        .query_first(query)
        .expect("Query should be successful")
        .unwrap();
    let (result_ind, result_id, result_fen, result_name) =
        get_next_challenge_fen(&mut db_conn, prefab_ind);
    assert_eq!(prefab_ind + 1, result_ind, "should return next ind");
    assert_eq!(3, result_id, "should choose prefab with id 3");
    assert_eq!(
        name_of_prefab, result_name,
        "should return nam eof chosen prefab"
    );
    assert!(!result_fen.contains('_'), "fen should be filled");
    assert!(!result_fen.contains('+'), "fen should be filled");
}

#[test]
fn get_prev_challenge_fen_should_return_prefab_before_given_one() {
    let mut db_conn = create_database_connection();
    let prefab_ind = 1;
    let query = format!("SELECT level_name FROM challenge_prefabs WHERE id = {}", 1);
    let name_of_prefab: String = db_conn
        .conn
        .query_first(query)
        .expect("Query should be successful")
        .unwrap();
    let (result_ind, result_id, result_fen, result_name) =
        get_prev_challenge_fen(&mut db_conn, prefab_ind);
    assert_eq!(prefab_ind - 1, result_ind, "should return previous ind");
    assert_eq!(1, result_id, "should choose prefab with id 1");
    assert_eq!(
        name_of_prefab, result_name,
        "should return nam eof chosen prefab"
    );
    assert!(!result_fen.contains('_'), "fen should be filled");
    assert!(!result_fen.contains('+'), "fen should be filled");
}

#[test]
fn save_multiplier_result_should_create_new_row() {
    let mut db_conn = create_database_connection();
    let player_id = 2;
    let fen = String::new();
    let num_of_steps = 5;
    let level_id = 1;
    let check_query = format!("SELECT id FROM multiplayer_solutions WHERE player_id = {player_id} AND prefab_id = {level_id}");
    let before: Option<i32> = db_conn
        .conn
        .query_first(check_query.clone())
        .expect("Query should be successful");
    assert!(before.is_none());

    save_multiplayer_result(&mut db_conn, player_id, fen, num_of_steps, level_id);

    let after: Option<i32> = db_conn
        .conn
        .query_first(check_query)
        .expect("Query should be successful");
    assert!(after.is_some());

    let delete_query = format!(
        "DELETE FROM multiplayer_solutions WHERE player_id = {player_id} AND prefab_id = {level_id}"
    );
    db_conn
        .conn
        .query_drop(delete_query)
        .expect("Query should be successful");
}

#[test]
fn get_best_ten_multiplayer_scores_for_player_with_no_solutions_should_be_empty() {
    let mut db_conn = create_database_connection();
    let result = get_best_ten_multiplayer_scores_for_player(&mut db_conn, 2);
    assert!(result.is_empty());
}

#[test]
fn get_best_ten_multiplayer_scores_for_player_with_less_than_ten_solutions_should_have_all_solutions(
) {
    let mut db_conn = create_database_connection();
    let result = get_best_ten_multiplayer_scores_for_player(&mut db_conn, 1);
    assert_eq!(5, result.len(), "player with id 1 has 5 solutions");
}

#[test]
fn get_best_ten_multiplayer_scores_for_player_with_more_than_ten_solutions_should_return_ten() {
    let mut db_conn = create_database_connection();
    let result = get_best_ten_multiplayer_scores_for_player(&mut db_conn, 9);
    assert_eq!(
        10,
        result.len(),
        "player with id 9 has more than 10 solutions"
    );
}

fn create_database_connection() -> DatabaseConnection {
    let opts = OptsBuilder::new()
        .user(Some("root"))
        .pass(Some("root1234"))
        .db_name(Some("tests"));
    let pool = Pool::new(opts).expect("Databse url is incorrect");
    let conn = pool.get_conn().expect("There must be a connection");
    DatabaseConnection { pool, conn }
}
