use std::fs::File;
use std::io::Read;

use sqlx::{cursor::Cursor, query, row::Row, SqlitePool};

pub async fn create_pool(path: &str) -> SqlitePool {
    SqlitePool::new(path).await.expect("Error connecting to db")
}

pub async fn create_tables(pool: SqlitePool) -> SqlitePool {
    let mut file = File::open("DATA/schema.sql").expect("Failed to open schema file.");
    let mut schema = String::new();
    file.read_to_string(&mut schema).unwrap();

    query(&schema)
        .execute(&pool)
        .await
        .expect("Failed to create tables");

    pool
}

pub async fn get_token(pool: SqlitePool) -> String {
    let mut cur = query("SELECT bot_token FROM settings;").fetch(&pool);
    match cur.next().await.expect("Failed to query the db for token") {
        Some(row) => row.get("bot_token"),
        None => "Bot Token".to_string(),
    }
}
