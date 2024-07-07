use std::env;

use clap::Parser;
use cli::{OBACli, OBASubCommands};
use get_order_book::get_order_books;
use subscribe::subscribe_instrument;
use unsubscribe::unsubscribe_instrument;

pub mod cli;
pub mod subscribe;
pub mod get_order_book;
pub mod unsubscribe;

#[tokio::main]
async fn main() {
    let cli = OBACli::parse();

    let host = env::var("OBA_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("OBA_PORT").unwrap_or_else(|_| String::from("3000"));
    let address = format!("http://{}:{}", host, port);

    match &cli.command {
        OBASubCommands::Subscribe { instrument, depth } => {
            println!("Here");
            subscribe_instrument(&address, instrument, depth).await;            
        },
        OBASubCommands::GetOrderBook { instrument } => {
           get_order_books(&address, instrument).await;
        },
        OBASubCommands::Unsubscribe { instrument } => {
            unsubscribe_instrument(&address, instrument).await;
            
        }
        
    }
    println!("Welcome to OBA CLI")
}
