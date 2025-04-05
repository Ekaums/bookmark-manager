use clap::{Parser, Subcommand}; // short for use clap::Parser as Parser, etc
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "A simple path manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    // This field is a subcommand, so the subcommand provided will be parsed into it (and it will carry the options given since its enum!!)
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a bookmark (provide path or use current path by default)
    Add {
        // TODO: can use named structs instead of these unnamed ones (https://github.com/ybda/shmarks/blob/main/src/cli.rs)
        #[arg(help = "Tag for the directory")]
        tag: String,

        #[arg(help = "Path to a directory (default: current dir)")]
        path: Option<PathBuf>,
    },
    Go {
        #[arg(help = "Tag for the directory to navigate to")]
        tag: String,
    },
    /// List bookmarks
    Ls,
    /// Rm bookmark
    Rm {},
}
