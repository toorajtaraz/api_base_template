use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn run_migrations(db_url: &str) {
    embed_migrations!();
    let connection = PgConnection::establish(db_url).expect("Error connecting to database");
    match embedded_migrations::run_with_output(&connection, &mut std::io::stdout()) {
        Ok(_) => {
            println!("Migration executed successfully!");
        }
        _ => {
            eprintln!("Migration execution failed!");
        }
    }
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    match Pool::builder().build(manager) {
        Ok(res) => {
            println!("Stablished connection to DB!");
            return res;
        }
        _ => {
            panic!("Could not connect to database!");
        }
    }
}
