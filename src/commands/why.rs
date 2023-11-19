use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, ReactionType};
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

use crate::utils::get_json_value;

#[command]
async fn why(ctx: &Context, msg: &Message) -> CommandResult {
    let emojis = ["🇧", "🇪", "🇨", "🇦", "🇺", "🇸", "3️⃣"];

    let data = get_json_value("config.json").await.unwrap();
    let _for = data["commands"]["why"]["for"].as_u64().unwrap();
    let id = data["id"].as_u64().unwrap();

    if _for == 1 {
        if msg.author.id == id {
            for emoji in emojis {
                let reaction_type = ReactionType::Unicode(emoji.to_string());
                if let Err(why) = msg.react(&ctx.http, reaction_type).await {
                    println!("Error reacting to message: {:?}", why);
                }
            }
        }
    } else if _for == 2 {
        for emoji in emojis {
            let reaction_type = ReactionType::Unicode(emoji.to_string());
            if let Err(why) = msg.react(&ctx.http, reaction_type).await {
                println!("Error reacting to message: {:?}", why);
            }
        }
    }
    
    Ok(())
}