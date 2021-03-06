use crate::{
    buffers, discord,
    discord::DISCORD,
    ffi::{self, *},
    plugin_print,
};
use dirs;
use serenity::{
    model::{
        channel::Channel,
        id::{ChannelId, GuildId},
    },
    prelude::RwLock,
    CACHE,
};
use std::{fs, ptr, sync::Arc, thread, time::Duration};

// *DO NOT* touch this outside of init/end
static mut MAIN_COMMAND_HOOK: *mut HookCommand = ptr::null_mut();
static mut BUFFER_SWITCH_CB: *mut SignalHook = ptr::null_mut();
static mut QUERY_CMD_HOOK: *mut HookCommandRun = ptr::null_mut();
static mut NICK_CMD_HOOK: *mut HookCommandRun = ptr::null_mut();
static mut TIMER_HOOK: *mut TimerHook = ptr::null_mut();

pub fn init() -> Option<()> {
    let main_cmd_hook = ffi::hook_command(
        weechat_cmd::COMMAND,
        weechat_cmd::DESCRIPTION,
        weechat_cmd::ARGS,
        weechat_cmd::ARGDESC,
        weechat_cmd::COMPLETIONS,
        move |buffer, input| run_command(&buffer, input),
    )?;

    let query_hook = ffi::hook_command_run("/query", handle_query)?;
    let nick_hook = ffi::hook_command_run("/nick", handle_nick)?;
    let buffer_switch_hook = ffi::hook_signal("buffer_switch", handle_buffer_switch)?;
    // TODO: Dynamic timer delay like weeslack?
    let timer_hook = ffi::hook_timer(50, 0, 0, handle_timer)?;

    unsafe {
        MAIN_COMMAND_HOOK = Box::into_raw(Box::new(main_cmd_hook));
        BUFFER_SWITCH_CB = Box::into_raw(Box::new(buffer_switch_hook));
        TIMER_HOOK = Box::into_raw(Box::new(timer_hook));
        QUERY_CMD_HOOK = Box::into_raw(Box::new(query_hook));
        NICK_CMD_HOOK = Box::into_raw(Box::new(nick_hook));
    };
    Some(())
}

pub fn destroy() {
    unsafe {
        let _ = Box::from_raw(MAIN_COMMAND_HOOK);
        MAIN_COMMAND_HOOK = ptr::null_mut();
        let _ = Box::from_raw(BUFFER_SWITCH_CB);
        BUFFER_SWITCH_CB = ptr::null_mut();
        let _ = Box::from_raw(TIMER_HOOK);
        TIMER_HOOK = ptr::null_mut();
        let _ = Box::from_raw(QUERY_CMD_HOOK);
        QUERY_CMD_HOOK = ptr::null_mut();
        let _ = Box::from_raw(NICK_CMD_HOOK);
        NICK_CMD_HOOK = ptr::null_mut();
    };
}

#[allow(clippy::needless_pass_by_value)]
fn handle_buffer_switch(data: SignalHookData) {
    if let SignalHookData::Pointer(buffer) = data {
        thread::spawn(move || {
            buffers::load_history(&buffer);
            buffers::load_nicks(&buffer);
        });
    }
}

fn handle_timer(_remaining: i32) {
    while let Ok(_) = crate::synchronization::WEE_SYNC.try_recv() {
        let _ = crate::synchronization::WEE_SYNC.send();
    }
}

// TODO: Transform irc/weechat style to discord style
#[allow(clippy::needless_pass_by_value)]
pub fn buffer_input(buffer: Buffer, message: &str) {
    let channel = buffer
        .get("localvar_channelid")
        .and_then(|id| id.parse().ok())
        .map(ChannelId);

    let message = ffi::remove_color(message);

    if let Some(channel) = channel {
        channel
            .say(message)
            .unwrap_or_else(|_| panic!("Unable to send message to {}", channel.0));
    }
}

// TODO: Make this faster
// TODO: Handle command options
fn handle_query(_buffer: Buffer, command: &str) {
    let current_user = &CACHE.read().user;
    let substr = &command["/query ".len()..].trim();

    let mut found_members = Vec::new();
    for guild in current_user.guilds().expect("Unable to fetch guilds") {
        if let Some(guild) = guild.id.to_guild_cached() {
            let guild = guild.read().clone();
            for m in guild.members_containing(substr, false, true) {
                found_members.push(m.clone());
            }
        }
    }
    found_members.dedup_by_key(|mem| mem.user.read().id);
    if let Some(target) = found_members.get(0) {
        if let Ok(chan) = target.user.read().create_dm_channel() {
            buffers::create_buffer_from_dm(
                Channel::Private(Arc::new(RwLock::new(chan))),
                &current_user.name,
                true,
            );
        }
    }
}

// TODO: Handle command options
fn handle_nick(buffer: Buffer, command: &str) {
    let mut substr = command["/nick".len()..].trim().to_owned();
    let mut split = substr.split(" ");
    let all = split.next() == Some("-all");
    if all {
        substr = substr["-all".len()..].trim().to_owned();
    }
    let guilds = if all {
        let current_user = &CACHE.read().user;

        // TODO: Error handling
        current_user
            .guilds()
            .unwrap_or_default()
            .iter()
            .map(|g| g.id)
            .collect()
    } else {
        let guild = on_main! {{
            let guild = match buffer.get("localvar_guildid") {
                Some(guild) => guild,
                None => return,
            };
            match guild.parse::<u64>() {
                Ok(v) => GuildId(v),
                Err(_) => return,
            }
        }};
        vec![guild]
    };

    thread::spawn(move || {
        for guild in guilds {
            let new_nick = if substr.is_empty() {
                None
            } else {
                Some(substr.as_str())
            };
            let _ = guild.edit_nickname(new_nick);
            // Make it less spammy
            thread::sleep(Duration::from_secs(1));
        }

        on_main! {{
            buffers::update_nick();
        }};
    });
}

fn run_command(_buffer: &Buffer, command: &str) {
    // TODO: Add rename command
    // TODO: Get a proper parser
    match command {
        "" => plugin_print("see /help discord for more information"),
        "connect" => {
            match ffi::get_option("token") {
                Some(t) => {
                    if DISCORD.lock().is_none() {
                        discord::init(&t);
                    }
                }
                None => {
                    plugin_print("Error: plugins.var.weecord.token unset. Run:");
                    plugin_print("/discord token 123456789ABCDEF");
                    return;
                }
            };
        }
        "disconnect" => {
            let mut discord = DISCORD.lock();
            if discord.is_some() {
                if let Some(discord) = discord.take() {
                    discord.shutdown();
                };
            }
            plugin_print("Disconnected");
        }
        _ if command.starts_with("token ") => {
            let token = &command["token ".len()..];
            user_set_option("token", token.trim_matches('"'));
            plugin_print("Set Discord token");
        }
        "autostart" => {
            set_option("autostart", "true");
            plugin_print("Discord will now load on startup");
        }
        "noautostart" => {
            set_option("autostart", "false");
            plugin_print("Discord will not load on startup");
        }
        _ if command.starts_with("upload ") => {
            let mut file = command["upload ".len()..].to_owned();
            // TODO: Find a better way to expand paths
            if file.starts_with("~/") {
                let rest: String = file.chars().skip(2).collect();
                let dir = match dirs::home_dir() {
                    Some(dir) => dir.to_string_lossy().into_owned(),
                    None => ".".to_owned(),
                };
                file = format!("{}/{}", dir, rest);
            }
            let full = match fs::canonicalize(file) {
                Ok(f) => f.to_string_lossy().into_owned(),
                Err(e) => {
                    plugin_print(&format!("Unable to resolve file path: {}", e));
                    return;
                }
            };
            let full = full.as_str();
            // TODO: Check perms and file size
            let channel = on_main! {{
                let buffer = match Buffer::current() {
                    Some(buf) => buf,
                    None => return,
                };
                let channel = match buffer.get("localvar_channelid") {
                    Some(channel) => channel,
                    None => return,
                };
                match channel.parse::<u64>() {
                    Ok(v) => ChannelId(v),
                    Err(_) => return,
                }
            }};
            // TODO: Check result here
            let _ = channel.send_files(vec![full], |m| m);
        }
        _ => {
            plugin_print("Unknown command");
        }
    };
}

fn user_set_option(name: &str, value: &str) {
    plugin_print(&ffi::set_option(name, value));
}

mod weechat_cmd {
    pub const COMMAND: &str = "discord";
    pub const DESCRIPTION: &str = "\
Discord from the comfort of your favorite command-line IRC client!
Source code available at https://github.com/Noskcaj19/weechat-discord
Originally by https://github.com/khyperia/weechat-discord
Options used:
plugins.var.weecord.token = <discord_token>
plugins.var.weecord.rename.<id> = <string>
plugins.var.weecord.autostart = <bool>
";
    pub const ARGS: &str = "\
                     connect
                     disconnect
                     autostart
                     noautostart
                     token <token>
                     upload <file>";
    pub const ARGDESC: &'static str = "\
connect: sign in to discord and open chat buffers
disconnect: sign out of Discord
autostart: automatically sign into discord on start
noautostart: disable autostart
token: set Discord login token
upload: upload a file to the current channel
Example:
  /discord token 123456789ABCDEF
  /discord connect
  /discord autostart
  /discord disconnect
  /discord upload file.txt
";
    pub const COMPLETIONS: &str =
        "\
         connect || disconnect || token || autostart || noautostart || upload %(filename)";
}
