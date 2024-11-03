use std::env;
use std::string::String;

use json::JsonValue;
use reqwest::blocking::get;

use poise::serenity_prelude as serenity;

use json::JsonValue::*;

use rand::prelude::*;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Handler;
const KEYWORDS: [&str; 10] = [
    "Clinton", "Obama", "Biden", "China", "Iran", "Russia", 
    "Syria", "Bush", "Sanders", "illegal immigration"
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

fn format_book_name(mut book: String) -> String {
    book = book.to_ascii_lowercase();
    let mut book: Vec<char> = book.chars().collect();
    'set_first_char_upper: for i in 0..book.len() {
        if book[i].is_alphabetic() {
            book[i] = book[i].to_uppercase().next().unwrap();
            break 'set_first_char_upper;
        }
    }
    book.into_iter().collect()
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
    let book = format_book_name(book.unwrap());
    let chapter = chapter.unwrap();
    let start = start.unwrap();
    let end = end.unwrap();
    let root_directory = env::var("CARGO_MANIFEST_DIR").expect("Couldn't find the root directory of the Rust project.");
    let parsing_error_msg = "Just like Trump, the program was unable to read the Bible.";
    let file_path = format!("{}/json/verses-1769.json", root_directory);
    let parsed_json = json::parse(&std::fs::read_to_string(&file_path)
        .expect(&parsing_error_msg))
        .expect(parsing_error_msg);
    let unwrapped_book: &str = &book.to_string();
    let mut verses: Vec<String> = Vec::new();
    for verse in start..end+1 {
        println!("{}", &verse);
        verses.push(parsed_json[format!("{} {}:{}", book, chapter, verse)].as_str().unwrap().to_string());
    }
    let passage = verses.join(" ");
    let response = format!("I love the Bible. It's my favorite book. Let me read you my favorite passage in the WHOLE Bible:
                                    \n\n*\"{passage}\"*\n\n
                                    God bless the USA.");
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN environment variable");
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