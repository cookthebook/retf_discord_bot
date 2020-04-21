use std::env;

mod retf_handler;
use retf_handler::RetfHandler;

use serenity::{
    prelude::*,
};



fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("No Discord token defined in environment");

    let prefix ="!retf";
    let crab =
"      \\
       \\
         _~^~^~_
     \\) /  o o  \\ (/
       '_   -   _'
       / '-----' \\";

    let mut client = Client::new(&token, RetfHandler::new(prefix, crab, 60).unwrap())
        .expect("Could not create client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
