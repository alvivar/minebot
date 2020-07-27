
use std::fs;
use std::process::Command;
use regex::Regex;

fn main() {
    println!("This is happening, right?");

    let paths = fs::read_dir("./data/logs").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    // Reading the log

    // let data = fs::read_to_string("/c/adros/code/soft-thunder/minecraft/data/logs/latest.log").expect("Unable to read file");
    let data = fs::read_to_string("./data/logs/latest.log").expect("Unable to read file");
    let files: Vec<&str> = data.split("\n").collect();

    let len = files.len();
    let last_index = 10;
    let tail = len - last_index;

    for i in tail..len {
        println!("{}", files[i]);
    }

    // Commands

    let output =
        Command::new("docker")
                // .args(&["ps", "-a"])
                .args(&["exec", "-i"])
                .args(&["mc", "rcon-cli"])
                .args(&["give", "adronomicon", "golden_apple"])
                .status()
                .expect("Failed to execute process");

    println!("Result: {}", output);
}
