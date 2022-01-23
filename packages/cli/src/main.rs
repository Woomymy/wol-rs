mod args;
use args::Args;
mod errors;
use libwol_rs::{
    packet::{make_packet, send_packet},
    Mac,
};
#[macro_use]
mod log;
fn main() -> Result<(), errors::Error> {
    let args: Args = argh::from_env();

    if args.verbose {
        debug!(
            "wol-rs-{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }

    let dest_mac = args.mac.parse::<Mac>()?;
    if args.verbose {
        debug!("Dest Mac address: {:#?}", &dest_mac);
    }
    info!("Sending packet to host {}", args.mac);
    // Generate packet to send
    let packet = make_packet(&dest_mac)?;
    if args.verbose {
        debug!("Packet to send: {:#?}", &packet);
        debug!("Packet len: {}", packet.len())
    }
    // Broadcast packet
    send_packet(&packet[0..102], None)?;

    info!("Packet sent!");

    Ok(())
}
