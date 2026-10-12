use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
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
    #[arg(short, long, default_value = "getSlot")]
    method: String,

    /// Таймаут запроса в секундах
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,

}

#[derive(Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: Vec<serde_json::Value>,
}

impl<'a> JsonRpcRequest<'a> {
    fn new(method: &'a str) -> Self {
        Self { jsonrpc: "2.0", id: 1, method, params: vec![] }
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("{:#?}", args)
}
