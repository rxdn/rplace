#[derive(Debug, Clone)]
pub struct ColourIndex {
    pub colour_index: usize,
    pub rgb: [u8; 3],
}

// 2 -> FF4500
impl ColourIndex {
    pub const fn new(index: usize, c: [u8; 3]) -> Self {
        Self {
            colour_index: index,
            rgb: c,
        }
    }

    pub fn get_closest(other: [u8; 3]) -> Self {
        let mut mapped = colours()
            .into_iter()
            .map(|colour| {
                let r_diff = 0isize + colour.rgb[0] as isize - other[0] as isize;
                let g_diff = 0isize + colour.rgb[1] as isize - other[1] as isize;
                let b_diff = 0isize + colour.rgb[2] as isize - other[2] as isize;

                let squared = r_diff.pow(2) + g_diff.pow(2) + b_diff.pow(2);

                (colour, (squared as f64).sqrt())
            })
            .collect::<Vec<(ColourIndex, f64)>>();

        mapped.sort_by(|(_, a),(_, b)| a.partial_cmp(b).unwrap());

        mapped[0].clone().0
    }
}

pub fn colours() -> Vec<ColourIndex> {
    vec![
        ColourIndex::new(2, [0xFF, 0x45, 0x00]),
        ColourIndex::new(3, [0xFF, 0x8A, 0x00]),
        ColourIndex::new(4, [0xFF, 0xD6, 0x35]),
        ColourIndex::new(6, [0x00, 0xA3, 0x68]),
        ColourIndex::new(8, [0x7E, 0xED, 0x56]),
        ColourIndex::new(12, [0x24, 0x50, 0xA4]),
        ColourIndex::new(13, [0x36, 0x90, 0xEA]),
        ColourIndex::new(14, [0x51, 0xE9, 0xF4]),
        ColourIndex::new(18, [0x81, 0x1E, 0x9F]),
        ColourIndex::new(19, [0xB4, 0x4A, 0xC0]),
        ColourIndex::new(23, [0xFF, 0x99, 0xAA]),
        ColourIndex::new(25, [0x9C, 0x69, 0x26]),
        ColourIndex::new(27, [0x00, 0x00, 0x00]),
        ColourIndex::new(29, [0x8D, 0x89, 0x90]),
        ColourIndex::new(30, [0xD4, 0xD7, 0xD9]),
        ColourIndex::new(31, [0xFF, 0xFF, 0xFF]),
    ]
}