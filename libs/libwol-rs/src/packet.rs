use super::Mac;
use std::net::SocketAddr;

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

pub fn send_packet(packet: &[u8], address: Option<SocketAddr>) -> Result<(), std::io::Error> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let addr;
    if let Some(a) = address {
        addr = a;
    } else {
        addr = "255.255.255.255:9"
            .parse::<SocketAddr>()
            .expect("Can't parse destination socketaddr!");
    }

    socket.send_to(packet, addr)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use crate::packet::{make_packet, send_packet};

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

    #[test]
    /// Tests send_packet by sending packet to loopback
    pub fn test_send_packet() {
        let packet = make_packet(&super::Mac::new((0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF))).unwrap();
        assert!(send_packet(
            &packet[0..102],
            Some("127.0.0.1:9".parse::<SocketAddr>().unwrap())
        )
        .is_ok())
    }
}
