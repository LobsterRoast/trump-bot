// Trump Bot by LobsterRoast
// Do whatever you want with it

use std::env;
use std::string::String;

use json::JsonValue;
use reqwest::blocking::get;

use poise::serenity_prelude as serenity;
use poise::serenity_prelude::EventHandler;
use poise::serenity_prelude::Message;
use poise::serenity_prelude::Context as OtherContext;
use poise::serenity_prelude::CacheHttp;
use poise::async_trait;

use json::JsonValue::*;

use rand::prelude::*;
use ::serenity::all::Event;

struct Data {}
struct Handler;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Defines Keywords that Trump will respond to
const KEYWORDS: [&str; 10] = [
    "Clinton", "Obama", "Biden", "China", "Iran", "Russia", 
    "Syria", "Bush", "Sanders", "illegal immigration"
    ];

// Implements the EventHandler trait in the Handler struct. This will handle things like detecting when a message is sent
#[async_trait]
impl EventHandler for Handler {
    // message() runs whenever a message is sent
    async fn message(&self, ctx: OtherContext, msg: Message) {
        for keyword in KEYWORDS {
            // we use to_ascii_lowercase() on both the sent message and the keywords to ensure that it's case insensitive
            // Also checks msg.author.id to ensure that the bot doesn't respond to itself
            if msg.content.to_ascii_lowercase().contains(&keyword.to_ascii_lowercase()) && msg.author.id != 1300497365735833661 {
                if let Err(why) = msg.channel_id.say(&ctx.http(), get_trump_quote(&keyword.to_string()).await.unwrap()).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}

// Function to call from the TronaldDump.io API to get Trump tweets. Keyword is passed to the function
async fn get_trump_quote(keyword: &str) -> Result<String, Error> {
    // Gets a tweet with the given keyword in json format
    let url = format!("https://api.tronalddump.io/search/quote?query={}", keyword);
    let response = reqwest::get(&url).await?.text().await?;
    let json = json::parse(&response).expect("Failed to parse json.");
    // Check how many tweets were provided in the json and picks a random one
    let count: usize = json["count"].as_usize().expect("Unable to parse json data to usize.");
    let rand = rand::thread_rng().gen_range(0..=count-1);
    // Gets the actual content of the tweet from the json
    let quote: &str = json["_embedded"]["quotes"][rand]["value"].as_str().unwrap();
    Ok(quote.to_string())
}

// This function is just to make sure that the /bible command is case-insensitve. It sets the first alphabetic character to uppercase.
// It also sets everything else to lowercase. That is how the json containing the Bible data is formatted.
fn format_book_name(mut book: String) -> String {
    book = book.to_ascii_lowercase();
    let mut book: Vec<char> = book.chars().collect();
    'set_first_letter_upper: for i in 0..book.len() {
        if book[i].is_alphabetic() {
            book[i] = book[i].to_uppercase().next().unwrap();
            break 'set_first_letter_upper;
        }
    }
    book.into_iter().collect()
}
// This is the code that runs when /bible is run
#[poise::command(slash_command, prefix_command)]
async fn bible(
    ctx: Context<'_>,
    #[description = "Book of the Bible"] book: Option<String>, // Parameter to choose a book of the Bible
    #[description = "Chapter of the selected book"] chapter: Option<String>, // Parameter to choose a chapter
    #[description = "Start Verse"] start: Option<u8>, // Parameter to choose a starting verse
    #[description = "End Verse"] end: Option<u8> // Parameter to choose an ending verse
) -> Result<(), Error> {
    // Unwraps all the parameters so that they're usable
    let book = format_book_name(book.unwrap());
    let chapter = chapter.unwrap();
    let start = start.unwrap();
    let end = end.unwrap();
    let parsing_error_msg = "Just like Trump, the program was unable to read the Bible.";
    let file_path = format!("{}/json/verses-1769.json", 
                                    env::var("CARGO_MANIFEST_DIR")
                                    .expect("Couldn't find the root directory of the Rust project."));
    // Parses the json file at file_path
    let parsed_json = json::parse(&std::fs::read_to_string(&file_path)
        .expect(&parsing_error_msg))
        .expect(parsing_error_msg);
    let unwrapped_book: &str = &book.to_string();
    // Defines an empty vector of strings to which all requested verses can be pushed to
    let mut verses: Vec<String> = Vec::new();
    for verse in start..end+1 {
        verses.push(parsed_json[format!("{} {}:{}", book, chapter, verse)].as_str().unwrap().to_string());
    }
    // Joins the verses vector into a single string
    let passage = verses.join(" ");
    let response = format!("I love the Bible. It's my favorite book. Let me read you my favorite passage in the WHOLE Bible:\n\n*\"{passage}\"*\n\nGod bless the USA.");
    // Prints the response to the chat
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Gets the bot token from the DISCORD_TOKEN environment variable
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN environment variable");
    // Gives the bot the MESSAGE_CONTENT intent so that it can read messages
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework= poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // Sets the bible() method to be registered
            commands: vec![bible()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Registers all commands (in this case, just bible())
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    // Initializes the bot and attaches the an event handler to it
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await;
    client.unwrap().start().await.unwrap();
}
