use std::io;

use serenity::async_trait;
use serenity::prelude::*;
//use serenity::model::{channel::Message, gateway::Ready};
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

use crate::commands::neo::NEO_COMMAND;

mod commands {
    pub mod neo;
}

#[group]
#[commands(neo)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // async fn message(&self, ctx: Context, msg: Message) {}
        

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "### "
        .group(&GENERAL_GROUP);

    println!("Please, place your discord account token here:");

    let mut token = String::new();

    match io::stdin().read_line(&mut token) {
        Ok(_) => {},
        Err(e) => println!("Error! {e}")
    }

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}