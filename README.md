# A bot to bring Former President Donald Trump to your discord server.

This is a simple discord bot built on the Serenity/Poise framework for Rust. When players post certain trigger words in chat, Donald Trump will respond with a tweet relating to that trigger word in some way.

You can also use /bible to retrieve a Bible verse from Donald Trump (he is very strong christian).

# Installation
I'm probably not keeping this bot online for very long but if you want to just build it yourself,
you'll need to ensure you have rustc and cargo installed so that you can compile rust programs.
After that, just do:
```bash
git clone https://github.com/LobsterRoast/trump-bot.git
cd trump-bot
cargo run # Some systems may require openssl-devel or equivalent to be installed
```
Make sure you also set the DISCORD_TOKEN environment variable.

On the offchance that I do for some reason have the bot running, you can add it to your server [here.](https://discord.com/oauth2/authorize?client_id=1300497365735833661)

Attributions:
- [farskipper's json formatted KJV Bible](https://github.com/farskipper/kjv/tree/master)
- [Serenity/Poise Discord Bot Framework](https://github.com/serenity-rs/poise)
- [Tronald Dump API](https://www.tronalddump.io/)
