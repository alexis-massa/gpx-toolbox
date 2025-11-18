use eframe::egui;
use std::path::PathBuf;
use walkdir::WalkDir;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to discover files from
    #[arg()]
    folder: PathBuf,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();

    let files: Vec<walkdir::DirEntry> = WalkDir::new(&args.folder)
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    let file_count: usize = files.len();
    let gpx_files: Vec<&walkdir::DirEntry> = files
        .iter()
        .filter(|&e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|s| s.eq_ignore_ascii_case("gpx"))
                .unwrap_or(false)
        })
        .collect();

    println!(
        "In '{}' there are {} files, including {} GPX.",
        args.folder.display(),
        file_count,
        gpx_files.len(),
    );

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GPX Toolbox",
        native_options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(cc)))),
    )
    // Ok(())
}

#[derive(Default)]
struct GuiApp {}

impl GuiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GPX Toolbox header");
        });
    }
}
