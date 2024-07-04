use crate::api::BuildApi;
use crate::controller::Controller;
use clap::{Parser, Subcommand};
use tracing::Level;

#[derive(Parser)]
#[clap(
    name = "argocd-sync",
    about = "Sync ArgoCD application from GitHub Actions using the API."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Sync {
        #[clap(long)]
        address: String,
        #[clap(long)]
        token: String,
        #[clap(long)]
        application: String,
        #[clap(long)]
        image_tag: String,
        #[clap(long)]
        debug: Option<bool>,
    },
    Version,
}

pub async fn handle_command(cli: Cli) {
    match cli.command {
        Commands::Sync {
            address,
            token,
            application,
            image_tag,
            debug,
        } => {
            if debug.unwrap_or(false) {
                tracing_subscriber::fmt()
                    .with_max_level(Level::DEBUG)
                    .init();
            };

            let api = BuildApi::new(address, token, debug.unwrap_or(false));

            let controller = Controller::new(api);
            if let Err(e) = controller.sync(&application, &image_tag).await {
                eprintln!("Failed to sync application: {}", e);
            } else {
                println!("Application synced successfully.");
            }
        }
        Commands::Version => {
            println!("Version {}", env!("CARGO_PKG_VERSION")); // Worth creating a proper config handler if more env vars are needed.
        }
    }
}
