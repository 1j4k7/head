use std::fs;
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
    let path = cli.path.unwrap_or_else(|| panic!());
    let num = cli.num.unwrap_or_else(|| 10);

    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        println!("Unable to read file: {err}");
        process::exit(1);
    });

    let mut counter = 1;
    for line in contents.lines() {
        if counter > num {
            break;
        }
        println!("{line}");
        counter += 1;
    }
}
