use std::{str::FromStr, string::ParseError};

#[derive(Debug, PartialEq)]
pub enum MacParseError {
    InvalidInput,
    InvalidLenght,
}

pub mod packet;
#[derive(Debug, PartialEq)]
pub struct Mac(u8, u8, u8, u8, u8, u8);

impl Mac {
    pub fn new(mac: (u8, u8, u8, u8, u8, u8)) -> Self {
        Self(mac.0, mac.1, mac.2, mac.3, mac.4, mac.5)
    }
    pub fn as_bytes(&self) -> [u8; 6] {
        [self.0, self.1, self.2, self.3, self.4, self.5]
    }
}

impl FromStr for Mac {
    type Err = MacParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .split(':')
            .map(|b| u8::from_str_radix(b, 16))
            .collect::<Result<Vec<_>, _>>();
        match bytes {
            Ok(mac) => {
                if mac.len() == 6 {
                    Ok(Mac::new((mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])))
                } else {
                    Err(MacParseError::InvalidLenght)
                }
            }
            Err(_) => Err(MacParseError::InvalidInput),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Mac;
    #[test]
    /// Tests Mac::new()
    pub fn test_mac_new() {
        assert_eq!(
            Mac::new((0xFF, 0xFF, 0xEF, 0xAF, 0xFF, 0xFF)),
            Mac(0xFF, 0xFF, 0xEF, 0xAF, 0xFF, 0xFF)
        )
    }
    #[test]
    /// Tests mac.as_bytes()
    pub fn test_mac_as_bytes() {
        assert_eq!(
            Mac::new((0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF)).as_bytes(),
            [0xFF; 6]
        )
    }
}
