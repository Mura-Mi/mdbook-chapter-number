mod chapter_number;

use std::process;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// Check whether a renderer is supported by this preprocessor
    Supports { _renderer: String },
}

fn main() {
    let opts = Opt::from_args();

    if let Some(Command::Supports { _renderer: _ }) = opts.command {
        process::exit(0);
    } else {
        eprintln!("out");
        process::exit(1);
    }
}
