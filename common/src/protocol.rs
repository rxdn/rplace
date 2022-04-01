use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PixelData {
    pub x: usize,
    pub y: usize,
    pub colour_index: usize,
}
