use crate::error::ApiError;
use crate::models::Country;
use chrono::{DateTime, Utc};
use image::{Rgb, RgbImage};
use std::fs;

pub struct ImageGenerator;

impl ImageGenerator {
    pub fn generate(
        top_countries: &[Country],
        _total_countries: i32,
        _last_refreshed_at: DateTime<Utc>,
    ) -> Result<(), ApiError> {
        fs::create_dir_all("cache")?;

        let width = 1000;
        let height = 700;
        
        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let intensity = 255 - (y * 255 / height) as u8;
                img.put_pixel(x, y, Rgb([intensity, intensity, 255u8]));
            }
        }

        for (idx, country) in top_countries.iter().enumerate().take(5) {
            let y = 100 + (idx as u32 * 100);
            let bar_width = (country.estimated_gdp.unwrap_or(0.0) / 1_000_000_000.0).min(800.0) as u32;
            
            for dy in 0..60 {
                for dx in 0..bar_width {
                    let x = 50 + dx;
                    if x < width && y + dy < height {
                        img.put_pixel(x, y + dy, Rgb([0u8, 150u8, 0u8]));
                    }
                }
            }
        }

        img.save("cache/summary.png")?;

        Ok(())
    }
}