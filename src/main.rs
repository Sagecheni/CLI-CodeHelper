mod api;
mod cli;
mod utils;
#[tokio::main]

async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    cli::start_interactive_mode().await
}
