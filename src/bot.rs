use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

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

pub async fn start(token: String) {
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating discord client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    };
}
