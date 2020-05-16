use std::sync::{Arc, Mutex};

use super::entry::Entry;
use super::schema::SCHEMA;

use sqlx::{cursor::Cursor, query, row::Row, Connect, SqliteConnection, SqlitePool};

static PATH: &str = "sqlite://DATA/app.db";

pub async fn get_pool() -> SqlitePool {
    SqlitePool::new(PATH)
        .await
        .expect("Failed to create sqlite pool")
}

pub async fn create_tables() {
    let mut con = SqliteConnection::connect(PATH)
        .await
        .expect("Failed to create connection to db");

    query(&SCHEMA)
        .execute(&mut con)
        .await
        .expect("Failed to create tables");
}

pub async fn get_token(pool: Arc<Mutex<SqlitePool>>) -> String {
    let pool = pool.lock().unwrap().clone();

    let mut cur = query("SELECT bot_token FROM settings;").fetch(&pool);
    match cur.next().await.expect("Failed to query the db for token") {
        Some(row) => row.get("bot_token"),
        None => "Bot Token".to_string(),
    }
}

pub async fn get_entries(pool: Arc<Mutex<SqlitePool>>) -> Vec<Entry> {
    let pool = pool.lock().unwrap().clone();
    let mut entries = vec![];

    let mut cur = query("SELECT * FROM words;").fetch(&pool);
    while let Some(e) = cur.next().await.expect("Failed to read entries cursor") {
        let i = entries.len();
        let mut entry = Entry::new_idle(i);

        entry.word = e.get("word");
        entry.chn_id = e.get("chn_id");
        entry.path = e.get("file_path");

        entries.push(entry);
    }

    entries
}

pub async fn save(pool: Arc<Mutex<SqlitePool>>, token: String, entries: Vec<Entry>) {
    let pool = pool.lock().unwrap().clone();

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
        query("INSERT INTO words (chn_id, word, file_path) VALUES (?, ?, ?)")
            .bind(&entry.chn_id)
            .bind(&entry.word)
            .bind(&entry.path)
            .execute(&pool)
            .await
            .expect("Failed to insert new entries");
    }
}
