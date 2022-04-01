use common::Result;
use common::protocol::PixelData;
use crate::config::Config;
use std::sync::Arc;

pub struct RedditClient {
    token: String,
    config: Arc<Config>,
    client: reqwest::Client,
}

impl RedditClient {
    pub fn new(token: String, config: Arc<Config>) -> Self {
        Self::new_with_client(token, config, reqwest::Client::new())
    }

    pub fn new_with_client(token: String, config: Arc<Config>, client: reqwest::Client) -> Self {
        Self {
            token,
            config,
            client
        }
    }

    pub async fn draw(&self, pixel_data: PixelData) -> Result<()> {
        // TODO: Error handling
        self.client.post("https://gql-realtime-2.reddit.com/query")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.84 Safari/537.36'")
            .header("content-type", "application/json")
            .header("authorization", format!("bearer {}", self.token))
            .body(Self::build_body(pixel_data.x, pixel_data.y, pixel_data.colour_index))
            .send()
            .await?;

        Ok(())
    }
    
    fn build_body(x: usize, y: usize, colour_index: usize) -> String {
        format!(r#"{{"operationName":"setPixel","variables":{{"input":{{"actionName":"r/replace:set_pixel","PixelMessageData":{{"coordinate":{{"x":{},"y":{}}},"colorIndex":{},"canvasIndex":0}}}}}},"query":"mutation setPixel($input: ActInput!) {{\n  act(input: $input) {{\n    data {{\n      ... on BasicMessage {{\n        id\n        data {{\n          ... on GetUserCooldownResponseMessageData {{\n            nextAvailablePixelTimestamp\n            __typename\n          }}\n          ... on SetPixelResponseMessageData {{\n            timestamp\n            __typename\n          }}\n          __typename\n        }}\n        __typename\n      }}\n      __typename\n    }}\n    __typename\n  }}\n}}\n"}}"#, x, y, colour_index)
    }

    pub async fn next_pixel(&self) -> Result<PixelData> {
        let res = self.client.get(self.config.api_url.as_str())
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}
