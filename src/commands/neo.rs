use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;
use sysinfo::{ProcessExt, SystemExt, ProcessorExt};
use uptime_lib;
use num_cpus;
use whoami;

#[command]
async fn neo(ctx: &Context, msg: &Message) -> CommandResult {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let num_of_cpus = num_cpus::get_physical();
    let cpu_name = sys.processors()[0].brand();
    let cpu_usage = format!("{}/{}", (sys.global_processor_info().cpu_usage().round() as usize) * num_of_cpus, 100 * num_of_cpus);
    let total_memory = sys.total_memory() / 1024;
    let used_memory = sys.used_memory() / 1024;
    let tmg = total_memory / 1024;
    let umg = used_memory / 1024;
    let user_name = whoami::username();
    let up_time;
    

    match uptime_lib::get() {
        Ok(uptime) => {
            let total_seconds = uptime.as_secs();
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            up_time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    }

    let mut processes = sys.processes().values().collect::<Vec<_>>();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
    let high_process = format!("{} - {} MB", processes[0].name(), processes[0].memory() / 1024);

    let info_text = format!("```ansi
[34;49m┍-┑┍--┑[0m [35mCPU    : [37m{} ({}%)
[34;49m┕e┙┕ve┙[0m [35mRAM    : [37m{}/{} MB ({}/{} GB) {}
[34;49m┍ng┑┍y┑[0m [35mUser   : [37m{}
[34;49m┕--┙┕-┙[0m [35mUpTime : [37m{}```", cpu_name, cpu_usage, used_memory, total_memory, umg, tmg, high_process, user_name, up_time);

    msg.channel_id.say(&ctx.http, info_text).await?;
    msg.delete(&ctx.http).await?;

    Ok(())
}