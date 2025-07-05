use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Tidy: A simple CLI file organizer
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Directory to organize
    #[arg(default_value = ".")]
    dir: String,

    /// Perform a dry run (just show what would be done)
    #[arg(short, long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let target_dir = args.dir;
    let dry_run = args.dry_run;

    let categories = get_categories();

    println!("Organizing files in: {}", target_dir);

    let entries = fs::read_dir(&target_dir).expect("Could not read directory");

    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();

        if path.is_file() {
            let extension = path.extension()
                                .and_then(|ext| ext.to_str())
                                .unwrap_or("")
                                .to_lowercase();

            let category = categories.get(extension.as_str()).unwrap_or(&"Others");

            let dest_dir = Path::new(&target_dir).join(category);
            let file_name = path.file_name().unwrap();
            let new_path = dest_dir.join(file_name);

            println!("Would move {:?} → {:?}", file_name, new_path);

            if !dry_run {
                if !dest_dir.exists() {
                    fs::create_dir_all(&dest_dir).expect("Could not create directory");
                }

                fs::rename(&path, &new_path).expect("Could not move file");
            }
        }
    }
}

fn get_categories() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("jpg", "Images");
    map.insert("png", "Images");
    map.insert("jpeg", "Images");
    map.insert("mp4", "Videos");
    map.insert("mov", "Videos");
    map.insert("pdf", "Documents");
    map.insert("docx", "Documents");
    map.insert("txt", "TextFiles");
    map.insert("zip", "Archives");
    map.insert("tar", "Archives");
    map.insert("gz", "Archives");
    map
}

