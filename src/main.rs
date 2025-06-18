use std::env;
use escpos_rs::{Printer, PrinterProfile};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if (msg.guild_id != None && msg.channel_id != 1384746031748616233) {
            return;
        }

        if let Err(e) = msg.react(&ctx.http, '🍞').await {
            println!("Reaction Fail: {e:?}");
        }  

        let printer_details = PrinterProfile::usb_builder(0x4b43, 0x3830).build();

        let printer: Printer = match Printer::new(printer_details) {
            Ok(maybe_printer) => match maybe_printer {
                Some(printer) => printer,
                None => panic!("No printer was found :(")
            },
            Err(e) => panic!("Error: {}", e)
        };
            
        let _res = printer.println(msg.content);
        // let _res = printer.cut();

    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
