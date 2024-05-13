use cli::app::Runnable;
use tokio;

mod log;
mod cli;
mod cache;
mod config;
mod domain;
mod persist;
mod scraper;
mod periodic;
mod processing;

#[tokio::main]
async fn main() {
    // 1. Read the runtime configuration
    let config = config::configuration::Configuration::new();

    // 2. Initialize the Application using config from above
    let app = cli::app::App::new(config);

    // 3. Perform blocking application run
    app.run();
}
