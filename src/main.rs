use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the markdown file to parse
    #[arg(short, long)]
    path: PathBuf
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.path);
}
