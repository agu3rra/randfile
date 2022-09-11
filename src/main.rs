use std::fs;
use clap::Parser;

#[derive(Parser, Default, Debug)]
struct Arguments{
    directory: String,
}

fn main() {
    let args = Arguments::parse();
    let directory = args.directory;
    let files = fs::read_dir(directory)
        .expect("Error while reading the provided directory");
    for file in files {
        println!("File: {}", file.unwrap().path().display());
    }
}
