mod cli;
mod parser;
mod symbols;

use crate::symbols::get_audio_file;
use clap::Parser;
use cli::Args;
use parser::Parser as EmuParser;
use rodio::Decoder;
use std::fs::File;
use std::{error::Error, io::Read};
use symbols::Symbols;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let args = Args::parse();

    if let Ok(mut file) = File::open(&args.file) {
        let mut source = String::new();

        if file.read_to_string(&mut source).is_err() {
            eprintln!("Error reading file {}", &args.file);
            return Ok(());
        }

        let parser = EmuParser { source };
        let symbols: Vec<Symbols> = parser.parse();

        for symbol in symbols {
            let file = File::open(format!("audio/{}", get_audio_file(symbol))).unwrap();
            let source = Decoder::try_from(file).unwrap();
            stream_handle.mixer().add(source);
            std::thread::sleep(std::time::Duration::from_secs_f32(1.1));
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    } else {
        eprintln!("File doesn't exist.")
    }

    Ok(())
}
