use crate::config::Config;
use axum::routing::get;
use axum::{Extension, Json, Router};
use common::protocol::PixelData;
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

pub struct Server {
    config: Config,
    pointer: Arc<Mutex<usize>>,
    pixel_data: Vec<PixelData>,
}

impl Server {
    pub fn new(config: Config, pixel_data: Vec<PixelData>) -> Self {
        Self {
            config,
            pointer: Arc::new(Mutex::new(0)),
            pixel_data,
        }
    }

    pub async fn start(self) {
        let addr = SocketAddr::from_str(self.config.server_addr.as_str())
            .expect("Failed to parse server address");

        let server = Arc::new(self);

        let router = Router::new()
            .route("/", get(Self::handler))
            .layer(Extension(server));

        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .expect("Failed to start server");
    }

    async fn handler(server: Extension<Arc<Self>>) -> Json<PixelData> {
        Json(server.next_pixel())
    }

    fn next_pixel(&self) -> PixelData {
        let mut pointer = self.pointer.lock();
        let data = self.pixel_data.get(*pointer).unwrap();

        *pointer += 1;
        if *pointer == self.pixel_data.len() {
            *pointer = 0;
        }

        data.clone()
    }
}
