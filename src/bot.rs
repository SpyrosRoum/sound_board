use super::word::Word;

use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex, RwLock};

use rodio;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use sqlx::{cursor::Cursor, query, row::Row, SqlitePool};
use tokio;

struct Handler; // For handling event.
struct ConnectionPool; // The connection to the database, because having multiple connections is a bad idea.
struct DevSink; // for rodio sink.
struct ReadEntries; // To know if there has been a change to the keywords.
struct KeyWords; // The keywords to look for.

impl TypeMapKey for ConnectionPool {
    // RwLock (aka Read Write Lock) makes the data only modify-able by 1 thread at a time
    // So you can only have the lock open with write a single use at a time.
    // You can have multiple reads, but you can't read as soon as the lock is opened for writing.
    // type Value = Arc<RwLock<PgPool>>;
    type Value = SqlitePool;
}

impl TypeMapKey for ReadEntries {
    type Value = bool;
}

impl TypeMapKey for KeyWords {
    type Value = Vec<Word>;
}

impl TypeMapKey for DevSink {
    type Value = rodio::Sink;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;

        if *data.get::<ReadEntries>().unwrap() {
            get_words(&ctx.data).await;
        }

        let words = data.get::<KeyWords>().unwrap();

        for word in words {
            println!("{:?}", word);
        }

        // for word in &msg.content.to_lowercase().split(' ') {
        //     let mut cur = query("SELECT * FROM words WHERE chn_id = ?")
        //         .bind(&msg.channel_id.as_u64().to_string())
        //         .fetch(pool);
        //
        //     while let Some(row) = cur.next().await.expect("Failed to query the db for words") {
        //         let sink = data.get::<DevSink>().unwrap();
        //         let path: String = row.get("file_path");
        //
        //     }
        // }
    }
    // play_sound(&sink, &path);

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn play_sound(sink: &rodio::Sink, path: &str) {
    let file = File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
}

async fn get_words(data: &Arc<tokio::sync::RwLock<TypeMap>>) -> Vec<Word> {
    let mut words = vec![];
    {
        let data = data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let mut cur = query("SELECT * FROM words").fetch(pool);

        while let Some(row) = cur.next().await.expect("Failed to query the db for words") {
            let path: String = row.get("file_path");
            let chn_id = row.get("chn_id");
            let word: String = row.get("word");

            words.push(Word { word, chn_id, path })
        }
    }

    {
        let mut data = data.write().await;
        let mut flag = data.get_mut::<ReadEntries>().unwrap();
        *flag = false;
    }

    words
}

pub async fn start(token: String, pool: Arc<Mutex<SqlitePool>>) {
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating discord client");

    let device = rodio::default_output_device().unwrap();
    {
        let sink = rodio::Sink::new(&device);
        let mut data = client.data.write().await;
        data.insert::<ReadEntries>(false);
        data.insert::<ConnectionPool>(pool.lock().expect("Failed to get lock for pool").clone());
        data.insert::<DevSink>(sink);
    }
    {
        let words = get_words(&client.data).await;
        let mut data = client.data.write().await;
        data.insert::<KeyWords>(words);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    };
}
