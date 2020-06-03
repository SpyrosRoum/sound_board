use super::word::Word;

use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use rodio;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler; // For handling event.
struct DevSink; // for rodio sink.
struct KeyWords; // The keywords to look for.
struct MsgLbl; // For messages to the user.

impl TypeMapKey for MsgLbl {
    type Value = Arc<Mutex<String>>;
}

impl TypeMapKey for KeyWords {
    type Value = Arc<Mutex<Vec<Word>>>;
}

impl TypeMapKey for DevSink {
    type Value = rodio::Sink;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;

        let sink = data.get::<DevSink>().unwrap();

        let words_arc = data.get::<KeyWords>().unwrap();
        let words = words_arc.lock().unwrap();

        let mut text = msg.content.to_lowercase();
        for embed in msg.embeds {
            if let Some(d) = embed.description {
                text += format!("{}\n", &d.to_lowercase()).as_ref();
            }
            if let Some(f) = embed.footer {
                text += format!("{}\n", f.text.to_lowercase()).as_ref();
            }
            if let Some(t) = embed.title {
                text += format!("{}\n", &t.to_lowercase()).as_ref();
            }
            for field in embed.fields {
                text += format!("{}\n{}\n", field.name, field.value).as_ref();
            }
        }
        println!("{}", text);
        for word in &*words {
            if word.chn_id != msg.channel_id.to_string() {
                continue;
            }
            if text.contains(&word.word) {
                let mut lbl = data.get::<MsgLbl>().unwrap().lock().unwrap();

                *lbl = format!("Found \"{}\".", word.word);

                play_sound(&sink, &word.path);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let data = ctx.data.read().await;
        let mut lbl = data.get::<MsgLbl>().unwrap().lock().unwrap();

        *lbl = format!("Connected to discord as {}", ready.user.name);
        println!("{} is connected!", ready.user.name);
    }
}

fn play_sound(sink: &rodio::Sink, path: &str) {
    let file = File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
}

pub async fn start(token: String, words: Arc<Mutex<Vec<Word>>>, msg: Arc<Mutex<String>>) {
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating discord client");

    let device = rodio::default_output_device().unwrap();
    {
        let sink = rodio::Sink::new(&device);
        let mut data = client.data.write().await;
        data.insert::<DevSink>(sink);
        data.insert::<KeyWords>(words);
        data.insert::<MsgLbl>(msg)
    }

    if let Err(why) = client.start().await {
        let data = client.data.read().await;
        let mut lbl = data.get::<MsgLbl>().unwrap().lock().unwrap();
        *lbl = "Error starting the bot.".to_string();
        println!("Client error: {:?}", why);
    };
}
