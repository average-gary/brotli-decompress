
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use clap::{arg, command, Parser};
use log::info;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the file to be decompressed
    #[arg(short, long, value_name = "FILE")]
    input_file: PathBuf,

    /// Output location, defaults to input directory
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    let input = args.input_file;
    let output_path = match args.output {
        Some(x) => x,
        None => {
            let mut out = input.clone();
            out.pop();
            out
        },
    };
    
    info!("reading from {}", input.to_string_lossy());
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("-------------");
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
