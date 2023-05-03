use std::{borrow::BorrowMut, fs};

use bevy::prelude::*;
use mysql::{prelude::Queryable, *};
use rand::Rng;
use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "./config.json";

#[derive(Clone, Debug)]
pub struct ChallengeScore {
    pub prefab_id: i32,
    pub level_name: String,
    pub num_of_steps: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct ConfigResource {
    pub languages: Vec<String>,
    pub selected_language: i32,
    pub selected_player_id: i32,
    pub local_players: Vec<Player>,
}

impl Default for ConfigResource {
    fn default() -> Self {
        let config_string =
            fs::read_to_string(FILE_PATH).expect("Should be able to read from file");
        let new_config: ConfigResource =
            serde_json::from_str(&config_string).expect("Config structure should be correct");
        new_config
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct FenPrefab {
    pub prefab_id: i32,
    pub fen: String,
    pub level_name: String,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct AllLevelsWithSolutions {
    pub level_id: i32,
    pub level_description: String,
    pub fen: String,
    pub number_of_steps: Option<i32>,
}

#[derive(Resource, Debug)]
pub struct DatabaseConnection {
    pub pool: Pool,
    pub conn: PooledConn,
}

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(connect_to_db)
            .init_resource::<ConfigResource>();
    }
}

fn connect_to_db(mut commands: Commands) {
    let url = "mysql://root:root1234@localhost:3306/thesis";
    let opts = Opts::from_url(url).expect("Databse url is incorrect");
    let pool = Pool::new(opts).expect("Databse url is incorrect");
    let conn = pool.get_conn().expect("There must be a connection");
    commands.insert_resource(DatabaseConnection { pool, conn });
}

pub fn get_all_levels_for_player(
    mut db_conn: ResMut<DatabaseConnection>,
    player_id: i32,
) -> Vec<AllLevelsWithSolutions> {
    let solved_levels_query = format!(
        r"SELECT tl.id, tl.descrip, tl.fen, tls.number_of_steps FROM tutorial_levels tl
        RIGHT JOIN tutorial_level_solutions tls ON tl.id = tls.level_id
        WHERE tls.player_id = {player_id}
        ORDER BY tl.id;"
    );

    let unsolved_levels_query = format!(
        r"SELECT DISTINCT tl.id, tl.descrip, tl.fen FROM tutorial_levels tl
        LEFT JOIN tutorial_level_solutions tls ON tl.id = tls.level_id
        WHERE tl.id NOT IN (SELECT tl.id FROM tutorial_levels tl
        RIGHT JOIN tutorial_level_solutions tls ON tl.id = tls.level_id
        WHERE tls.player_id = {player_id})
        ORDER BY tl.id;"
    );

    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");
    let mut all_levels = transaction
        .query_map(
            solved_levels_query,
            |(level_id, level_description, fen, number_of_steps)| AllLevelsWithSolutions {
                level_id,
                level_description,
                fen,
                number_of_steps,
            },
        )
        .expect("List of all levels and solutions for given player must be returned");
    let mut unsolved_levels = transaction
        .query_map(
            unsolved_levels_query,
            |(level_id, level_description, fen)| AllLevelsWithSolutions {
                level_id,
                level_description,
                fen,
                number_of_steps: None,
            },
        )
        .expect("List of all levels and solutions for given player must be returned");
    all_levels.append(&mut unsolved_levels);
    transaction
        .commit()
        .expect("Transaction for getting all levels must be commited");
    all_levels
}

pub fn update_score_for_tutorial_level(
    db_conn: &mut ResMut<DatabaseConnection>,
    player_id: i32,
    level_id: i32,
    steps: i32,
) {
    let check_query = format!(
        r"SELECT COUNT(tls.id) FROM tutorial_level_solutions tls
        WHERE tls.level_id = {level_id} AND player_id = {player_id};"
    );
    let get_score_query = format!(
        r"SELECT number_of_steps FROM tutorial_level_solutions tls
        WHERE tls.level_id = {level_id} AND player_id = {player_id};"
    );
    let update_query = format!(
        r"UPDATE tutorial_level_solutions
        SET number_of_steps = {steps}
        WHERE level_id = {level_id} AND player_id = {player_id};"
    );
    let insert_query = format!(
        r"INSERT INTO tutorial_level_solutions (player_id, level_id, number_of_steps)
        VALUES ({player_id},{level_id},{steps})"
    );

    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");
    let is_already_solved: Option<i32> = transaction
        .query_first(check_query)
        .expect("Query must be successful");
    if is_already_solved.is_none() {
        panic!("No result from query");
    }
    if is_already_solved.unwrap() == 0 {
        // Not previously solved
        transaction
            .query_drop(insert_query)
            .expect("Query must be successful");
    } else {
        // Previously solved
        let prev_result: Option<i32> = transaction
            .query_first(get_score_query)
            .expect("Previous score should be in databse");
        if prev_result.unwrap() > steps {
            transaction
                .query_drop(update_query)
                .expect("Query must be successful");
        }
    }
    transaction
        .commit()
        .expect("Transaction for getting all levels must be commited");
}

pub fn get_random_challenge_fen(db_conn: &mut ResMut<DatabaseConnection>) -> (i32, String) {
    let prefabs = db_conn
        .conn
        .query_map(
            "SELECT id, fen, level_name FROM challenge_prefabs",
            |(id, fen_prefab, level_name)| FenPrefab {
                prefab_id: id,
                fen: fen_prefab,
                level_name,
            },
        )
        .expect("Query must be successful");
    let mut rng = rand::thread_rng();
    let rand_prefab = rng.gen_range(0..prefabs.len());
    (
        prefabs.get(rand_prefab).unwrap().prefab_id,
        make_fen_from_prefab(prefabs.get(rand_prefab).unwrap().fen.clone()),
    )
}

fn make_fen_from_prefab(prefab: String) -> String {
    let mut random = rand::thread_rng();
    let mut pawn_probability = 0.4;
    let mut perl_probability = 0.5;

    let mut fen: String = String::new();
    let mut pawns_added: i32 = 0;
    let mut perl_counter: i32 = 0;
    let mut prefab_iter = prefab.split_whitespace();
    fen.push_str(prefab_iter.next().unwrap());
    fen.push(' ');
    fen.push_str(prefab_iter.next().unwrap());
    fen.push(' ');
    let string = prefab_iter.next().unwrap().chars();
    let number_of_pawns: i32 = prefab_iter.next().unwrap().parse().unwrap();
    let mut updated_string = String::new();
    for char in string {
        if char == '_' {
            if random.gen_bool(perl_probability) {
                updated_string.push('C');
                perl_probability -= 0.1;
                perl_counter += 1;
            } else {
                updated_string.push('1');
                perl_probability += 0.1;
            }
        } else if char == '+' {
            if pawns_added < number_of_pawns {
                if random.gen_bool(pawn_probability) {
                    if pawns_added == 0 {
                        updated_string.push('p');
                    } else {
                        updated_string.push('P');
                    }
                    pawns_added += 1;
                    pawn_probability -= 0.1;
                } else {
                    updated_string.push('1');
                    pawn_probability += 0.1;
                }
            } else {
                updated_string.push('1');
            }
        } else {
            updated_string.push(char);
        }
    }
    fen.push_str(updated_string.as_str());
    fen.push(' ');
    fen.push_str(perl_counter.to_string().as_str());
    fen
}

pub fn save_challenge_result(
    db_conn: &mut ResMut<DatabaseConnection>,
    player_id: i32,
    fen: String,
    num_of_steps: i32,
    level_id: i32,
) {
    let insert_query = format!(
        r"INSERT INTO challenge_solutions (fen, num_of_steps, player_id, prefab_id)
        VALUES ('{fen}', {num_of_steps}, {player_id}, {level_id});"
    );

    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");

    transaction
        .query_drop(insert_query)
        .expect("Query must be successful");

    transaction
        .commit()
        .expect("Transaction for getting all levels must be commited");
}

pub fn create_new_player(
    db_conn: &mut ResMut<DatabaseConnection>,
    config: &mut ResMut<ConfigResource>,
) {
    let all_players_query = "SELECT player_name FROM players;".to_string();
    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");

    let all_players: Vec<String> = transaction
        .query(all_players_query)
        .expect("The query should be successful");

    let mut rng = rand::thread_rng();
    let mut player = format!(
        "Unknown{}{}{}{}",
        rng.gen_range(0..9),
        rng.gen_range(0..9),
        rng.gen_range(0..9),
        rng.gen_range(0..9)
    );
    while all_players.contains(&player) {
        player = format!(
            "Unknown{}{}{}{}",
            rng.gen_range(0..9),
            rng.gen_range(0..9),
            rng.gen_range(0..9),
            rng.gen_range(0..9)
        );
    }
    transaction
        .query_drop(format!(
            r"INSERT INTO players (player_name)
        VALUES ('{player}');"
        ))
        .expect("This query should be successful");
    let new_player: Option<i32> = transaction
        .query_first(format!(
            r"SELECT id FROM players
        WHERE player_name = '{player}';"
        ))
        .expect("This query should be successful");
    let player_struct: Player = if let Some(player_id) = new_player {
        Player {
            id: player_id,
            name: player,
        }
    } else {
        panic!("Player id not received")
    };
    transaction
        .commit()
        .expect("The transaction should be committed");
    config.local_players.push(player_struct);
    config.selected_player_id = config.local_players.len() as i32 - 1;
    update_cofig_file(config.borrow_mut());
}

pub fn update_cofig_file(config: &mut ConfigResource) {
    let json_config = serde_json::to_string(&config).expect("Config should be serializable");
    fs::write(FILE_PATH, json_config).expect("File should be rewritten");
}

pub fn get_best_ten_challenge_scores_for_player(
    db_conn: &mut ResMut<DatabaseConnection>,
    player_id: i32,
) -> Vec<ChallengeScore> {
    let query = format!(
        r"SELECT cs.prefab_id, cp.level_name, cs.num_of_steps FROM challenge_solutions cs
        LEFT JOIN challenge_prefabs cp ON cs.prefab_id = cp.id
        WHERE cs.player_id = {player_id}
        ORDER BY cs.num_of_steps DESC;"
    );
    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");

    let all_player_scores = transaction
        .query_map(query, |(prefab_id, level_name, num_of_steps)| {
            ChallengeScore {
                prefab_id,
                level_name,
                num_of_steps,
            }
        })
        .expect("Query must be successful");

    transaction
        .commit()
        .expect("Transaction should be committed");
    if all_player_scores.len() >= 10 {
        all_player_scores[0..10].to_vec()
    } else {
        all_player_scores.to_vec()
    }
}

pub fn get_challenge_fen_at_ind(
    db_conn: &mut ResMut<DatabaseConnection>,
    ind: i32,
) -> (i32, String, String) {
    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");
    let prefabs = transaction
        .query_map(
            "SELECT id, fen, level_name FROM challenge_prefabs",
            |(id, fen_prefab, level_name)| FenPrefab {
                prefab_id: id,
                fen: fen_prefab,
                level_name,
            },
        )
        .expect("Query must be successful");
    transaction
        .commit()
        .expect("Transaction should be committed");
    (
        prefabs.get(ind as usize).unwrap().prefab_id,
        make_fen_from_prefab(prefabs.get(ind as usize).unwrap().fen.clone()),
        prefabs.get(ind as usize).unwrap().level_name.clone(),
    )
}

pub fn get_next_challenge_fen(
    db_conn: &mut ResMut<DatabaseConnection>,
    ind: i32,
) -> (i32, i32, String, String) {
    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");
    let prefabs = transaction
        .query_map(
            "SELECT id, fen, level_name FROM challenge_prefabs",
            |(id, fen_prefab, level_name)| FenPrefab {
                prefab_id: id,
                fen: fen_prefab,
                level_name,
            },
        )
        .expect("Query must be successful");
    transaction
        .commit()
        .expect("Transaction should be committed");
    let next_ind = if ind + 1 >= prefabs.len() as i32 {
        0
    } else {
        ind + 1
    };
    (
        next_ind,
        prefabs.get(ind as usize).unwrap().prefab_id,
        make_fen_from_prefab(prefabs.get(ind as usize).unwrap().fen.clone()),
        prefabs.get(ind as usize).unwrap().level_name.clone(),
    )
}

pub fn get_prev_challenge_fen(
    db_conn: &mut ResMut<DatabaseConnection>,
    ind: i32,
) -> (i32, i32, String, String) {
    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");
    let prefabs = transaction
        .query_map(
            "SELECT id, fen, level_name FROM challenge_prefabs",
            |(id, fen_prefab, level_name)| FenPrefab {
                prefab_id: id,
                fen: fen_prefab,
                level_name,
            },
        )
        .expect("Query must be successful");
    transaction
        .commit()
        .expect("Transaction should be committed");
    let prev_ind = if ind == 0 {
        prefabs.len() as i32 - 1
    } else {
        ind - 1
    };
    (
        prev_ind,
        prefabs.get(ind as usize).unwrap().prefab_id,
        make_fen_from_prefab(prefabs.get(ind as usize).unwrap().fen.clone()),
        prefabs.get(ind as usize).unwrap().level_name.clone(),
    )
}

pub fn save_multiplayer_result(
    db_conn: &mut ResMut<DatabaseConnection>,
    player_id: i32,
    fen: String,
    num_of_steps: i32,
    level_id: i32,
) {
    let insert_query = format!(
        r"INSERT INTO multiplayer_solutions (fen, num_of_steps, player_id, prefab_id)
        VALUES ('{fen}', {num_of_steps}, {player_id}, {level_id});"
    );

    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");

    transaction
        .query_drop(insert_query)
        .expect("Query must be successful");

    transaction
        .commit()
        .expect("Transaction for getting all levels must be commited");
}
