use std::fs;

use cryptography::{Decoder, Encoder};
use fractal_key::FractalKey;

mod complex;
mod cryptography;
mod fractal_key;
mod fractal_stream;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    key: Option<String>,

    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    decode: bool,
}

fn main() {
    let args = Args::parse();

    let key = match args.key {
        Some(key_path) => {
            let data = fs::read(key_path);
            if let Err(e) = data {
                panic!("Failed to read key: {}", e)
            }

            FractalKey::from_u8(data.unwrap())
        }
        None => {
            // make a key and save it
            let key = FractalKey::new();
            key.save("frac_key.frk");

            key
        }
    };

    if args.decode {
        let decoder = Decoder::new(key);
        decoder.decode(args.file);
    } else {
        let encoder = Encoder::new(key);
        encoder.encode(args.file);
    }
}
