use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub fn init_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Limit pool to a single connection
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.")
}

// sample use case
// let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
// let pool = init_pool(&database_url);

// // Create both structs sharing the same pool.
// let ask_db = AskDatabase {
//     pool: pool.clone(),
//     private_store: PrivateInputStore::new(),
// };

// let diesel_gen_store = DieselGeneratorStore {
//     pool: pool.clone(),
// };
