use std::{fs, path::Path};

use crate::{fractal_key::FractalKey, fractal_stream::FractalStream};

pub struct Encoder {
    key: FractalKey,
}

impl Encoder {
    pub fn new(key: FractalKey) -> Self {
        Self { key }
    }

    pub fn encode(&self, file_path: String) -> bool {
        let data: Vec<u8> = match fs::read(&file_path) {
            Ok(d) => d,
            Err(e) => panic!("Failed to read file {}: {}", file_path, e),
        };

        let mut out_file = file_path.clone();
        out_file.push_str(".frc");

        let mut frac_stream = FractalStream::new(&self.key);
        let mut frac_chunk = vec![];
        let mut output_data = Vec::with_capacity(data.len());

        for byte in data {
            if frac_chunk.is_empty() {
                frac_chunk = frac_stream.next_chunk();
            }

            let frac = frac_chunk.pop().unwrap();

            output_data.push(frac ^ byte);
        }

        fs::write(out_file, output_data);

        return true;
    }
}

pub struct Decoder {
    key: FractalKey,
}

impl Decoder {
    pub fn new(key: FractalKey) -> Self {
        Self { key }
    }

    pub fn decode(&self, file_path: String) -> bool {
        let data: Vec<u8> = match fs::read(&file_path) {
            Ok(d) => d,
            Err(e) => panic!("Failed to read file {}: {}", file_path, e),
        };

        let orig_filepath = file_path.replace(".frc", "");
        let input_path = Path::new(&orig_filepath);
        let mut dir = input_path.parent().unwrap().to_str().unwrap();
        if dir.is_empty() {
            dir = ".";
        }
        let orig_filename = input_path.file_stem();
        let out_file = format!(
            "{}/{}-2.{}",
            dir,
            orig_filename.unwrap().to_str().unwrap(),
            input_path.extension().unwrap().to_str().unwrap()
        );
        println!("Writing to {}", out_file);

        let mut frac_stream = FractalStream::new(&self.key);
        let mut frac_chunk = vec![];
        let mut output_data = Vec::with_capacity(data.len());

        for byte in data {
            if frac_chunk.is_empty() {
                frac_chunk = frac_stream.next_chunk();
            }

            let frac = frac_chunk.pop().unwrap();

            output_data.push(frac ^ byte);
        }

        if let Err(e) = fs::write(out_file, output_data) {
            println!("Failed to write decoded file: {}", e);
            return false;
        }

        true
    }
}
