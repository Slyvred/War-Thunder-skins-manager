use std::fs;
use std::io::Write;

pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Error reading input");
    let choice = choice.trim().to_string();//.to_lowercase();

    choice
}

pub fn set_game_directory() {
    println!("Setting game directory...");
    let mut path = get_user_input("Please enter the game directory:\n\tExample: C:\\Users\\<your_username>\\AppData\\Local\\WarThunder\n");
    path = path.replace('\\', "/");
    path = path.trim_end_matches('/').to_string();

    if !path.contains("UserSkins") {
        path = format!("{}/UserSkins", path);
    }

    let mut file = fs::File::create("game_directory.txt").expect("Failed to create file");
    file.write_all(path.as_bytes())
        .expect("Failed to write to file");

    println!("Game directory set to: {}", path);
}
