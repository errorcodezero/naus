mod cli;
mod emu;
mod parser;
mod symbols;

use clap::Parser;
use cli::Args;
use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Ok(mut file) = File::open(&args.file) {
        let mut source = String::new();

        if file.read_to_string(&mut source).is_err() {
            eprintln!("Error reading file {}", &args.file);
            return Ok(());
        }
    } else {
        eprintln!("File doesn't exist.")
    }

    Ok(())
}
