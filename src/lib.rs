#[derive(Clone, Debug, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Colour {
    _rgb: RGB,
    _hex: String,
}

impl Colour {
    pub fn from_hex(hex: &str) -> Colour {
        // if there is a leading #, remove it
        let hex = if hex.starts_with('#') {
            &hex[1..]
        } else {
            hex
        };

        Colour {
            _rgb: RGB { 
                r: u8::from_str_radix(&hex[0..2], 16).unwrap(),
                g: u8::from_str_radix(&hex[2..4], 16).unwrap(),
                b: u8::from_str_radix(&hex[4..6], 16).unwrap(),
            },
            _hex: String::from(hex),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            _rgb: RGB { r, g, b },
            _hex: format!("{:02x}{:02x}{:02x}", r, g, b),
        }
    }

    pub fn to_hex(&self) -> String {
        self._hex.clone()
    }

    pub fn to_rgb(&self) -> RGB {
        self._rgb.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_works() {
        let colour = Colour::from_hex("ff00ff");
        assert_eq!(colour.to_hex(), "ff00ff");
        assert_eq!(colour.to_rgb(), RGB { r: 255, g: 0, b: 255 });
    }

    #[test]
    fn from_hex_with_hashtag_works() {
        let colour = Colour::from_hex("#ff00ff");
        assert_eq!(colour.to_hex(), "ff00ff");
        assert_eq!(colour.to_rgb(), RGB { r: 255, g: 0, b: 255 });
    }
}
