/*
Last updated: 11-15-2023

Description: This crate defines the main thread of the data mesh node

Author: James Dean
*/
use server::run_server;
use config::load_config;
use logging;

const CONFIG_FILEPATH: &str = "../config.json";

#[tokio::main]
async fn main() {
    // Initialize logging
    logging::init();

    // Load configuration
    let config = load_config(CONFIG_FILEPATH)
        .expect("Failed to load config");

    // Start the web server
    log::info!("Starting server on port: {}", config.port);
    run_server(config.port).await;
}
