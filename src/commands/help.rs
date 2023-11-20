use std::collections::HashMap;

use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

use crate::utils::get_json_value;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let data = get_json_value("config.json").await.unwrap();
    let style = data["commands"]["help"]["style"].as_str().unwrap();
    let _for = data["commands"]["help"]["for"].as_u64().unwrap();
    let id = data["id"].as_u64().unwrap();
    let prefix = data["prefix"].as_str().unwrap();

    let mut do_reply = false;

    if _for == 1 {
        if msg.author.id == id {
            do_reply = true;
        }
    } else if _for == 2 {
        do_reply = true;
    }

    if do_reply {
        let command_hash = HashMap::from([
            ("help", "list of commands"),
            ("neo", "neofetch"),
            ("why", "why? because!")
        ]);

        if style == "default" {
            let mut _answer = String::from("```\nList of commands:\n");

            for cmd in command_hash {
                let cmd_name = cmd.0;
                let cmd_desc = cmd.1;

                _answer += &String::from(format!(" > {} - {}\n", cmd_name, cmd_desc));
            }

            _answer += &String::from(format!("\nPrefix for commands: {}\n```", prefix));

            msg.reply(&ctx.http, _answer).await?;
        } else if style == "embed" {
            let embed = CreateEmbed::new()
                .title("List of commands")
                .description(format!("Prefix for commands: {}", prefix));

            let mut fields = vec![];
            
            for cmd in command_hash {
                let cmd_name = cmd.0;
                let cmd_desc = cmd.1;

                fields.push((cmd_name, cmd_desc, true));
            }

            embed.fields(fields);

            let builder = CreateMessage::new().embed(embed);

            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
        }
    }

    Ok(())
}