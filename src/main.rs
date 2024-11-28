extern crate uom;
mod config;
mod bmi;
mod bri;
mod handler;

use std::io::{BufReader, Read};
use std::fs::File;
use serenity::prelude::*;
use crate::config::Config;
use crate::handler::Handler;

#[tokio::main]
async fn main() {
    // Read in config file
    let mut reader = BufReader::new(
        File::open("/etc/bulk-bot.toml")
            .expect("Expected /etc/bulk-bot.toml to exist and be readable"),
    );

    // Parse config file
    let mut config_data = String::new();
    reader
        .read_to_string(&mut config_data)
        .expect("Expected valid UTF-8 in config file");

    let config_data: Config = toml::from_str(&config_data).expect("Invalid config file format");

    let mut client = Client::builder(
        &config_data.discord_token,
        GatewayIntents::GUILD_SCHEDULED_EVENTS | GatewayIntents::GUILD_MESSAGES,
    )
    .application_id(config_data.application_id.into())
    .event_handler(Handler::new())
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
