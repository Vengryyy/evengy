use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, ReactionType};
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

#[command]
async fn why(ctx: &Context, msg: &Message) -> CommandResult {
    let emojis = ["🇧", "🇪", "🇨", "🇦", "🇺", "🇸", "3️⃣"];
    for emoji in emojis {
        let reaction_type = ReactionType::Unicode(emoji.to_string());
        if let Err(why) = msg.react(&ctx.http, reaction_type).await {
            println!("Error reacting to message: {:?}", why);
        }
    }
    Ok(())
}