use clap::Parser;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};

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
struct RpcRequest<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct RpcResponse {
    result: Option<serde_json::Value>,
    error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    code: i64,
    message: String,
}

impl<'a> RpcRequest<'a> {
    fn new(method: &'a str) -> Self {
        Self { jsonrpc: "2.0", id: 1, method, params: vec![] }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(cli.timeout))
        .build()?;

    let started = Instant::now();

    let result = client
        .post(&cli.url)
        .json(&RpcRequest::new(&cli.method))
        .send()
        .await
        .and_then(|r| r.error_for_status());

    let resp = match result {
        Ok(r) => r,
        Err(e) if e.is_timeout() => {
            eprintln!("Таймаут: нода не ответила за {} с", cli.timeout);
            std::process::exit(1);
        }
        Err(e) if e.is_connect() => {
            eprintln!("Не удалось подключится к {}", cli.url);
            std::process::exit(1);
        }
        Err(e) => return Err(e),
        
    };

    let rpc: RpcResponse = resp.json().await?;
    let elapsed = started.elapsed();

    match (rpc.result, rpc.error) {
        (Some(result), _) => {
            println!("OK за {} мс", elapsed.as_millis());
            println!("{}: {}", cli.method, result);
        }
        (None, Some(err)) => {
            eprintln!("RPC-ошибка {}: {}", err.code, err.message);
            std::process::exit(2);
        }
        (None, None) => {
            eprintln!("Пустой ответ без result и error");
            std::process::exit(2);
        }
    }
    Ok(())
}
