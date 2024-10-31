use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::str::from_utf8;
use std::string::String;
use std::str::SplitWhitespace;
use std::collections::HashMap;

use json::JsonValue;
use reqwest::blocking::get;
use reqwest::Error;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use json::JsonValue::*;

use curl::easy::Easy;

use rand::prelude::*;

const KEYWORDS: [&str; 10] = ["Clinton", "Obama", "Biden", "China", "Iran", "Russia", "Syria", "Jeb Bush", "Bernie Sanders", "illegal immigration"];
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        for keyword in KEYWORDS {
            if msg.content.to_ascii_lowercase().contains(&keyword.to_ascii_lowercase()) && msg.author.id != 1300497365735833661 {
                if let Err(why) = msg.channel_id.say(&ctx.http, get_trump_quote(&keyword.to_string()).await.unwrap()).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}
async fn get_trump_quote(keyword: &str) -> Result<String, Error> {
    let url = format!("https://api.tronalddump.io/search/quote?query={}", keyword);
    let response = reqwest::get(&url).await?.text().await?;
    let json = json::parse(&response).expect("Failed to parse json.");
    let count: usize = json["count"].as_usize().expect("Unable to parse json data to usize.");
    let rand = rand::thread_rng().gen_range(0..=count-1);
    let quote: &str = json["_embedded"]["quotes"][rand]["value"].as_str().unwrap();
    Ok(quote.to_string())
}

// Rust doesn't have a good substring function so I made one
fn substring(string: &String, pos: &usize, len: &usize) -> String {
    let return_string: String = string.chars().skip(*pos).take(*len).collect();
    println!("{}", &return_string);
    return return_string;
}

// This is a variant of substring that just returns the rest of the string instead of having a specified length.
// There is no function overloading in Rust.
fn substring_no_len(string: &String, pos: &usize) -> String {
    let return_string: String = string.chars().skip(*pos).collect();
    println!("{}", &return_string);
    return return_string;
}

fn get_bible_verse() -> String {
    let root_directory = env::var("CARGO_MANIFEST_DIR").expect("Couldn't find the root directory of the Rust project.");
    let mut file = File::open(format!("{}/json/en_kjv.json", root_directory)).expect("Just like Trump, the program was unable to read the Bible.");
    let mut contents = String::new();
    //let size = file.read_to_string(&mut contents).expect("Just like Trump, the program was unable to read the Bible.");
    return contents;
}
#[tokio::main]
async fn main() {
    get_bible_verse();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}