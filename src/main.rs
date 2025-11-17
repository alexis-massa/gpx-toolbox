use std::{error::Error, path::PathBuf};
use walkdir::WalkDir;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to discover files from
    #[arg()]
    folder: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_count: usize = WalkDir::new(&args.folder)
        .into_iter()
        .collect::<Vec<Result<walkdir::DirEntry, walkdir::Error>>>()
        .into_iter()
        .filter(|dir| dir.is_ok())
        .collect::<Vec<Result<walkdir::DirEntry, walkdir::Error>>>()
        .len();
    println!("{} files in {}!", file_count, args.folder.display());
    Ok(())
}
