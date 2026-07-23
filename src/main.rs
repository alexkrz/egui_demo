use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    age: u32,
    counter: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Welcome to egui!");

            ui.add(
                egui::Image::new(egui::include_image!("../data/ferris.png"))
                    .max_width(200.0)
                    .corner_radius(10),
            );

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Increment Counter").clicked() {
                self.counter += 1;
            }

            ui.label(format!("Counter: {}", self.counter));

            ui.separator();

            if !self.name.is_empty() {
                ui.label(format!(
                    "Hello, {}! You are {} years old.",
                    self.name, self.age
                ));
            }
        });
    }
}
