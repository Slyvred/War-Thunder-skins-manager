use std::env::var;
use std::fs;
use std::path::Path;

use crate::installer::get_user_input;

pub fn get_camo_path() -> Option<String> {
    let user = match var("USERNAME") {
        Ok(user) => user,
        Err(_) => {
            println!("Error retrieving username.");
            return None;
        }
    };

    let path = format!("C:/Users/{}/AppData/Local/WarThunder/UserSkins", user);
    Some(path)
}

pub fn list_installed_camo() {
    let path = match get_camo_path() {
        Some(path) => path,
        None => {
            println!("Error retrieving the camouflages path.");
            return;
        }
    };

    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Error reading the folder.");
            return;
        }
    };

    println!("Installed camouflages:");
    let mut i = 1;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let path = path.to_str().unwrap();
            let path = path.split("\\").last().unwrap();
            println!("\t({}) {}", i, path);
            i += 1;
        }
    }
}

pub fn get_camo_path_from_index(index: usize) -> Option<String> {
    let path = match get_camo_path() {
        Some(path) => path,
        None => {
            println!("Error retrieving the camouflages path.");
            return None;
        }
    };

    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Error reading the folder.");
            return None;
        }
    };

    let mut i = 1;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            if i == index {
                return Some(path.to_str().unwrap().to_string());
            }
            i += 1;
        }
    }

    None
}

pub fn uninstall_camo() {
    list_installed_camo();
    let choice = get_user_input("Which camouflage do you want to uninstall?");

    let path = match get_camo_path_from_index(choice.parse().unwrap()) {
        Some(path) => path,
        None => {
            println!("Error retrieving the camouflage path.");
            return;
        }
    };

    if Path::new(&path).exists() == false {
        println!("The camouflage does not exist.");
        return;
    }

    // Ask for confirmation
    println!("Selected camouflage: {}", path.split("\\").last().unwrap());
    let mut choice = get_user_input("Do you want to uninstall the camouflage (Yes/No):");

    while choice != "yes" && choice != "no" {
        println!("Please enter \"Yes\" or \"No\".");
        choice = get_user_input("Do you want to uninstall the camouflage (Yes/No):");
    }

    if choice == "no" {
        println!("Uninstallation cancelled.");
        return;
    }

    match fs::remove_dir_all(path) {
        Ok(_) => (),
        Err(_) => {
            println!("Error uninstalling the camouflage.");
            return;
        }
    }

    println!("The camouflage was successfully uninstalled.");
}
