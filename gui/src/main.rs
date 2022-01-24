use eframe::{
    egui::{CentralPanel, CtxRef},
    epi::{self, Frame},
    run_native,
};

use wolrs_common::{config::Config, error, errors::Error, host::wake_up_host, info};

struct WolApplication {
    config: Config,
    failed_hosts: Vec<String>,
    custom_host: String,
    old_custom_host: String,
    failed_custom_wake: bool,
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
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.custom_host);
                if ui.button("Wake up input host").clicked() {
                    info!("Waking up custom host with mac {}", &self.custom_host);
                    if let Err(error) =
                        wake_up_host(("input".to_string(), self.custom_host.clone()))
                    {
                        error!("Can't wake up {} {:#?}", &self.custom_host, error);
                        self.failed_custom_wake = true;
                    }
                    self.old_custom_host = self.custom_host.clone();
                }
                if self.failed_custom_wake && (self.custom_host == self.old_custom_host) {
                    ui.label("Failed!");
                }
            });
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
            custom_host: "".to_string(),
            old_custom_host: "".to_string(),
            failed_custom_wake: false,
        }),
        opts,
    );
}
