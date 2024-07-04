mod api;
mod app;
mod cli;
mod controller;
mod error;

use clap::Parser;
use cli::{handle_command, Cli};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    handle_command(cli).await;
}
