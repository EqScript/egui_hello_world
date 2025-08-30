use eframe::egui;

struct HelloWorld {
    show_label: bool,
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self { show_label: false }
    }
}


impl eframe::App for HelloWorld {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Click!").clicked() {
                    self.show_label = !self.show_label;
                }

                if ui.button("Close").clicked() {
                    // Tell the native window to close
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            if self.show_label {
                ui.label("Hello, eGUI!");
            }
        });
    }
}

fn main() -> eframe::Result <()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "eGUI Hello World App",
        options,
        Box::new( |_cc| Box::new( HelloWorld::default() ) ),
    )
}
