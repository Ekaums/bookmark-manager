mod cli;
mod commands;

use clap::Parser; // TODO: why this needed here too?
use cli::{Cli, Commands};
use commands::*;

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Add { tag, path } => add_bookmark(tag, path),

        Commands::Ls => list_bookmarks(),
        
        _ => {}
    }
}
