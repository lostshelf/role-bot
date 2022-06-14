use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// use toml::value; // TODO: Read prefix from configuration file

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        if msg.author.bot != true {
            return;
        }

        if msg.content.contains('>') {
            // TODO: Implement some type of function handler
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name)
    }
}

fn main() {
    let _token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(e) => panic!("Couldn't get token from environment: {}", e),
    };
}