pub mod packet;
#[derive(Debug)]
pub struct Mac(u8, u8, u8, u8, u8, u8);

impl Mac {
    pub fn new(mac: (u8, u8, u8, u8, u8, u8)) -> Self {
        Self(mac.0, mac.1, mac.2, mac.3, mac.4, mac.5)
    }
    pub fn as_bytes(&self) -> [u8; 6] {
        [self.0, self.1, self.2, self.3, self.4, self.5]
    }
}
