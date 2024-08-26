use stb_image::image::{self, LoadResult};
use std::path::Path;

pub struct Image {
    pub width: i32,
    pub height: i32,
    data: Vec<u8>,
}

impl Image {
    pub fn new(filename: &str) -> Result<Self, String> {
        let current_dir = std::env::current_dir().unwrap();
        let relative_path = Path::new("images").join(filename);
        let full_path = current_dir.join(relative_path);

        let image = image::load_with_depth(full_path, 3, false); // Load as RGB with 3 channels

        match image {
            LoadResult::ImageU8(image_data) => {
                let width = image_data.width as i32;
                let height = image_data.height as i32;
                let mut data = image_data.data;

                // Convert sRGB to linear gamma
                for pixel in data.chunks_mut(3) {
                    pixel[0] = Self::srgb_to_linear_u8(pixel[0]);
                    pixel[1] = Self::srgb_to_linear_u8(pixel[1]);
                    pixel[2] = Self::srgb_to_linear_u8(pixel[2]);
                }

                Ok(Self {
                    width,
                    height,
                    data,
                })
            }
            _ => Err(String::from("Failed to load image or unsupported format")),
        }
    }

    fn srgb_to_linear_u8(c: u8) -> u8 {
        let c = c as f32 / 255.0;
        let linear = if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        };
        (linear * 255.0).round() as u8
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> (u8, u8, u8) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return (255, 0, 255); // Magenta for out-of-bounds pixels
        }

        let index = ((y * self.width + x) * 3) as usize;

        (
            self.data[index],     // Red
            self.data[index + 1], // Green
            self.data[index + 2], // Blue
        )
    }
}
