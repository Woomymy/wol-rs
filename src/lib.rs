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
