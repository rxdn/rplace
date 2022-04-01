use image::{GenericImageView, Pixel};
use image::imageops::FilterType;
use common::protocol::PixelData;
use crate::{ColourIndex, Config};

pub fn convert(config: &Config) -> Vec<PixelData> {
    let mut img = image::open("image.png")
        .expect("Failed to read image");

    img = img.resize(config.image_width, config.image_height, FilterType::Lanczos3);

    let mut pixel_data = vec![];

    for (x, y, colour_data) in img.pixels() {
        let colour_index = ColourIndex::get_closest(colour_data.to_rgb().0);
        pixel_data.push(PixelData::new(
            config.grid_offset_x + x as usize,
            config.grid_offset_y + y as usize,
            colour_index.colour_index,
        ));
    }

    pixel_data
}