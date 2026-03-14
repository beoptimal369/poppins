// src/main.rs

use std::env;
use std::process;

pub mod help;
pub mod train;
pub mod infer;
pub mod bootstrap;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        crate::help::help();
        process::exit(0);
    }

    match args[1].as_str() {
        "help" | "--help" | "-h" => crate::help::help(),
        "bootstrap" => crate::bootstrap::bootstrap(),
        "train" => crate::train::train(),
        "infer" => crate::infer::infer(),
        _ => {
            println!("❌ Unknown command: {}\n", args[1]);
            crate::help::help();
            process::exit(1);
        }
    }
}
