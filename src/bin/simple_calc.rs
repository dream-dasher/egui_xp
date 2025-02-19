use eframe::egui;

fn main() {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))).unwrap();
}

#[derive(Default)]
struct MyEguiApp {
        left:       Option<i32>,
        right:      Option<i32>,
        out:        Option<i32>,
        left_text:  String,
        right_text: String,
}

impl MyEguiApp {
        #[expect(unused)]
        fn new(cc: &eframe::CreationContext<'_>) -> Self {
                // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
                // Restore app state using cc.storage (requires the "persistence" feature).
                // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
                // for e.g. egui::PaintCallback.
                Self::default()
        }
}

impl eframe::App for MyEguiApp {
        #[expect(unused)]
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                // -- Menu Bar --
                egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                        egui::menu::bar(ui, |ui| {
                                ui.menu_button("File", |ui| {
                                        if ui.button("Quit").clicked() {
                                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                        }
                                });
                                ui.add_space(16.0);
                                egui::widgets::global_theme_preference_buttons(ui);
                        });
                });

                // -- Input and Results Strip --
                egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Number Calc.");
                        ui.horizontal(|ui| {
                                ui.label("Left number:");
                                if ui.text_edit_singleline(&mut self.left_text).changed() {
                                        self.left = self.left_text.parse().ok();
                                }
                                ui.add_space(8.0);
                                ui.label("Right number:");
                                if ui.text_edit_singleline(&mut self.right_text).changed() {
                                        self.right = self.right_text.parse().ok();
                                }
                                ui.add_space(8.0);
                        });
                        // Add buttons for operations
                        ui.horizontal(|ui| {
                                if let (Some(left), Some(right)) = (self.left, self.right) {
                                        if ui.button("Add").clicked() {
                                                self.out = Some(left + right);
                                        }
                                        if ui.button("Subtract").clicked() {
                                                self.out = Some(left - right);
                                        }
                                        if ui.button("Multiply").clicked() {
                                                self.out = Some(left * right);
                                        }
                                        if ui.button("Divide").clicked() && self.right.unwrap() != 0 {
                                                self.out = Some(left / right);
                                        }
                                } else {
                                        ui.label("Enter two integer numbers for operation options.");
                                        self.out = None;
                                }
                        });

                        ui.add_space(8.0);

                        // Display result
                        if let Some(result) = self.out {
                                ui.label(format!("Result: {}", result));
                        }
                });
        }
}
