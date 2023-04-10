use bevy::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
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
