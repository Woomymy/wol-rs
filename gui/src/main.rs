use eframe::{
    egui::{CentralPanel, CtxRef},
    epi::{self, Frame},
    run_native,
};

use wolrs_common::{config::Config, error, errors::Error, host::wake_up_host};

struct WolApplication {
    config: Config,
    failed_hosts: Vec<String>,
}

impl epi::App for WolApplication {
    fn name(&self) -> &str {
        "Wol GUI"
    }

    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wake-on-lan application");

            for host in self.config.get_hosts() {
                if self.failed_hosts.contains(&host.0) {
                    ui.label(format!("{} ({}): FAILED", &host.0, &host.1));
                } else {
                    ui.horizontal(|hui| {
                        hui.label(format!("{} ({})", host.0, host.1));
                        if hui.button("Wake up host").clicked() {
                            if let Err(error) = wake_up_host(host.clone()) {
                                error!("Can't wake up {} {} {:#?}", &host.0, &host.1, error);
                                self.failed_hosts.push(host.0.clone());
                            }
                        }
                    });
                }
            }
        });

        frame.set_window_size(ctx.used_size());
    }
}
fn main() -> Result<(), Error> {
    let opts = eframe::NativeOptions::default();
    let config = Config::from_machine()?;
    run_native(
        Box::new(WolApplication {
            config,
            failed_hosts: Vec::new(),
        }),
        opts,
    );
}
