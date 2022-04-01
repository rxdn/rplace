mod config;
use config::Config;

mod server;
use server::Server;

mod image;

mod colour_index;
pub use colour_index::ColourIndex;
use crate::image::convert;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::read();

    /*
    let pixel_data = vec![
        PixelData::new(0, 0, 0),
        PixelData::new(1, 1, 1),
        PixelData::new(2, 2, 2),
        PixelData::new(3, 3, 3),
    ];
     */

    let pixel_data = convert(&config);
    println!("{:?}", pixel_data);

    let server = Server::new(config, pixel_data);
    server.start().await;
}
