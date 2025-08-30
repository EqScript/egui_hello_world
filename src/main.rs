use eframe::egui;
use eframe::{self, App};

struct HelloWorld {
    show_label: bool,
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self { show_label: false }
    }
}

impl App for HelloWorld {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Optional: Cmd/Ctrl+Q to quit
        let input = ctx.input(|i| i.clone());
        if input.modifiers.command && input.key_pressed(egui::Key::Q) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Click!").clicked() {
                    self.show_label = !self.show_label;
                }

                if ui.button("Close").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            if self.show_label {
                ui.label("Hello, eGUI!");
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    use eframe::egui::{Vec2, ViewportBuilder};

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2::new(300.0, 200.0))
            .with_min_inner_size(Vec2::new(300.0, 200.0))
            .with_max_inner_size(Vec2::new(300.0, 200.0)),
        ..Default::default()
    };

    eframe::run_native(
        "eGUI Hello World App",
        options,
        Box::new(|_cc| Ok(Box::new(HelloWorld::default()))),
    )
}
