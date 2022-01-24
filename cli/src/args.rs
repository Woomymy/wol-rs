use argh::FromArgs;

#[derive(FromArgs)]
/// CLI args
pub struct Args {
    #[argh(switch, short = 'v')]
    /// enable verbose logging
    pub verbose: bool,
    /// mac address of host to wake up
    #[argh(option, short = 'm')]
    pub mac: Option<String>,
    /// host to wake up
    #[argh(option, short = 'h')]
    pub host: Option<String>,
}
