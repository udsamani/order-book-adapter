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

    match &cli.command {
        OBASubCommands::Subscribe { instrument, depth } => {
            println!("Here");
            subscribe_instrument(instrument, depth).await;            
        },
        OBASubCommands::GetOrderBook { instrument } => {
           get_order_books(instrument).await;
        },
        OBASubCommands::Unsubscribe { instrument } => {
            unsubscribe_instrument(instrument).await;
            
        }
        
    }
    println!("Welcome to OBA CLI")
}
