use chrono;
use eframe::egui;
use std::env;
use std::env::current_dir;
use std::fs::{create_dir_all, remove_file, rename};
use std::path::PathBuf;
use std::sync::Arc;
fn main() -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../icon.png"))
        .expect("The icon data must be valid");

    // args[1] = shaderfile (that bonzomatic use on the postExitCmd)
    // args[2] = default save directory
    let args: Vec<String> = env::args().collect();

    let save_directory = args
        .get(2)
        .map(|x| x.to_owned())
        .unwrap_or("shaders".to_owned());
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 200.0])
            .with_icon(Arc::new(icon))
            .with_resizable(false)
            .with_window_level(egui::WindowLevel::AlwaysOnTop),
        ..Default::default()
    };
    eframe::run_native(
        "Bonzomatic Onquit",
        options,
        Box::new(|_cc| {
            Ok(Box::new(BonzomaticOnquitApp::new(
                args[1].clone(),
                save_directory.clone(),
            )))
        }),
    )
}
struct BonzomaticOnquitApp {
    shader_file: String,
    save_directory: String,
}
impl BonzomaticOnquitApp {
    fn new(shader_file: String, save_directory: String) -> Self {
        Self {
            shader_file: shader_file,
            save_directory: save_directory,
        }
    }
}
impl eframe::App for BonzomaticOnquitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.heading("Bonzomatic On Quit");
            let mut clicked: bool = false;
            if ui.button("Do Nothing").clicked() {
                clicked = true;
            }

            if ui.button("Delete Shader").clicked() {
                match remove_file(&self.shader_file) {
                    Ok(()) => eprintln!("File Deleted"),
                    Err(e) => eprintln!("File couldn't be deleted : {e}"),
                };
                clicked = true;
            }
            if ui.button("Save Shader to File").clicked() {
                let mut target = PathBuf::from(current_dir().unwrap());
                target.push(&self.shader_file);
                match tinyfiledialogs::save_file_dialog("Save", target.to_str().unwrap()) {
                    Some(file) => match rename(&self.shader_file, &file) {
                        Ok(()) => eprintln!("Saved shjader to {file}"),
                        Err(e) => eprintln!(
                            "Couldn't save to selected file, quitting doing nothing : {e}"
                        ),
                    },
                    None => eprintln!("Couldn't save to selected file, quitting doing nothing"),
                }

                clicked = true;
            }
            if ui.button("Move Shader to Directory").clicked() {
                let now = chrono::offset::Local::now();
                match create_dir_all(&self.save_directory) {
                    Ok(()) => {
                        let mut target = PathBuf::from(&self.save_directory);
                        target.push(format!("{}_{}", now.timestamp(), self.shader_file));
                        match rename(&self.shader_file, target.as_os_str()) {
                            Ok(()) => eprint!("Moved file to {:?}", target),
                            Err(e) => eprint!(
                                "Can't move file to {:?}, quitting doing nothing : {e}",
                                target
                            ),
                        }
                    }
                    Err(e) => eprintln!(
                        "{} doesn't exists and can't create, quitting doing nothing : {e}",
                        self.save_directory
                    ),
                }
                clicked = true;
            }
            if clicked {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
