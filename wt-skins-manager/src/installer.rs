// use chrono;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::helpers::get_user_input;
use crate::scrapper::Camouflage;

pub async fn download_file(url: &str, path: &str) -> Result<String, String> {
    let response = reqwest::get(url)
        .await
        .expect("Download failed, please check your connection.");

    if response.status().is_success() {
        let mut file = fs::File::create(path).expect("Failed to create file");
        let content = response
            .bytes()
            .await
            .expect("Failed to read response content");
        file.write_all(&content).expect("Failed to write to file");
        Ok(path.to_string())
    } else {
        Err("Error downloading file".to_string())
    }
}

pub async fn install_camouflage(camouflage: &Camouflage) -> Result<String, String> {
    println!("\n{}\n", camouflage);

    let mut choice = get_user_input("Would you like to install the camouflage (Yes/No):");

    while choice != "yes" && choice != "no" {
        println!("Please enter \"Yes\" or \"No\".");
        choice = get_user_input("Would you like to install the camouflage (Yes/No):");
    }

    if choice == "no" {
        return Ok("Installation cancelled.".to_string());
    }

    let mut camo_name = "camouflage.zip".to_string();
    let choice = get_user_input(
        "How would you like to name the camouflage folder (leave empty for default):",
    );
    if !choice.is_empty() {
        camo_name = format!("{}.zip", choice);
    }

    // Télécharger le fichier
    println!("Downloading the camouflage...");

    match download_file(&camouflage.download, camo_name.as_str()).await {
        Ok(_) => println!("Camouflage downloaded successfully."),
        Err(e) => {
            println!("{}", e);
            return Err("Error while downloading the camouflage".to_string());
        }
    }

    let mut path = match fs::read_to_string("game_directory.txt") {
        Ok(path) => path,
        Err(_) => {
            return Err("Error reading the game directory".to_string());
        }
    };

    println!("Installing the camouflage...");
    if !Path::new(&path).exists() {
        match fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(_) => {
                return Err("Error creating the camouflage folder".to_string());
            }
        }
    }

    println!("Extracting the archive...");
    let file = match fs::File::open(camo_name.as_str()) {
        Ok(file) => file,
        Err(_) => {
            return Err("Failed to open the archive".to_string());
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(_) => {
            return Err("Failed to open the archive".to_string());
        }
    };

    if camo_name != "camouflage.zip" {
        let tmp = format!("/{}", &camo_name.replace(".zip", ""));
        path.push_str(&tmp);
    }

    match archive.extract(&path) {
        Ok(_) => (),
        Err(_) => {
            return Err("Error during extraction".to_string());
        }
    }

    // Supprimer le fichier zip
    match fs::remove_file(&camo_name) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Failed to delete the archive, please delete it manually.");
        }
    }

    Ok(format!("Camouflage successfully installed in {}", path))
}
