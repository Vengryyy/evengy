use serenity::async_trait;
use serenity::model::channel::ReactionType;
use serenity::prelude::*;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

mod utils;
use utils::get_json_value;

use crate::commands::{neo::NEO_COMMAND, why::WHY_COMMAND, help::HELP_COMMAND};
use rand::{Rng, rngs::StdRng, SeedableRng};

mod commands {
    pub mod neo;
    pub mod why;
    pub mod help;
}

#[group]
#[commands(neo, why, help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().contains("dead") || msg.content.to_lowercase().contains("skull") || msg.content.to_lowercase().contains("skeleton") {
            let strings = ["💀", "☠️"];
            let mut rng = StdRng::from_entropy();
            let index = rng.gen_range(0..2);
            let reaction_type = ReactionType::Unicode(strings[index].to_string());
            if let Err(why) = msg.react(&ctx.http, reaction_type).await {
                println!("Error reacting to message: {:?}", why);
            }
        }
    }

    async fn resume(&self, _: Context, _: serenity::model::event::ResumedEvent) {
        println!("Resumed");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    let data = get_json_value("config.json").await.unwrap();
    let token = data["token"].as_str().unwrap();
    let prefix = data["prefix"].as_str().unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}