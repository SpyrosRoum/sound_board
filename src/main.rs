use std::fs::File;

use sqlx;
use sqlx::SqlitePool;

use async_std::task;
use std::io::Read;

async fn create_sqlite_pool() -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::new("sqlite://DATA/app.db").await
}

async fn create_db_tables(pool: &SqlitePool) {
    let mut file = File::open("DATA/schema.sql").expect("Failed to open schema file.");
    let mut schema = String::new();
    file.read_to_string(&mut schema).unwrap();

    sqlx::query(&schema)
        .execute(pool)
        .await
        .expect("Failed to create tables");
}

fn main() -> Result<(), sqlx::Error> {
    let pool = task::block_on(create_sqlite_pool())?;
    task::block_on(create_db_tables(&pool));

    println!("cool");
    Ok(())
}
