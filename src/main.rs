use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.file).unwrap();
    println!("File: {}", file);
    println!("Not implemented");
}
