use eframe::egui;
use std::path::PathBuf;
use rfd::FileDialog;
use ureq;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    
    eframe::run_native(
        "Nimble GUI",
        native_options,
        Box::new(|_cc| Ok(Box::new(NimbleGui::default())))
    )
}

struct NimbleGui {
    // Sync command state
    sync_repo_url: String,
    sync_path: String,
    sync_dry_run: bool,

    // GenSrf command state
    gen_srf_path: String,

    // Launch command state
    launch_path: String,

    // Message display state
    message: Option<(String, bool)>, // (message, is_error)
}

impl Default for NimbleGui {
    fn default() -> Self {
        Self {
            sync_repo_url: String::new(),
            sync_path: String::new(),
            sync_dry_run: false,
            gen_srf_path: String::new(),
            launch_path: String::new(),
            message: None,
        }
    }
}

impl NimbleGui {
    fn show_message(&mut self, msg: String, is_error: bool) {
        self.message = Some((msg, is_error));
    }

    fn pick_folder(&mut self, path: &mut String) {
        if let Some(folder_path) = FileDialog::new().pick_folder() {
            *path = folder_path.display().to_string();
        }
    }
}

impl eframe::App for NimbleGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nimble GUI");

            // Sync Section
            ui.group(|ui| {
                ui.heading("Sync Command");
                ui.horizontal(|ui| {
                    ui.label("Repository URL:");
                    ui.text_edit_singleline(&mut self.sync_repo_url);
                });
                ui.horizontal(|ui| {
                    ui.label("Path:");
                    ui.text_edit_singleline(&mut self.sync_path);
                    if ui.button("Browse").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.sync_path = path.display().to_string();
                        }
                    }
                });
                ui.checkbox(&mut self.sync_dry_run, "Dry Run");
                if ui.button("Run Sync").clicked() {
                    let mut agent = ureq::agent();
                    if let Err(e) = nimble::commands::sync::sync(
                        &mut agent,
                        &self.sync_repo_url,
                        &PathBuf::from(&self.sync_path),
                        self.sync_dry_run,
                    ) {
                        eprintln!("Sync error: {:?}", e);
                        self.show_message(format!("Sync error: {:?}", e), true);
                    } else {
                        self.show_message("Sync completed successfully".to_string(), false);
                    }
                }
            });

            ui.add_space(10.0);

            // GenSrf Section
            ui.group(|ui| {
                ui.heading("Generate SRF Command");
                ui.horizontal(|ui| {
                    ui.label("Path:");
                    ui.text_edit_singleline(&mut self.gen_srf_path);
                    if ui.button("Browse").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.gen_srf_path = path.display().to_string();
                        }
                    }
                });
                if ui.button("Generate SRF").clicked() {
                    if let Err(e) = nimble::commands::gen_srf::gen_srf(&PathBuf::from(&self.gen_srf_path)) {
                        eprintln!("Gen SRF error: {:?}", e);
                        self.show_message(format!("Gen SRF error: {:?}", e), true);
                    } else {
                        self.show_message("SRF generated successfully".to_string(), false);
                    }
                }
            });

            ui.add_space(10.0);

            // Launch Section
            ui.group(|ui| {
                ui.heading("Launch Command");
                ui.horizontal(|ui| {
                    ui.label("Path:");
                    ui.text_edit_singleline(&mut self.launch_path);
                    if ui.button("Browse").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.launch_path = path.display().to_string();
                        }
                    }
                });
                if ui.button("Launch").clicked() {
                    if let Err(e) = nimble::commands::launch::launch(&PathBuf::from(&self.launch_path)) {
                        eprintln!("Launch error: {:?}", e);
                        self.show_message(format!("Launch error: {:?}", e), true);
                    } else {
                        self.show_message("Launch completed successfully".to_string(), false);
                    }
                }
            });

            ui.add_space(20.0);

            // Message display
            if let Some((msg, is_error)) = &self.message {
                let text = egui::RichText::new(msg)
                    .color(if *is_error {
                        egui::Color32::RED
                    } else {
                        egui::Color32::GREEN
                    });
                ui.label(text);
            }
        });
    }
}
