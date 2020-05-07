use std::fs::File;
use std::io::Read;

use sqlx;
use sqlx::SqlitePool;

pub async fn create_sqlite_pool() -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::new("sqlite://DATA/app.db").await
}

pub async fn create_db_tables(pool: &SqlitePool) {
    let mut file = File::open("DATA/schema.sql").expect("Failed to open schema file.");
    let mut schema = String::new();
    file.read_to_string(&mut schema).unwrap();

    sqlx::query(&schema)
        .execute(pool)
        .await
        .expect("Failed to create tables");
}
