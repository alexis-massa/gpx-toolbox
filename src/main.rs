use eframe::egui::{self, Button};
use std::path::PathBuf;
use walkdir::WalkDir;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to discover files from
    #[arg(default_value = ".")]
    folder: PathBuf,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();

    let files: Vec<PathBuf> = WalkDir::new(&args.folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|e| {
            let path = e.path();
            let is_gpx = path
                .extension()
                .and_then(|x| x.to_str())
                .map(|s| s.eq_ignore_ascii_case("gpx"))
                .unwrap_or(false);

            if !is_gpx {
                return None;
            }

            path.strip_prefix(&args.folder)
                .ok()
                .map(|rel| rel.to_path_buf())
        })
        .collect();
    let file_count: usize = files.len();
    println!(
        "In '{}' there are {} files, including {} GPX.",
        args.folder.display(),
        file_count,
        files.len(),
    );

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GPX Toolbox",
        native_options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(files, args.folder, cc)))),
    )
    // Ok(())
}

struct GuiApp {
    files: Vec<PathBuf>,
    folder: PathBuf,
    selected_index: Option<usize>,
}

impl GuiApp {
    fn new(files: Vec<PathBuf>, folder: PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            files,
            folder,
            selected_index: None,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("GPX Toolbox").heading().underline())
            });

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
    }
}
