use std::{env, fs};

use todomd::Board;

fn dothing() {
    let default_todo;

    if let Ok(dir) = env::var("ORIGINAL_PWD") {
        env::set_current_dir(&dir).unwrap();
        default_todo = String::from("TODO.md");
    } else {
        default_todo = String::from("TODO.md");
    }

    let file = env::args().nth(1).unwrap_or(default_todo);
    let markdown = fs::read_to_string(file).unwrap();

    let board = Board::parse(&markdown);

    println!("");

    board.print(&markdown, true);
}

fn main() {
    dothing();
}
