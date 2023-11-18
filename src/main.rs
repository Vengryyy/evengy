use std::io;

use serenity::async_trait;
use serenity::model::channel::ReactionType;
use serenity::prelude::*;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

use crate::commands::{neo::NEO_COMMAND, why::WHY_COMMAND};
use rand::{Rng, rngs::StdRng, SeedableRng};

use std::fs::File;
use std::io::{BufRead, BufReader};

mod commands {
    pub mod neo;
    pub mod why;
}

#[group]
#[commands(neo, why)]
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

async fn read_token() -> io::Result<String> {
    let file = File::open("token")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}


#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "### "
        .group(&GENERAL_GROUP);

    // println!("Please, place your discord account token here:");

    // let mut token = String::new();

    let token = match read_token().await {
        Ok(token) => token,
        Err(e) => {
            println!("An error occurred while reading the token: {}", e);
            return;
        },
    };

    // match io::stdin().read_line(&mut token) {
    //     Ok(_) => {},
    //     Err(e) => println!("Error! {e}")
    // }

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