use bevy::prelude::*;
use mysql::{prelude::Queryable, *};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct FenPrefab {
    pub prefab_id: i32,
    pub fen: String
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct AllLevels {
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
        app.add_startup_system(connect_to_db);
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
) -> Vec<AllLevels> {
    let query = format!(
        r"SELECT tl.id, tl.descrip, tl.fen, tls.number_of_steps FROM tutorial_levels tl
        LEFT JOIN tutorial_level_solutions tls ON tl.id = tls.level_id
        WHERE tls.player_id = {player_id} OR tls.player_id IS NULL
        ORDER BY tl.id;"
    );

    let mut transaction = db_conn
        .conn
        .start_transaction(TxOpts::default())
        .expect("New transaction must be started");
    let all_levels = transaction
        .query_map(
            query,
            |(level_id, level_description, fen, number_of_steps)| AllLevels {
                level_id,
                level_description,
                fen,
                number_of_steps,
            },
        )
        .expect("List of all levels and solutions for given player must be returned");
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

pub fn get_challenge_fen(db_conn: &mut ResMut<DatabaseConnection>) -> String{
    let prefabs = db_conn.conn.query_map(
        "SELECT id, fen FROM challenge_prefabs",
        |(id, fen_prefab)| {
            FenPrefab { prefab_id: id, fen: fen_prefab }
        }
    ).expect("Query must be successful");
    let mut rng = rand::thread_rng();
    let rand_prefab = rng.gen_range(0..prefabs.len());
    make_fen_from_prefab(prefabs.get(rand_prefab).unwrap().fen.clone())
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

pub fn save_challenge_result(db_conn: &mut ResMut<DatabaseConnection>, player_id: i32, fen: String, num_of_steps: i32) {
    let insert_query = format!(
        r"INSERT INTO challenge_solutions (fen, num_of_steps, player_id)
        VALUES ('{fen}', {num_of_steps}, {player_id});"
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