mod gui;
mod filetree;

use std::path::PathBuf;
use eframe::egui;
use walkdir::WalkDir;

use clap::Parser;

use crate::gui::GuiApp;
use crate::filetree::{FileTree, Node};

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
    let file_tree = FileTree::from_file_list(&args.folder, &files);
    println!("{}", file_tree.is_empty());

    println!("{}", file_tree.root.name());

    println!(
        "In '{}' there are {} files, including {} GPX.",
        args.folder.display(),
        file_count,
        files.len(),
    );

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "GPX Toolbox",
        native_options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(file_tree, args.folder, cc)))),
    )
    // Ok(())
}
