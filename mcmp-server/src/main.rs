use clap::Parser;


mod server;
mod config;
mod database;
mod cli;


#[tokio::main]
async fn main() {
    env_logger::init();
    let cli = cli::CliArgs::parse();
    let Ok(mut config) = config::Config::parse(cli.config.clone().into()) else {
        return;
    };

    config.append(cli);

    server::start_listening(config).await.unwrap();
    // if let Err(e) = server::start_listening(config).await {
    //     log::error!("Failed to start server: {e}");
    // }
}
