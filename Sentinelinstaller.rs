// Sentinelinstaller.rs
//
// Run this inside an empty project folder.
// It downloads the official Sentinel Cargo.toml and src/ folder
// from the GitHub repository and installs them locally.

use std::fs;
use std::io::Cursor;
use std::path::Path;
use reqwest::blocking::get;
use zip::ZipArchive;

fn download_file(url: &str, path: &str) -> Result<(), String> {
    let response = get(url).map_err(|e| e.to_string())?;
    let bytes = response.bytes().map_err(|e| e.to_string())?;
    fs::write(path, &bytes).map_err(|e| e.to_string())?;
    Ok(())
}

fn download_and_extract_zip(url: &str, dest: &str) -> Result<(), String> {
    let response = get(url).map_err(|e| e.to_string())?;
    let bytes = response.bytes().map_err(|e| e.to_string())?;

    let reader = Cursor::new(bytes);
    let mut zip = ZipArchive::new(reader).map_err(|e| e.to_string())?;

    fs::create_dir_all(dest).map_err(|e| e.to_string())?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(|e| e.to_string())?;
        let outpath = Path::new(dest).join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn main() {
    println!("Installing Sentinel framework...");

    // URLs
    let cargo_url = "https://github.com/Rudycon55555/Sentinel/raw/refs/heads/main/Cargo.toml";
    let src_zip_url = "https://github.com/Rudycon55555/Sentinel/archive/refs/heads/main.zip";

    // Download Cargo.toml
    println!("Downloading Cargo.toml...");
    if let Err(e) = download_file(cargo_url, "Cargo.toml") {
        eprintln!("Failed to download Cargo.toml: {}", e);
        return;
    }

    // Download and extract src/
    println!("Downloading and extracting src/ folder...");
    if let Err(e) = download_and_extract_zip(src_zip_url, ".") {
        eprintln!("Failed to download src/: {}", e);
        return;
    }

    // Move extracted folder into place
    let extracted = Path::new("Sentinel-main/src");
    let target = Path::new("src");

    if extracted.exists() {
        println!("Installing src/...");
        if target.exists() {
            fs::remove_dir_all(target).unwrap();
        }
        fs::rename(extracted, target).unwrap();
        fs::remove_dir_all("Sentinel-main").unwrap();
    } else {
        eprintln!("Could not find extracted src/ folder.");
        return;
    }

    println!("Sentinel installation complete!");
    println!("You can now run: cargo build");
}
