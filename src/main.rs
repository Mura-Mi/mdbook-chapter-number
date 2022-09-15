mod chapter_number;

use mdbook::errors::Error;
use std::{io, process};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use structopt::StructOpt;
use crate::chapter_number::ChapterNumber;

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// Check whether a renderer is supported by this preprocessor
    Supports { renderer: String },
}

fn main() {
    let opts = Opt::from_args();

    if let Some(Command::Supports { renderer: _ }) = opts.command {
        println!("hit!");
        process::exit(0);
    } else if let Err(e) = process() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn process() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let processed = ChapterNumber::new().run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;

    Ok(())
}
