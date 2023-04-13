use bevy::prelude::*;
use mysql::{prelude::Queryable, *};

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
    return all_levels;
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
        transaction
            .query_drop(update_query)
            .expect("Query must be successful");
    }
    transaction
        .commit()
        .expect("Transaction for getting all levels must be commited");
}
