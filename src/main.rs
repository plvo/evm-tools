mod supply;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Clone, Debug, clap::ValueEnum)]
enum Network {
    EthereumMainnet,
    EthereumSepolia,
    ArbitrumMainnet,
    ArbitrumSepolia,
}

#[derive(Parser)]
#[command(name = "evm-tools")]
#[command(version = "1.0")]
#[command(about = "CLI tool for interacting with EVM blockchains (github.com/plvo)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    #[arg(default_value = "ethereum-sepolia")]
    #[arg(help = "🌐 Network to use")]
    network: Option<Network>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "🪂 Supply/airdrop wallets with ETH from one wallet to many")]
    Supply {
        #[arg(short, long = "private-key")]
        #[arg(help = "📁 File containing the private key of the supplier wallet to supply from")]
        #[arg(default_value = "./supply/private-key.txt")]
        private_key_path: String,

        #[arg(short, long)]
        #[arg(help = "📁 File containing all public addresses of the wallets to supply to")]
        #[arg(default_value = "./supply/wallets.txt")]
        wallets_path: String,

        #[arg(short, long)]
        #[arg(help = "🧮 Amount of ETH to supply to each wallet")]
        amount: f64,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    println!("Network: {:?}", cli.network);

    match &cli.command {
        Some(Commands::Supply {
            private_key_path,
            wallets_path,
            amount,
        }) => supply::cmd(private_key_path, wallets_path, amount).await,
        None => {
            println!("{}", Cli::command().render_long_help());
        }
    }
}
