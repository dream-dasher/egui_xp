//! Very simple, native-only, egui implementation in a single file.

// ///////////////////////////////// -use- ///////////////////////////////// //
use eframe::egui;

// ///////////////////////////////// -main- ///////////////////////////////// //
fn main() {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(SimpleEguiApp::new(cc))))).unwrap();
}

// ///////////////////////////////// -App Memory- ///////////////////////////////// //
//                                     and init
#[derive(Default)]
struct SimpleEguiApp {
        text_one: String,
        text_two: String,
}
impl SimpleEguiApp {
        #[expect(unused)]
        fn new(cc: &eframe::CreationContext<'_>) -> Self {
                // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
                // Restore app state using cc.storage (requires the "persistence" feature).
                // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
                // for e.g. egui::PaintCallback.
                Self::default()
        }
}

// ///////////////////////////////// -Core Loop- ///////////////////////////////// //
impl eframe::App for SimpleEguiApp {
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
                        ui.heading("Simple Native Example App");
                        ui.label("text-one");
                        ui.text_edit_singleline(&mut self.text_one);
                        ui.separator();
                        ui.label("text-two");
                        ui.text_edit_multiline(&mut self.text_two);
                });
        }
}
