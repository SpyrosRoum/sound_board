use std::fs::File;
use std::io::Read;

use sqlx::{cursor::Cursor, query, row::Row, Connect, SqliteConnection};

static PATH: &str = "sqlite://DATA/app.db";

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

pub async fn save(token: String) -> bool {
    let mut con = get_connection().await;
    match query("DELETE FROM settings; INSERT INTO settings (bot_token) VALUES (?);")
        .bind(token)
        .execute(&mut con)
        .await
    {
        Ok(_) => true,
        _ => false,
    }
}
