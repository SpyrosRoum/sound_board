use std::sync::{Arc, Mutex};

use super::entry::Entry;
use super::schema::SCHEMA;
use super::word::Word;

use sqlx::{cursor::Cursor, query, row::Row, Connect, SqliteConnection, SqlitePool};

static PATH: &str = "sqlite://app.db";

pub async fn get_pool() -> SqlitePool {
    SqlitePool::new(PATH)
        .await
        .expect("Failed to create sqlite pool")
}

pub async fn create_tables(pool: &SqlitePool) {
    query(&SCHEMA)
        .execute(pool)
        .await
        .expect("Failed to create tables");
}

pub async fn get_token() -> String {
    let mut con = SqliteConnection::connect(PATH)
        .await
        .expect("Failed to create connection to db");

    let mut cur = query("SELECT bot_token FROM settings;").fetch(&mut con);
    match cur.next().await.expect("Failed to query the db for token") {
        Some(row) => row.get("bot_token"),
        None => "Bot Token".to_string(),
    }
}

pub async fn get_words(pool: &SqlitePool) -> Vec<Word> {
    let mut words = vec![];

    let mut cur = query("SELECT * FROM words;").fetch(pool);
    while let Some(row) = cur.next().await.expect("Failed to read words cursor") {
        let mut word = Word::default();

        word.word = row.get("word");
        word.chn_id = row.get("chn_id");
        word.path = row.get("file_path");

        words.push(word)
    }

    words
}

pub async fn get_new_words(pool: Arc<Mutex<SqlitePool>>) -> Vec<Word> {
    let pool = pool.lock().unwrap().clone();
    let mut words = vec![];

    let mut cur = query("SELECT * FROM words;").fetch(&pool);
    while let Some(row) = cur.next().await.expect("Failed to read words cursor") {
        let mut word = Word::default();

        word.word = row.get("word");
        word.chn_id = row.get("chn_id");
        word.path = row.get("file_path");

        words.push(word)
    }

    words
}

pub async fn get_entries(pool: Arc<Mutex<SqlitePool>>) -> Vec<Entry> {
    let pool = pool.lock().unwrap().clone();
    let mut entries = vec![];

    let mut cur = query("SELECT * FROM words;").fetch(&pool);
    while let Some(e) = cur.next().await.expect("Failed to read entries cursor") {
        let i = entries.len();
        let mut entry = Entry::new_idle(i);

        entry.word.word = e.get("word");
        entry.word.chn_id = e.get("chn_id");
        entry.word.path = e.get("file_path");

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
            .bind(&entry.word.chn_id)
            .bind(&entry.word.word)
            .bind(&entry.word.path)
            .execute(&pool)
            .await
            .expect("Failed to insert new entries");
    }
}
