use std::fs;
use std::path::Path;

fn main() {
    let dir = r"C:\Users\CraigParker\OneDrive - Wits PHR\PHR PC\WRHI Files\Documents"; // Use raw string for Windows path
    let paths = match fs::read_dir(dir) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    for path in paths {
        let path = match path {
            Ok(p) => p.path(),
            Err(e) => {
                eprintln!("Error reading path: {}", e);
                continue;
            }
        };
        if path.is_file() {
            match fs::metadata(&path) {
                Ok(metadata) => {
                    if metadata.len() == 0 {
                        if let Err(e) = fs::remove_file(&path) {
                            eprintln!("Error deleting file {}: {}", path.display(), e);
                        } else {
                            println!("Deleted empty file: {}", path.display());
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error reading metadata for {}: {}", path.display(), e);
                }
            }
        }
    }

    println!("Empty file cleanup completed!");
}
