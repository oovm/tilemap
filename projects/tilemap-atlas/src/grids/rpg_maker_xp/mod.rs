use super::*;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridCornerRMVXFile {
    key: String,
    cell_w: u32,
    cell_h: u32,
}

/// A corner type tile set used in [RPG Maker 2000](), [RPG Maker 2003](), [RPG Maker XP]().
///
/// ## Example
///
/// ![]()
pub struct GridCornerRMXP {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridCornerRMVXFile {
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub fn new(key: &str, width: u32, height: u32) -> Self {
        Self { key: key.to_string(), cell_w: width, cell_h: height }
    }
    pub fn as_standard(&self, key: &str, image: &RgbaImage) -> ImageResult<(GridCornerAtlas, RgbaImage)> {
        let mut output = RgbaImage::new(self.cell_w * 16, self.cell_h);
        for i in 0..16 {
            let view = view_rpg4x6_cell(image, i as u8)?;
            output.copy_from(&view, i * self.cell_w, 0)?;
        }
        Ok((GridCornerAtlas { key: key.to_string(), cell_w: self.cell_w, cell_h: self.cell_h, count: [1; 16] }, output))
    }
    pub fn make_complete(raw: &RgbaImage, width: u32, height: u32) -> RgbaImage {
        const C: u32 = 24;
        const L: u32 = 8;
        let mut output = RgbaImage::new(width * C, height * L);
        for i in 0..C {
            for j in 0..L {
                let (x, y) = rpg4x6_to_complete(i, j);
                let view = raw.view(x * width, y * height, width, height);
                output.copy_from(&*view, i * width, j * height).unwrap();
            }
        }
        output
    }
}

// getters
impl GridCornerRMVXFile {
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub fn get_key(&self) -> &str {
        &self.key
    }
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub fn get_path(&self, root: &Path) -> PathBuf {
        root.join(&self.key)
    }
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub fn get_image(&self, root: &Path) -> ImageResult<RgbaImage> {
        Ok(image::open(self.get_path(root))?.to_rgba8())
    }
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub fn load_corner(&self, root: &Path, mask: u8) -> ImageResult<RgbaImage> {
        debug_assert!(mask >= 16, "corner mask {} is not in range [0b0000, 0b1111]", mask);
        let image = self.get_image(root)?;
        Ok(view_rpg4x6_cell(&image, mask)?)
    }
}

/// ```js
/// 0b0000 <- [(1, 1), (2, 1), (1, 2), (2, 2)]
/// 0b0001 <- [(4, 6), (2, 1), (1, 2), (2, 2)]
/// 0b0010 <- [(1, 1), (1, 6), (1, 2), (2, 2)]
/// 0b0011 <- [(2, 6), (3, 6), (1, 2), (2, 2)]
/// 0b0100 <- [(1, 1), (2, 1), (4, 3), (2, 2)]
/// 0b0101 <- [(4, 4), (2, 1), (4, 3), (2, 2)]
/// 0b0110 <- [(1, 1), (2, 1), (3, 4), (2, 2)]
/// 0b0111 <- [(2, 4), (3, 4), (4, 3), (2, 2)]
/// 0b1000 <- [(1, 1), (2, 1), (1, 2), (1, 3)]
/// 0b1001 <- [(4, 6), (2, 1), (1, 2), (1, 3)]
/// 0b1010 <- [(1, 1), (1, 6), (1, 2), (1, 5)]
/// 0b1011 <- [(2, 6), (3, 6), (1, 2), (1, 5)]
/// 0b1100 <- [(1, 1), (2, 1), (4, 3), (3, 3)]
/// 0b1101 <- [(4, 4), (2, 1), (4, 3), (3, 3)]
/// 0b1110 <- [(1, 1), (2, 1), (3, 4), (3, 1)]
/// 0b1111 <- [(2, 4), (3, 4), (4, 3), (3, 5)]
/// ```
fn view_rpg4x6_cell(raw: &RgbaImage, mask: u8) -> ImageResult<RgbaImage> {
    let width = raw.width() / 4;
    let height = raw.height() / 6;
    let xs = match mask {
        0b0000 => [(0, 0), (1, 0), (0, 1), (1, 1)],
        0b0001 => [(3, 5), (1, 0), (0, 1), (1, 1)],
        0b0010 => [(0, 0), (0, 5), (0, 1), (1, 1)],
        0b0011 => [(1, 5), (2, 5), (0, 1), (1, 1)],
        0b0100 => [(0, 0), (1, 0), (3, 2), (1, 1)],
        0b0101 => [(3, 3), (1, 0), (3, 4), (1, 1)],
        0b0110 => [(0, 0), (0, 5), (3, 2), (1, 1)],
        0b0111 => [(3, 1), (2, 5), (3, 4), (1, 1)],
        0b1000 => [(0, 0), (1, 0), (0, 1), (0, 2)],
        0b1001 => [(3, 5), (1, 0), (0, 1), (0, 2)],
        0b1010 => [(0, 0), (0, 3), (0, 1), (0, 4)],
        0b1011 => [(1, 5), (2, 1), (0, 1), (0, 4)],
        0b1100 => [(0, 0), (1, 0), (1, 2), (2, 2)],
        0b1101 => [(3, 3), (1, 0), (3, 0), (2, 2)],
        0b1110 => [(0, 0), (0, 3), (1, 2), (2, 0)],
        0b1111 => [(1, 3), (2, 3), (1, 4), (2, 4)],
        _ => unreachable!(),
    };
    let mut out = RgbaImage::new(width * 2, height * 2);
    for (i, (x, y)) in xs.iter().enumerate() {
        let view = raw.view(*x * width, *y * height, width, height);
        let x = (i as u32 % 2) * width;
        let y = (i as u32 / 2) * height;
        out.copy_from(&view.to_image(), x, y)?;
    }
    Ok(out)
}

fn rpg4x6_to_complete(x: u32, y: u32) -> (u32, u32) {
    match (x, y) {
        //
        (0, 0) => (0, 2),
        (0, 1) => (0, 3),
        (0, 2) => (0, 4),
        (0, 3) => (0, 3),
        (0, 4) => (0, 4),
        (0, 5) => (0, 5),
        (0, 6) => (0, 2),
        (0, 7) => (0, 5),
        //
        (1, 0) => (3, 2),
        (1, 1) => (3, 3),
        (1, 2) => (3, 4),
        (1, 3) => (3, 3),
        (1, 4) => (3, 4),
        (1, 5) => (3, 5),
        (1, 6) => (3, 2),
        (1, 7) => (3, 5),
        //
        (2, 0) => (0, 2),
        (2, 1) => (0, 3),
        (2, 2) => (0, 4),
        (2, 3) => (0, 3),
        (2, 4) => (0, 4),
        (2, 5) => (0, 5),
        (2, 6) => (0, 2),
        (2, 7) => (0, 5),
        //
        (3, 0) => (1, 2),
        (3, 1) => (3, 1),
        (3, 2) => (3, 0),
        (3, 3) => (3, 1),
        (3, 4) => (3, 0),
        (3, 5) => (1, 5),
        (3, 6) => (1, 2),
        (3, 7) => (1, 5),
        //
        (4, 0) => (2, 2),
        (4, 1) => (2, 1),
        (4, 2) => (2, 0),
        (4, 3) => (2, 1),
        (4, 4) => (2, 0),
        (4, 5) => (2, 5),
        (4, 6) => (2, 2),
        (4, 7) => (2, 5),
        //
        (5, 0) => (1, 2),
        (5, 1) => (3, 1),
        (5, 2) => (3, 0),
        (5, 3) => (3, 1),
        (5, 4) => (3, 0),
        (5, 5) => (1, 5),
        (5, 6) => (1, 2),
        (5, 7) => (1, 5),
        //
        (6, 0) => (2, 2),
        (6, 1) => (2, 1),
        (6, 2) => (2, 0),
        (6, 3) => (2, 1),
        (6, 4) => (2, 0),
        (6, 5) => (2, 5),
        (6, 6) => (2, 2),
        (6, 7) => (2, 5),
        //
        (7, 0) => (3, 2),
        (7, 1) => (3, 3),
        (7, 2) => (3, 4),
        (7, 3) => (3, 3),
        (7, 4) => (3, 4),
        (7, 5) => (3, 5),
        (7, 6) => (3, 2),
        (7, 7) => (3, 5),
        //
        (8, 0) => (2, 4),
        (8, 1) => (2, 1),
        (8, 2) => (0, 4),
        (8, 3) => (0, 3),
        (8, 4) => (0, 4),
        (8, 5) => (0, 3),
        (8, 6) => (2, 0),
        (8, 7) => (2, 3),
        //
        (9, 0) => (3, 0),
        (9, 1) => (3, 1),
        (9, 2) => (3, 0),
        (9, 3) => (1, 3),
        (9, 4) => (1, 4),
        (9, 5) => (3, 1),
        (9, 6) => (3, 0),
        (9, 7) => (3, 1),
        //
        (10, 0) => (2, 2),
        (10, 1) => (2, 1),
        (10, 2) => (2, 0),
        (10, 3) => (2, 3),
        (10, 4) => (2, 4),
        (10, 5) => (2, 1),
        (10, 6) => (2, 0),
        (10, 7) => (2, 5),
        //
        (11, 0) => (1, 2),
        (11, 1) => (1, 3),
        (11, 2) => (1, 4),
        (11, 3) => (1, 3),
        (11, 4) => (1, 4),
        (11, 5) => (1, 3),
        (11, 6) => (1, 4),
        (11, 7) => (1, 5),
        //
        (12, 0) => (2, 2),
        (12, 1) => (2, 3),
        (12, 2) => (2, 4),
        (12, 3) => (2, 3),
        (12, 4) => (2, 4),
        (12, 5) => (2, 3),
        (12, 6) => (2, 4),
        (12, 7) => (2, 5),
        //
        (13, 0) => (1, 2),
        (13, 1) => (3, 1),
        (13, 2) => (3, 0),
        (13, 3) => (1, 3),
        (13, 4) => (1, 4),
        (13, 5) => (3, 1),
        (13, 6) => (3, 0),
        (13, 7) => (1, 5),
        //
        (14, 0) => (2, 0),
        (14, 1) => (2, 1),
        (14, 2) => (2, 0),
        (14, 3) => (2, 3),
        (14, 4) => (2, 4),
        (14, 5) => (2, 1),
        (14, 6) => (2, 0),
        (14, 7) => (2, 1),
        //
        (15, 0) => (1, 4),
        (15, 1) => (3, 1),
        (15, 2) => (3, 4),
        (15, 3) => (3, 3),
        (15, 4) => (3, 4),
        (15, 5) => (3, 3),
        (15, 6) => (3, 0),
        (15, 7) => (1, 3),
        //
        (16, 0) => (0, 2),
        (16, 1) => (0, 3),
        (16, 2) => (0, 4),
        (16, 3) => (0, 3),
        (16, 4) => (2, 0),
        (16, 5) => (2, 1),
        (16, 6) => (0, 4),
        (16, 7) => (0, 5),
        //
        (17, 0) => (1, 2),
        (17, 1) => (1, 3),
        (17, 2) => (1, 4),
        (17, 3) => (1, 3),
        (17, 4) => (1, 4),
        (17, 5) => (1, 3),
        (17, 6) => (1, 4),
        (17, 7) => (1, 5),
        //
        (18, 0) => (2, 0),
        (18, 1) => (2, 3),
        (18, 2) => (2, 0),
        (18, 3) => (2, 3),
        (18, 4) => (2, 4),
        (18, 5) => (2, 3),
        (18, 6) => (2, 4),
        (18, 7) => (2, 5),
        //
        (19, 0) => (3, 0),
        (19, 1) => (1, 3),
        (19, 2) => (1, 4),
        (19, 3) => (3, 1),
        (19, 4) => (1, 4),
        (19, 5) => (1, 3),
        (19, 6) => (1, 4),
        (19, 7) => (1, 5),
        //
        (20, 0) => (2, 2),
        (20, 1) => (2, 3),
        (20, 2) => (0, 0),
        (20, 3) => (0, 1),
        (20, 4) => (2, 4),
        (20, 5) => (2, 1),
        (20, 6) => (2, 4),
        (20, 7) => (2, 1),
        //
        (21, 0) => (1, 2),
        (21, 1) => (1, 3),
        (21, 2) => (1, 0),
        (21, 3) => (1, 1),
        (21, 4) => (3, 0),
        (21, 5) => (1, 3),
        (21, 6) => (1, 4),
        (21, 7) => (3, 1),
        //
        (22, 0) => (2, 2),
        (22, 1) => (2, 3),
        (22, 2) => (2, 4),
        (22, 3) => (2, 3),
        (22, 4) => (2, 4),
        (22, 5) => (2, 3),
        (22, 6) => (2, 4),
        (22, 7) => (2, 5),
        //
        (23, 0) => (3, 2),
        (23, 1) => (3, 3),
        (23, 2) => (3, 0),
        (23, 3) => (3, 1),
        (23, 4) => (3, 4),
        (23, 5) => (3, 3),
        (23, 6) => (3, 4),
        (23, 7) => (3, 5),
        _ => unreachable!(),
    }
}
