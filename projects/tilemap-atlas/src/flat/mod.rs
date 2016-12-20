use std::path::Path;
use image::{ImageError, ImageResult, RgbaImage};
use image::error::{LimitError, LimitErrorKind};

#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod der;

/// A tile set which commonly used in rpg maker
#[derive(Clone, Debug)]
pub struct TilesetEdge2 {
    /// Raw image buffer.
    image: RgbaImage,
    /// Cache for corner patterns.
    cache: [RgbaImage; 16],
    /// random selection for full tiles.
    fulls: Vec<RgbaImage>,
}

impl TilesetEdge2 {
    /// Create a new tile atlas from a image.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tilemap_atlas::TileAtlas4x6;
    /// use image::RgbaImage;
    /// ```
    pub fn from_rpg_maker(image: RgbaImage) -> Self {
        assert_eq!(image.width() % 4, 0, "image width {} does not divide by 4", image.width());
        assert_eq!(image.height() % 6, 0, "image height {} does not divide by 6", image.height());
        let mut out = Self {
            image,
            cache: Default::default(),
            fulls: vec![],
        };
        // SAFETY: dimensions already checked
        unsafe {
            out.make_cache();
        }
        out
    }
    pub fn load<P>(path: P) -> ImageResult<Self> where P: AsRef<Path> {
        let image = image::open(path)?.to_rgba8();
        if image.width() % 4 != 0 || image.height() % 6 != 0 {
            Err(ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))?
        }
        Ok(Self::from_rpg_maker(image))
    }
    pub fn save<P>(&self, path: P) -> ImageResult<()> where P: AsRef<Path> {
        self.image.save(path)
    }
}

impl TilesetEdge2 {
    /// Get a tile by side relation mask.
    ///
    /// # Arguments
    ///
    /// - **R** = Right
    /// - **U** = Up
    /// - **L** = Left
    /// - **D** = Down
    ///
    /// returns: &ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn get_side(&self, r: bool, u: bool, l: bool, d: bool) -> &RgbaImage {
        let lu = l && u;
        let ru = r && u;
        let ld = l && d;
        let rd = r && d;
        self.get_corner(lu, ld, ru, rd)
    }
    /// Get a tile by corner relation mask.
    ///
    /// # Arguments
    ///
    /// - **LU** = Left Up
    /// - **LD** = Right Up
    /// - **RU** = Left Down
    /// - **RD** = Right Down
    ///
    /// returns: &ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```
    /// use tilemap_atlas::TileAtlas4x6;
    /// ```
    pub fn get_corner(&self, lu: bool, ru: bool, ld: bool, rd: bool) -> &RgbaImage {
        let index = (rd as u8) << 3 | (ld as u8) << 2 | (ru as u8) << 1 | (lu as u8);
        // SAFETY: index must in range `[0b0000, 0b1111]`
        unsafe {
            self.cache.get_unchecked(index as usize)
        }
    }
    fn cell_width(&self) -> u32 {
        self.image.width() / 4
    }
    fn cell_height(&self) -> u32 {
        self.image.height() / 6
    }
    unsafe fn make_cache(&mut self) {
        self.cache[00] = self.make_cell([(0, 0), (1, 0), (0, 1), (1, 1)]);
        self.cache[01] = self.make_cell([(3, 5), (1, 0), (0, 1), (1, 1)]);
        self.cache[02] = self.make_cell([(0, 0), (0, 5), (0, 1), (1, 1)]);
        self.cache[03] = self.make_cell([(1, 5), (2, 5), (0, 1), (1, 1)]);
        self.cache[04] = self.make_cell([(0, 0), (1, 0), (3, 2), (1, 1)]);
        self.cache[05] = self.make_cell([(3, 3), (1, 0), (3, 4), (1, 1)]);
        self.cache[06] = self.make_cell([(0, 0), (0, 5), (3, 2), (1, 1)]);
        self.cache[07] = self.make_cell([(3, 1), (2, 5), (3, 4), (1, 1)]);
        self.cache[08] = self.make_cell([(0, 0), (1, 0), (0, 1), (0, 2)]);
        self.cache[09] = self.make_cell([(3, 5), (1, 0), (0, 1), (0, 2)]);
        self.cache[10] = self.make_cell([(0, 0), (0, 3), (0, 1), (0, 4)]);
        self.cache[11] = self.make_cell([(1, 5), (2, 1), (0, 1), (0, 4)]);
        self.cache[12] = self.make_cell([(0, 0), (1, 0), (1, 2), (2, 2)]);
        self.cache[13] = self.make_cell([(3, 3), (1, 0), (3, 0), (2, 2)]);
        self.cache[14] = self.make_cell([(0, 0), (0, 3), (1, 2), (2, 0)]);
        self.cache[15] = self.make_cell([(1, 3), (2, 3), (1, 4), (2, 4)]);
    }
    // [left up, right up, left down, right down]
    unsafe fn make_cell(&self, index: [(u32, u32); 4]) -> RgbaImage {
        let mut image = RgbaImage::new(self.cell_width() * 2, self.cell_height() * 2);
        for (i, (x, y)) in index.iter().enumerate() {
            let sx = x * self.cell_width();
            let sy = y * self.cell_height();
            for dx in 0..self.cell_width() {
                for dy in 0..self.cell_height() {
                    let x = (i as u32 % 2) * self.cell_width() + dx;
                    let y = (i as u32 / 2) * self.cell_height() + dy;
                    let pixel = self.image.get_pixel(sx + dx, sy + dy);
                    image.put_pixel(x, y, *pixel);
                }
            }
        }
        image
    }
}

/// Must 6 * 8 = 48
pub struct TileAtlas6x8 {
    image: RgbaImage,
}

#[test]
fn test() {
    for r in [false, true] {
        for u in [false, true] {
            for l in [false, true] {
                for d in [false, true] {
                    let idx1 = (r as u8) << 3 | (u as u8) << 2 | (l as u8) << 1 | (d as u8);
                    let lu = l && u;
                    let ru = r && u;
                    let ld = l && d;
                    let rd = r && d;
                    let idx2 = (lu as u8) << 3 | (ru as u8) << 2 | (ld as u8) << 1 | (rd as u8);
                    println!("{} -> {}", idx1, idx2)
                }
            }
        }
    }
}