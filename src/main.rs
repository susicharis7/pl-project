mod game;
mod players;
mod display;
mod io;
mod scoreboard;
mod menu;
mod utils;

use menu::main_menu::run_main_menu;

fn main() {
    let _ = std::fs::create_dir_all("saves");   
    run_main_menu();
}

