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

    // Télécharger le fichier
    println!("Downloading the camouflage...");
    let path = "camouflage.zip";

    match download_file(&camouflage.download, path).await {
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
    let file = match fs::File::open("camouflage.zip") {
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

    // Si l'archive contient plusieurs fichiers, créer un dossier pour les contenir
    if archive.len() != 1 {
        let folder_name = format!("{}_{}", camouflage.author, chrono::Local::now().timestamp());
        path.push_str(&format!("/{}", folder_name));
    }

    match archive.extract(&path) {
        Ok(_) => (),
        Err(_) => {
            return Err("Error during extraction".to_string());
        }
    }

    // Supprimer le fichier zip
    match fs::remove_file("camouflage.zip") {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Failed to delete the archive, please delete it manually.");
        }
    }

    Ok(format!("Camouflage successfully installed in {}", path))
}
