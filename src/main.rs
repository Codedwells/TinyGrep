use std::{env, process};

use mini_grep::Config;

fn main() {
    println!("Welcome to mini-grep: allowed args query_name file_name");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing : {}", err);
        process::exit(1)
    });

    match mini_grep::search_text(&config) {
        Ok(matched) => {
            println!("Matched lines:");
            for line in &matched {
                println!("{}", line)
            }
        }
        Err(err) => eprintln!("{}", err),
    };
}
