use std::fs::File;
use std::io::Read;

use sqlx;
use sqlx::cursor::Cursor;
use sqlx::row::Row;
use sqlx::SqlitePool;

pub async fn create_pool() -> SqlitePool {
    SqlitePool::new("sqlite://DATA/app.db")
        .await
        .expect("Failed to connect to db")
}

pub async fn create_tables(pool: &SqlitePool) {
    let mut file = File::open("DATA/schema.sql").expect("Failed to open schema file.");
    let mut schema = String::new();
    file.read_to_string(&mut schema).unwrap();

    sqlx::query(&schema)
        .execute(pool)
        .await
        .expect("Failed to create tables");
}

pub async fn get_token(pool: &SqlitePool) -> Option<String> {
    let mut cur = sqlx::query("SELECT bot_token FROM settings;").fetch(pool);
    match cur.next().await.expect("Failed to query the db for token") {
        Some(row) => row.get("bot_token"),
        None => None,
    }
}
