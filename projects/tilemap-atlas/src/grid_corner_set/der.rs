use std::fmt::Formatter;
use image::{GenericImage, ImageResult};
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;
use crate::utils::dimension_error;
use super::*;

struct VisitorAtlas4x6;

impl<'de> Deserialize<'de> for TailCornerAtlas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_map(VisitorAtlas4x6)
    }
}


impl<'de> Visitor<'de> for VisitorAtlas4x6 {
    type Value = TailCornerAtlas;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except TileAtlas4x6 {width, height, image}")
    }
}


impl GridCornerAtlas {
    /// A 4*4
    pub fn from_wang(wang: &RgbaImage) -> ImageResult<Self> {
        let cell_size = wang.width() / 4;
        if wang.width() % 4 != 0 || wang.width() != wang.height() {
            dimension_error()?
        }
        let mut out = Self {
            image: RgbaImage::new(cell_size * 16, cell_size),
            count: [1; 16],
        };
        for i in 0..16 {
            let view = make_wing_cell(wang, i, cell_size);
            out.image.copy_from(&view.to_image(), i * cell_size, 0)?;
        }
        Ok(out)
    }
}

// 0b0000 <- 0  <- (1, 4)
// 0b0001 <- 2  <- (2, 4)
// 0b0010 <- 1  <- (1, 3)
// 0b0011 <- 3  <- (2, 3)
// 0b0100 <- 8  <- (4, 4)
// 0b0101 <- 10 <- (3, 4)
// 0b0110 <- 9  <- (4, 3)
// 0b0111 <- 11 <- (3, 3)
// 0b1000 <- 4  <- (1, 1)
// 0b1001 <- 6  <- (2, 1)
// 0b1010 <- 5  <- (1, 2)
// 0b1011 <- 7  <- (2, 2)
// 0b1100 <- 12 <- (4, 1)
// 0b1101 <- 14 <- (3, 1)
// 0b1110 <- 13 <- (4, 2)
// 0b1111 <- 15 <- (3, 2)
fn make_wing_cell(raw: &RgbaImage, id: u32, s: u32) -> SubImage<&RgbaImage> {
    match id {
        0b0000 => raw.view(0 * s, 3 * s, s, s),
        0b0001 => raw.view(3 * s, 3 * s, s, s),
        0b0010 => raw.view(0 * s, 2 * s, s, s),
        0b0011 => raw.view(1 * s, 2 * s, s, s),
        0b0100 => raw.view(0 * s, 0 * s, s, s),
        0b0101 => raw.view(3 * s, 2 * s, s, s),
        0b0110 => raw.view(2 * s, 3 * s, s, s),
        0b0111 => raw.view(3 * s, 1 * s, s, s),
        0b1000 => raw.view(1 * s, 3 * s, s, s),
        0b1001 => raw.view(0 * s, 1 * s, s, s),
        0b1010 => raw.view(1 * s, 0 * s, s, s),
        0b1011 => raw.view(2 * s, 2 * s, s, s),
        0b1100 => raw.view(3 * s, 0 * s, s, s),
        0b1101 => raw.view(2 * s, 0 * s, s, s),
        0b1110 => raw.view(1 * s, 1 * s, s, s),
        0b1111 => raw.view(2 * s, 1 * s, s, s),
        _ => unreachable!(),
    }
}