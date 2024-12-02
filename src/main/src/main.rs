use clap::Parser;
use cmd::{Args, Commands};
use common::{errors, logs};
use tracing::error;

mod cmd;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("{:?}", e);
    }
}

async fn run() -> errors::Result<()> {
    logs::init();

    let args = Args::parse();

    match args.command {
        Commands::Http {} => http::serve().await?,
    }

    Ok(())
}
