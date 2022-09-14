use std::fs::{self, DirEntry, ReadDir};
use clap::Parser;
use rand::Rng;

#[derive(Parser, Default, Debug)]
struct Arguments{
    directory: String,
}

fn main() {
    let args = Arguments::parse();
    let directory: String = args.directory;
    let paths: ReadDir = fs::read_dir(&directory)
        .expect("Error while reading the provided directory");
    let mut files: Vec<String> = Vec::new();
    // Traverse folder and identify files since we're not interested in directories.
    for path in paths {
        let entry: DirEntry = path.unwrap();
        let is_dir = &entry.metadata().unwrap().is_dir();
        if ! is_dir {
            let filename = entry.file_name().into_string().expect("Error converting to String from this OS.");
            files.push(filename);
        }
    }
    if files.len() == 0 {
        panic!("The provided directory has no files to select.");
    }
    println!("Files identified in {}: {}", directory, files.len());
    let random_select: usize = rand::thread_rng().gen_range(0..files.len());
    println!("File selected at random: {}", files[random_select]);
}
