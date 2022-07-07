use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Directory name
    directory: PathBuf,

    /// Show stats for each filename extension
    #[clap(short = 'A')]
    extensions: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let dir_entries = match args.directory.read_dir() {
        Ok(entries) => entries,
        Err(err) => panic!("{}", err),
    };
}
