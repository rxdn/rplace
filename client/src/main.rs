mod config;

use config::Config;

mod reddit_client;

use reddit_client::RedditClient;

use tokio::time::{sleep, Duration};
use std::sync::Arc;

use log::{error, info};
use common::protocol::PixelData;
use common::Result;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Arc::new(Config::read());
    let http_client = reqwest::Client::new();

    let mut handles = Vec::with_capacity(config.tokens.len());
    config.tokens.iter().for_each(|token| {
        let config = Arc::clone(&config);
        let client = RedditClient::new_with_client(token.to_string(), config, http_client.clone());

        let join_handle = tokio::spawn(async move {
            loop {
                let pixel_data = match client.next_pixel().await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Error fetching pixel data: {}", e);
                        sleep(Duration::from_secs(300)).await;
                        continue
                    }
                };

                info!("Got pixel data: {:?}", pixel_data);

                match client.draw(pixel_data).await {
                    Ok(_) => info!("Pixel drawn successfully"),
                    Err(e) => error!("Error drawing pixel: {}", e),
                };

                sleep(Duration::from_secs(300)).await;
            }
        });

        handles.push(join_handle);
    });

    for handle in handles {
        let _ = handle.await;
    }
}

