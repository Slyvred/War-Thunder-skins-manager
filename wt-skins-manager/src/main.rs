use std::fs;
use std::process::exit;

use crate::helpers::*;
use crate::installer::*;
use crate::scrapper::*;
use crate::uninstaller::*;

mod helpers;
mod installer;
mod scrapper;
mod uninstaller;

#[tokio::main]
async fn main() {
    let path = fs::read_to_string("game_directory.txt");
    if path.is_err() {
        println!("Please set the game directory first.");
        set_game_directory();
    }

    let mut choice = get_user_input("What would you like to do?\n\t1: Install a camouflage\n\t2: Uninstall a camouflage\n\t3: Set game directory\n\t4: Quit");
    while !["1", "2", "3", "4"].contains(&choice.as_str()) {
        println!("Please enter a number in the 1 to 4 range.");
        choice = get_user_input("What would you like to do?\n\t1: Install a camouflage\n\t2: Uninstall a camouflage\n\t3: Set game directory\n\t4: Quit");
    }

    match choice.as_str() {
        "1" => {
            let binding = get_camo_url();
            let url = binding.as_str();
            let mut scrapper = Scrapper::new(url);
            let camouflage = match scrapper.get_camouflage() {
                Some(camo) => camo,
                None => {
                    println!("No camo found.");
                    exit(1);
                }
            };
            match install_camouflage(&camouflage).await {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("{}", e),
            }
        }
        "2" => {
            uninstall_camo();
        }
        "3" => {
            set_game_directory();
        }
        "4" => {
            exit(0);
        }
        _ => {
            println!("Unknown error.");
            exit(1);
        }
    }
}
