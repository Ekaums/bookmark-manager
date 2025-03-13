use clap::{Parser, Subcommand}; // short for use clap::Parser as Parser, use clap::Subcommand as Subcommand

#[derive(Parser, Debug)] // TODO: what is debug?
#[command(about = "A simple path manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    // This field is a subcommand, so the subcommand provided will be parsed into it (and it will carry the options given since its enum!!)
    command: Commands,
}

#[derive(Subcommand, Debug)] // Subcommand trait parses subcommand flags
enum Commands {
    /// Add the current directory as a bookmark. Or optionally, provide the dir to bookmark
    Add { // TODO: can use named structs instead of these unnamed ones (https://github.com/ybda/shmarks/blob/main/src/cli.rs)
        #[arg(help = "Tag for the directory")]
        tag: String,

        //#[arg(help = "Path to a directory (default: current dir)")]
        //path: Option<String>,

        // use #[arg(short, long)] for - flags
    },
    Go {
        #[arg(help = "Tag for the directory to navigate to")]
        tag: String,
    },
    /// List bookmarks
    Ls,
    /// Rm bookmark
    Rm {
        
    },
}

fn add_path(tag : String){

}

fn main() {
    let args = Cli::parse();
    match args.command{
        Commands::Add { tag } => add_path(tag),
        _ => {}
    }

    println!("wassup bruv");
}
