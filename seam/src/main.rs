mod declarative;


use crate::declarative::{get_source_url, Commands};
use anyhow::Result;
use clap::Parser;

/// 获取直播源
#[derive(Debug, Parser)]
#[command(name = "seam")]
#[command(about ="
________ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    get_source_url().await?;
    Ok(())
}