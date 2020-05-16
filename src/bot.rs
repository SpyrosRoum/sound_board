use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use rodio;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use sqlx::{cursor::Cursor, query, row::Row, SqlitePool};

struct Handler; // For handling event.
struct ConnectionPool; // The connection to the database, because having multiple connections is a bad idea.
struct DevSink; // for rodio sink.

impl TypeMapKey for ConnectionPool {
    // RwLock (aka Read Write Lock) makes the data only modify-able by 1 thread at a time
    // So you can only have the lock open with write a single use at a time.
    // You can have multiple reads, but you can't read as soon as the lock is opened for writing.
    // type Value = Arc<RwLock<PgPool>>;
    type Value = SqlitePool;
}

impl TypeMapKey for DevSink {
    type Value = rodio::Sink;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();

        let mut cur = query("SELECT * FROM words WHERE chn_id = ? AND word = ?")
            .bind(&msg.channel_id.as_u64().to_string())
            .bind(&msg.content.to_lowercase())
            .fetch(pool);

        match cur.next().await.expect("Failed to query the db for token") {
            Some(row) => {
                let sink = data.get::<DevSink>().unwrap();
                let path: String = row.get("file_path");
                play_sound(&sink, &path);
            }
            None => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn play_sound(sink: &rodio::Sink, path: &str) {
    let file = File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
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
        data.insert::<ConnectionPool>(pool.lock().expect("Failed to get lock for pool").clone());
        data.insert::<DevSink>(sink);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    };
}
