#![allow(warnings)]

mod cmd;
mod constant;
mod model;
mod signer;
mod utils;

use clap::{CommandFactory, Parser, Subcommand};
use cmd::{create_wallet, supply};
use utils::{get_rpc_from_config, is_file_exists, open_or_create_file, read_config_file};

#[derive(Parser)]
#[command(name = "evm-tools")]
#[command(version = "1.0")]
#[command(about = "CLI tool for interacting with EVM blockchains (github.com/plvo)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    #[arg(default_value = "arbitrum-sepolia")]
    #[arg(help = "🌐 Network to use")]
    network: String,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "🪂 Supply/airdrop wallets with ETH from one wallet to many")]
    Supply {
        #[arg(short, long)]
        #[arg(help = "📁 File containing all public addresses of the wallets to supply to")]
        #[arg(default_value = "./wallets/wallets_to_supply.txt")]
        wallets_path: String,

        #[arg(short, long)]
        #[arg(help = "🧮 Amount of ETH to supply to each wallet")]
        amount: f64,
    },

    #[command(about = "🔑 Create multiple wallets and save them")]
    CreateWallet {
        #[arg(short, long)]
        #[arg(help = "🔢 Number of wallets to create")]
        #[arg(default_value = "1")]
        count: u32,
    },
}

#[tokio::main]
async fn main() {
    utils::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Supply {
            wallets_path,
            amount,
        }) => {
            if amount < &0.0 || amount == &0.0 {
                log::error!("Amount must be greater than 0");
                std::process::exit(1);
            }

            if is_file_exists(wallets_path) == false {
                log::error!("File with wallets to supply to not found ({}). Please create the file and try again", wallets_path);
                std::process::exit(1);
            }

            let config_data = read_config_file();
            let private_key = config_data.supplier_private_key.clone();

            let rpc_url =
                get_rpc_from_config(Some(config_data), &cli.network).unwrap_or_else(|| {
                    println!("Network not found in config file");
                    std::process::exit(1);
                });

            supply::cmd(&rpc_url, &private_key, wallets_path, amount).await
        }

        Some(Commands::CreateWallet { count }) => create_wallet::cmd(count).await,

        None => println!("{}", Cli::command().render_long_help()),
    }
}
