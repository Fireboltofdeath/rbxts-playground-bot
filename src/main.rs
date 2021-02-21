pub mod commands;
mod events;
mod groups;
pub mod util;

use std::env;

use dotenv::dotenv;
use events::*;
use groups::*;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
