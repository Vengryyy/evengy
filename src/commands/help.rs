use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

use crate::utils::get_json_value;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let data = get_json_value("config.json").await.unwrap();
    let text = data["commands"]["help"]["text"].as_str().unwrap();
    let _for = data["commands"]["help"]["for"].as_u64().unwrap();
    let id = data["id"].as_u64().unwrap();

    if _for == 1 {
        if msg.author.id == id {
            msg.reply(&ctx.http, text).await?;
        }
    } else if _for == 2 {
        msg.reply(&ctx.http, text).await?;
    }

    Ok(())
}