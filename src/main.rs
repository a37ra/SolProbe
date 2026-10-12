use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "SolProbe",
    version = "0.1.0",
    about = "тут будет описание",
    long_about = None)]
struct Cli {
    /// URL Solana RPC-ноды (например, Helius, QuickNode или публичная)
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com/")]
    url: String,
    /// JSON-RPC метод для проверки (например, getSlot, getHealth)
    #[arg(short, long, default_value = "get_slot")]
    method: String,

    /// Таймаут запроса в секундах
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,

}

fn main() {
    println!("Hello, world!");
}
