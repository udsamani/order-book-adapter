use clap::{arg, command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oba")]
#[command(about = "Order Book Adatper CLI to interact with Order Book Adapter Server")]
pub struct OBACli {
    #[command(subcommand)]
    pub command: OBASubCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum OBASubCommands {
    #[command(about = "Subscribe to an instrument")]
    Subscribe {
        #[arg(short, long)]
        instrument: String,

        #[arg(short, long)]
        depth: usize,
    },
    #[command(about = "Get Order Book")]
    GetOrderBook {
        #[arg(short, long)]
        instrument: String,
    },
    #[command(about = "Unsubscribe an instrument's order book")]
    Unsubscribe {
        #[arg(short, long)]
        instrument: String,
    },
    #[command(about = "Command to fetch the best bid for the instrument")]
    BestBid {
        #[arg(short, long)]
        instrument: String,
    },
    #[command(about = "Command to fetch the best ask for the instrument")]
    BestAsk {
        #[arg(short, long)]
        instrument: String,
    },
}
