use super::entry::Entry;

use std::fs::File;
use std::io::Read;

use sqlx::{cursor::Cursor, query, row::Row, Connect, SqliteConnection, SqlitePool};

static PATH: &str = "sqlite://DATA/app.db";

async fn get_pool() -> SqlitePool {
    SqlitePool::new(PATH)
        .await
        .expect("Failed to create sqlite pool")
}

async fn get_connection() -> SqliteConnection {
    SqliteConnection::connect(PATH)
        .await
        .expect("Failed to get connection")
}

pub async fn create_tables() {
    let mut con = get_connection().await;

    let mut file = File::open("DATA/schema.sql").expect("Failed to open schema file.");
    let mut schema = String::new();
    file.read_to_string(&mut schema).unwrap();

    query(&schema)
        .execute(&mut con)
        .await
        .expect("Failed to create tables");
}

pub async fn get_token() -> String {
    let mut con = get_connection().await;

    let mut cur = query("SELECT bot_token FROM settings;").fetch(&mut con);
    match cur.next().await.expect("Failed to query the db for token") {
        Some(row) => row.get("bot_token"),
        None => "Bot Token".to_string(),
    }
}

pub async fn save(token: String, entries: Vec<Entry>) {
    let pool = get_pool().await;

    query("DELETE FROM settings; INSERT INTO settings (bot_token) VALUES (?);")
        .bind(token)
        .execute(&pool)
        .await
        .expect("Failed to delete and insert token");

    query("DELETE FROM words;")
        .execute(&pool)
        .await
        .expect("Failed to delete old words.");

    for entry in entries.iter() {
        query("INSERT INTO words (g_id, chn_id, word, file_path) VALUES (?, ?, ?, ?)")
            .bind(&entry.g_id)
            .bind(&entry.chn_id)
            .bind(&entry.word)
            .bind(&entry.path)
            .execute(&pool)
            .await
            .expect("Failed to insert new entries.");
    }
}
