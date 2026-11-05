use std::sync::Arc;

use serenity::{
    Client,
    all::{Context, EventHandler, FullEvent, GatewayIntents, Token},
    async_trait,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn dispatch(&self, _: &Context, event: &FullEvent) {
        let FullEvent::Ready { data_about_bot, .. } = event else {
            return;
        };
        if let Some(shard_info) = data_about_bot.shard {
            println!(
                "{} shard {}/{} is connected",
                data_about_bot.user.name,
                shard_info.id,
                shard_info.total.get() - 1,
            );
        } else {
            println!("{} is connected", data_about_bot.user.name);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = Token::from_env("DISCORD_TOKEN").expect("DISCORD_TOKEN should be set");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Arc::new(Handler))
        .await
        .unwrap();

    client.start_autosharded().await.unwrap();
}
