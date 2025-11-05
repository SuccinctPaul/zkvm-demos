use clap::Parser;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub execute: bool,

    #[arg(long)]
    pub prove: bool,
}
