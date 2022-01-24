mod args;
use args::Args;
mod errors;
mod config;
use config::Config;
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

    let config = Config::from_machine()?;

    debug!("{:#?}", config);

    let mac: String;
    if let Some(macaddr) = args.mac {
        mac = macaddr;
    } else if let Some(host) = args.host {
        debug!("Host to wake: {host}");
        if let Some(host) = config.find_host(host.clone()) {
            mac = host.1;
        } else {
            error!("Host {host} not found!");
            std::process::exit(0);
        }
    } else {
        error!("No host/mac specified!");
        std::process::exit(0);
    }

    let dest_mac = mac.parse::<Mac>()?;
    if args.verbose {
        debug!("Dest Mac address: {:#?}", &dest_mac);
    }
    info!("Sending packet to host {}", &mac);
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