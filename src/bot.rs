use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use tokio::runtime::Runtime;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        println!(
            "{} said {} in {}",
            msg.author.name,
            &msg.content.to_lowercase(),
            msg.channel_id.as_u64(),
        );
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn start(token: String) {
    let mut rt = Runtime::new().expect("Failed to create runtime");
    rt.block_on(actual_start(token))
}

async fn actual_start(token: String) {
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating discord client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    };
}
