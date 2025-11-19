use std::path::PathBuf;

use eframe::egui::{self, Button};


pub struct GuiApp {
    files: Vec<PathBuf>,
    folder: PathBuf,
    selected_index: Option<usize>,
}

impl GuiApp {
    pub fn new(files: Vec<PathBuf>, folder: PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            files,
            folder,
            selected_index: None,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("GPX Toolbox").heading().underline())
            });
        });
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label(egui::RichText::new(format!("Files in {}", &self.folder.display())).strong());

            if self.files.is_empty() {
                ui.label("No GPX files found.");
            } else {
                for (i, file) in self.files.iter().enumerate() {
                    let selected = self.selected_index == Some(i);
                    let file_name = file.display().to_string();

                    if ui
                        .add(Button::selectable(selected, file_name).frame(selected))
                        .clicked()
                    {
                        if selected {
                            self.selected_index = None
                        } else {
                            self.selected_index = Some(i)
                        }
                    };
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Main zone 2").heading().underline())
            });
        });
    }
}
