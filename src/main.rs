use anyhow::{Ok, Result};
use base64::engine;
use base64::read::DecoderReader;
use clap::{arg, command, Parser};
use log::{debug, info};
use std::fs::{self, File};
use std::io::{self, BufRead, Cursor, Read, Write};
use std::path::{Path, PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the file to be decompressed
    #[arg(short, long, value_name = "FILE")]
    input_file: PathBuf,

    /// Output location, defaults to input directory. It always uses the same name as the input
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();
    let input = args.input_file;
    let output_path = match args.output {
        Some(x) => x,
        None => {
            let mut out = input.clone();
            out.pop();
            let new_name = format!("{}-out", input.file_name().unwrap().to_string_lossy());
            debug!("{}", new_name);
            out.join(Path::new(&new_name))
        }
    };

    info!("reading from {}", input.to_string_lossy());
    info!("writing to {}", output_path.to_string_lossy());
    // File hosts.txt must exist in the current path
    // let mut result = Vec::new();
    let output_writer = File::create_new(&output_path);
    let mut output_writer = match output_writer {
        std::result::Result::Ok(x) => x,
        Err(_) => {
            fs::remove_file(&output_path).unwrap();
            File::create_new(output_path.clone()).expect("why did we fail after deleting?")
        }
    };
    let lines = read_lines(input)?;
    // Consumes the iterator, returns an (Optional) String
    for line in lines.flatten() {
        let mut wrapper_reader = Cursor::new(line);
        let base64_reader =
            DecoderReader::new(&mut wrapper_reader, &engine::general_purpose::STANDARD);
        let mut reader = brotli::Decompressor::new(
            base64_reader,
            4096, // buffer size
        );
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf[..]) {
                Err(e) => {
                    if let io::ErrorKind::Interrupted = e.kind() {
                        continue;
                    }
                    panic!("{}", e);
                }
                std::result::Result::Ok(size) => {
                    if size == 0 {
                        break;
                    }
                    output_writer.write_all(&buf[..size])?;
                    let text =
                        String::from_utf8(buf[..size].to_vec()).expect("no string from utf8");
                    debug!("decompressed to: {}", text);
                }
            }
        }
    }
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
