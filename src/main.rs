use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use toml::value;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot != true {
            return;
        }

        if msg.content.contains('>') {

        }
    }
}

fn main() {
    let _token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(e) => panic!("Couldn't get token from environment: {}", e),
    };


}