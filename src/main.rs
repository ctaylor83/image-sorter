use std::env;
use std::fs;
use walkdir::{DirEntry, WalkDir};
use regex::Regex;
use dialoguer::{theme::ColorfulTheme, Input};

fn is_image_or_video(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| {
            let ext = ext.to_lowercase();
            ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif" || ext == "bmp"
                || ext == "mp4" || ext == "avi" || ext == "mkv" || ext == "flv" || ext == "mov"
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        return;
    }

    let path = &args[1];
    let re = Regex::new(r"[\s]+").unwrap();
    let input_title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a title for the images and videos")
        .interact_text()
        .unwrap();

    let title = re.replace_all(&input_title, "_").to_string();
    let mut counter = 1;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(is_image_or_video)
    {
        let ext = entry.path().extension().unwrap().to_str().unwrap();
        let new_name = format!("{}_{:04}.{}", title, counter, ext);

        let new_path = entry.path().with_file_name(new_name);
        match fs::rename(entry.path(), &new_path) {
            Ok(_) => {
                println!("Renamed: {} -> {}", entry.path().display(), new_path.display());
                counter += 1;
            }
            Err(err) => {
                eprintln!("Error renaming file: {:?}", err);
            }
        }
    }

    println!("Processed {} files.", counter - 1);
}