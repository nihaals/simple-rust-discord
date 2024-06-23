use std::env;

use serenity::all::{Context, EventHandler, GatewayIntents};
use serenity::model::gateway::Ready;
use serenity::{async_trait, Client};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        if let Some(shard_info) = ready.shard {
            println!("{} shard {}/{} is connected", ready.user.name, shard_info.id, shard_info.total - 1);
        } else {
            println!("{} is connected", ready.user.name);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN should be set");
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .unwrap();

    client.start_autosharded().await.unwrap();
}
