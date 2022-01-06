use super::Mac;
pub fn make_packet(mac: &Mac) -> Result<Vec<u8>, std::io::Error> {
    let macbytes = mac.as_bytes();
    let mut packet = vec![0xFF; 6];
    if macbytes.len() != 6 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid mac!",
        ));
    }

    // Append input MAC address 16 times
    for _ in 0..=15 {
        packet.extend_from_slice(&macbytes);
    }

    Ok(packet)
}

#[cfg(test)]
mod tests {
    use crate::packet::make_packet;

    #[test]
    pub fn test_make_packet() {
        let mac = super::Mac::new((0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF));
        // Test if we can build packet
        assert!(make_packet(&mac).is_ok());
        // Ensure packet len is 102 bytes
        assert_eq!(make_packet(&mac).unwrap().len(), 102);
        // Test if packet correctly constructed
        assert_eq!(make_packet(&mac).unwrap(), vec![0xFF; 102]);
    }
}
