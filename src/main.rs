mod commands;
use std::env;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
//
use serenity::framework::standard::macros::group;

use crate::commands::ping::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group("pingpong")]
#[commands(ping)]
struct General;

#[tokio::main]

async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let token = env::var("BOT_TOKEN").expect("Expected a token");
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .intents(intents)
        .framework(framework)
        .await
        .expect("Err creating a client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
