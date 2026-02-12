use crate::platform::file_utils::FileUtils;
use image::GenericImageView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    JPG,
    PNG,
    TIFF,
    WEBP,
    PVR,
    ETC,
    S3TC,
    ATITC,
    TGA,
    RAW_DATA,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
    format: ImageFormat,
    has_alpha: bool,
    premultiplied_alpha: bool,
    number_of_mipmaps: i32,
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

impl Image {
    pub fn new() -> Image {
        Image {
            data: Vec::new(),
            width: 0,
            height: 0,
            format: ImageFormat::UNKNOWN,
            has_alpha: false,
            premultiplied_alpha: false,
            number_of_mipmaps: 0,
        }
    }

    pub fn with_file(path: &str) -> Option<Image> {
        let mut image = Image::new();
        if image.init_with_file(path) {
            Some(image)
        } else {
            None
        }
    }

    pub fn init_with_file(&mut self, path: &str) -> bool {
        let file_utils = FileUtils::get_instance();
        let full_path = if let Some(path) = file_utils.get_full_path(path) {
            path
        } else {
            // Fallback: try relative path if not found in search paths
            std::path::PathBuf::from(path)
        };

        if !full_path.exists() {
            log::error!("Image file not found: {}", path);
            return false;
        }

        // Open file using image crate
        let img = match image::open(&full_path) {
            Ok(img) => img,
            Err(e) => {
                log::error!("Failed to load image: {}, error: {}", path, e);
                return false;
            }
        };

        self.width = img.width();
        self.height = img.height();

        // Convert to RGBA8
        let rgba = img.to_rgba8();
        self.data = rgba.into_raw();
        self.has_alpha = true; // RGBA always has alpha
        self.premultiplied_alpha = false; // standard image loaders usually don't premultiply
        self.format = ImageFormat::PNG; // Treating internal data as generic RGBA, similar to PNG
        self.number_of_mipmaps = 0;

        true
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn has_alpha(&self) -> bool {
        self.has_alpha
    }

    pub fn is_premultiplied_alpha(&self) -> bool {
        self.premultiplied_alpha
    }
}
