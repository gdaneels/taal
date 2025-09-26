use clap::Parser;
mod taal;

#[derive(Parser, Debug)]
struct Args {
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    taal::taal(args.path);
}
