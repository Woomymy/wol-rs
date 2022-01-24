use super::{debug, errors::Error, info};
use libwol_rs::{
    packet::{make_packet, send_packet},
    Mac,
};
pub fn wake_up_host(host: (String, String)) -> Result<(), Error> {
    let destination_mac = host.1.parse::<Mac>()?;
    // Prepare the magic packet
    let packet = make_packet(&destination_mac)?;
    debug!("Sending magic packet: {:#?}", &packet);

    info!("Sending wake-up packet to {}", &host.1);
    send_packet(&packet[0..102], None)?;
    Ok(())
}
