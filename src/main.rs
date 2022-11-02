use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process;
use std::path::PathBuf;
use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of file
    #[clap(value_parser, value_name = "FILE")]
    path: Option<PathBuf>,

    /// Number of lines
    #[clap(short, long, value_parser)]
    num: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or_else(|| {
        println!("No path provided");
        process::exit(1);
    });
    let num = cli.num.unwrap_or_else(|| 10);

    let file = File::open(path).unwrap_or_else(|err| {
        println!("Unable to read file: {err}");
        process::exit(1);
    });
    let mut reader = BufReader::new(file);

    let mut counter = 0;
    let mut output = String::new();
    while counter < num {
        counter += 1;
        let num_bytes = reader.read_line(&mut output)
            .expect("Read failed");
        if num_bytes == 0 {
            break;
        }
    }
    print!("{}", output)
}
