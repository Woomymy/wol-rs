mod args;
use args::Args;
mod errors;
use libwol_rs::{
    packet::{make_packet, send_packet},
    Mac,
};
fn main() -> Result<(), errors::Error> {
    let args: Args = argh::from_env();

    if args.verbose {
        println!(
            "wol-rs-{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    let dest_mac = args.mac.parse::<Mac>()?;
    if args.verbose {
        println!("Dest Mac address: {:#?}", &dest_mac);
    }
    println!("Sending packet to host {}", args.mac);
    // Generate packet to send
    let packet = make_packet(&dest_mac)?;
    if args.verbose {
        println!("Packet to send: {:#?}", &packet);
        println!("Packet len: {}", packet.len())
    }
    // Broadcast packet
    send_packet(&packet[0..102], None)?;

    println!("Packet sent!");

    Ok(())
}
