pub mod formats;
pub mod conversions;
pub mod errors;

use formats::{RGB, HSV, Hex};
use errors::InvalidColourFormat;

pub struct Colour {
    rgb: RGB,
    hsv: HSV,
    hex: Hex,
}

// For the sake of better code while I learn rust, I'm generating all of the formats in the
// constructor. A better use of memory would be to generate & store other formats on-demand
impl Colour {
    pub fn new_from_rgb(r: u8, g: u8, b: u8) -> Result<Colour, InvalidColourFormat> {
        let rgb = RGB::new(r, g, b)?;
        
        Ok(Colour {
            rgb: rgb.clone(),
            hsv: conversions::rgb_to_hsv(&rgb)?,
            hex: conversions::rgb_to_hex(&rgb)?,
        })
    }

    pub fn new_from_hsv(h: f32, s: f32, v: f32) -> Result<Colour, InvalidColourFormat> {
        let hsv = HSV::new(h, s, v)?;

        Ok(Colour {
            rgb: conversions::hsv_to_rgb(&hsv)?,
            hsv: hsv.clone(),
            hex: conversions::hsv_to_hex(&hsv)?,
        })
    }

    pub fn new_from_hex(hex: &str) -> Result<Colour, InvalidColourFormat> {
        let hex = Hex::new(hex)?;

        Ok(Colour {
            rgb: conversions::hex_to_rgb(&hex)?,
            hsv: conversions::hex_to_hsv(&hex)?,
            hex: hex.clone(),
        })
    }

    pub fn rgb(&self) -> &RGB {
        &self.rgb
    }

    pub fn hsv(&self) -> &HSV {
        &self.hsv
    }

    pub fn hex(&self) -> &Hex {
        &self.hex
    }
}
