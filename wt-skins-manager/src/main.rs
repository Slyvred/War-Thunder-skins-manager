use std::process::exit;

use crate::scrapper::*;
use crate::installer::*;
use crate::uninstaller::*;
mod scrapper;
mod installer;
mod uninstaller;

#[tokio::main]
async fn main() {

    let mut choice = get_user_input("What would you like to do?\n\t1: Install a camouflage\n\t2: Uninstall a camouflage\n\t3: Quit");
    while choice != "1" && choice != "2" && choice != "3" {
        println!("Please enter 1, 2, or 3.");
        choice = get_user_input("What would you like to do?\n\t1: Install a camouflage\n\t2: Uninstall a camouflage\n\t3: Quit");
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
            exit(0);
        }
        _ => {
            println!("Unknown error.");
            exit(1);
        }
    }
}
