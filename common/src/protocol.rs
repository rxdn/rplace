use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PixelData {
    pub x: usize,
    pub y: usize,
    pub colour_index: usize,
}

impl PixelData {
    pub fn new(x: usize, y: usize, colour_index: usize) -> Self {
        Self {
            x,
            y,
            colour_index,
        }
    }
}
