#![feature(const_slice_len)]
extern crate libc;
#[macro_use]
extern crate weechat_lib;
use weechat_lib::ffi;

weechat_plugin!(
    name: b"weecord\0",
    author: b"khyperia <khyperia@live.com>\0",
    description: b"Discord support for weechat\0",
    version: b"0.1\0",
    license: b"MIT\0",
);

weechat_plugin_init!(plugin, {
    plugin.print("Foo");
    weechat_lib::MAIN_BUFFER.print("Bar");

    let cmd = weechat_discord_cmd::COMMAND;
    let desc = weechat_discord_cmd::DESCRIPTION;
    let args = weechat_discord_cmd::ARGS;
    let argdesc = weechat_discord_cmd::ARGDESC;
    let compl = weechat_discord_cmd::COMPLETIONS;

    return true;
});

weechat_plugin_end!({ return true });

mod weechat_discord_cmd {
    pub const COMMAND: &'static str = "discord";
    pub const DESCRIPTION: &'static str = "\
Discord from the comfort of your favorite command-line IRC client!
Source code available at https://github.com/Noskcaj19/weechat-discord
Originally by https://github.com/khyperia/weechat-discord
How does channel muting work?
If plugins.var.weecord.mute.<channel_id> is set to the literal \"1\", \
then that buffer will not be opened. When a Discord channel is muted \
(in the official client), weechat-discord detects this and automatically \
sets this setting for you. If you would like to override this behavior \
and un-mute the channel, set the setting to \"0\". (Do not unset it, as it \
will just get automatically filled in again)
Options used:
plugins.var.weecord.token = <discord_token>
plugins.var.weecord.rename.<id> = <string>
plugins.var.weecord.mute.<channel_id> = (0|1)
plugins.var.weecord.on_delete.<server_id> = <channel_id>
";
    pub const ARGS: &'static str = "\
                     connect
                     disconnect
                     token <token>";
    pub const ARGDESC: &'static str = "\
connect: sign in to discord and open chat buffers
disconnect: sign out of Discord
token: set Discord login token
query: open PM buffer with user
Example:
  /discord token 123456789ABCDEF
  /discord connect
  /discord query khyperia
  /discord disconnect
";
    pub const COMPLETIONS: &'static str =
        "\
         connect || disconnect || token || autostart || noautostart || query";
}
