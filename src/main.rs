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

use poise::serenity_prelude as serenity;

use json::JsonValue::*;

use curl::easy::Easy;

use rand::prelude::*;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Handler;
const KEYWORDS: [&str; 10] = [
    "Clinton", "Obama", "Biden", "China", "Iran", "Russia", 
    "Syria", "Bush", "Sanders", "illegal immigration"
];
const BOOKS: [&str; 66] = [
    "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy",
    "Joshua", "Judges", "Ruth",
    "1 Samuel", "2 Samuel", "1 Kings", "2 Kings", "1 Chronicles", "2 Chronicles",
    "Ezra", "Nehemiah", "Esther",
    "Job", "Psalms", "Proverbs", "Ecclesiastes", "Song of Solomon",
    "Isaiah", "Jeremiah", "Lamentations", "Ezekiel", "Daniel",
    "Hosea", "Joel", "Amos", "Obadiah", "Jonah", "Micah", "Nahum",
    "Habakkuk", "Zephaniah", "Haggai", "Zechariah", "Malachi",
    "Matthew", "Mark", "Luke", "John",
    "Acts", "Romans", "1 Corinthians", "2 Corinthians",
    "Galatians", "Ephesians", "Philippians", "Colossians",
    "1 Thessalonians", "2 Thessalonians", "1 Timothy", "2 Timothy",
    "Titus", "Philemon",
    "Hebrews", "James", "1 Peter", "2 Peter", "1 John", "2 John", "3 John",
    "Jude", "Revelation",
];
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

// Posts a bible passage in chat
#[poise::command(slash_command, prefix_command)]
async fn bible(
    ctx: Context<'_>,
    #[description = "Book of the Bible"] book: Option<String>,
    #[description = "Chapter of the selected book"] chapter: Option<String>,
    #[description = "Start Verse"] start: Option<u8>,
    #[description = "End Verse (May be blank)"] end: Option<u8>
) -> Result<(), Error> {
    let root_directory = env::var("CARGO_MANIFEST_DIR").expect("Couldn't find the root directory of the Rust project.");
    let mut file = File::open(format!("{}/json/en_kjv.json", root_directory)).expect("Just like Trump, the program was unable to read the Bible.");
    let mut contents = String::new();
    let unwrapped_book: &str = &book.unwrap().to_string();
    if !BOOKS.contains(&unwrapped_book) {
        let response = format!("{} is not a valid book in the protestant Bible you Kamala-voting heathen.", &unwrapped_book);
        ctx.say(response).await?;
        return Ok(())
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![bible()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}