use clap::{Parser, Subcommand}; // short for use clap::Parser as Parser, use clap::Subcommand as Subcommand
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use toml::Table;

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
    Add {
        // TODO: can use named structs instead of these unnamed ones (https://github.com/ybda/shmarks/blob/main/src/cli.rs)
        #[arg(help = "Tag for the directory")]
        tag: String,
        #[arg(help = "Path to a directory TODO: (default: current dir)")]
        path: Option<String>,
        // use #[arg(short, long)] for - flags
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

// #[derive(Serialize, Deserialize, Debug)] // TODO: what is debug?
// struct Config {
//     // Holds TOML data
//     bookmarks: Table<Bookmark>,
// }

#[derive(Serialize, Deserialize, Debug)] // TODO: what is debug?
struct Bookmark {
    // Each entry
    tag: String,
    dir: String,
}

fn add_bookmark(tag: String, path: String) {
    let config_file = "bookmarks.toml"; // TODO: make a constant? if thats a thing
                                        // Load existing bookmarks
    if !Path::new(config_file).exists() {
        println!("Creating new {config_file}");
        fs::write(config_file, "[bookmarks]\n").expect("Failed to config create file!");
    }

    // Read contents of file
    let contents = match fs::read_to_string(config_file) // This is pretty much what .expect() does lul
                                                         // I think this err handling only makes sense if you aren't going to immediately panic
    {
        // If success return file contents as String
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read {config_file}");
            exit(1);
        }
    };

    // Parse the string into a toml table
    // Toml table is a hash map for string to a Toml value (which can be string, int, or in this case another table (e.g. bookmarks))
    let mut main_table: Table = contents.parse::<Table>().expect("Could not parse file contents");

    // Query the hash map for the bookmarks entry (which returns type &Value which is just an enum to the multiple types that a toml can have)
    let bookmarks = main_table.get("bookmarks").expect("No bookmarks entry found");
    let bookmarks = bookmarks.as_table().expect("Could not convert bookmark into table");
    let mut b_cpy = bookmarks.to_owned(); // bookmarks was a reference (borrow) to the entry in the main_table. so now we create a copy so we can insert new entry
    b_cpy.insert(tag, path.into()); // TODO: what is into?
    main_table.insert("bookmarks".to_owned(), toml::Value::Table(b_cpy)); // TODO: can i use .into again for b_cpy?

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file)
        .unwrap();

    println!("{}", main_table.to_string());
    file.write_all(main_table.to_string().as_bytes()).unwrap();
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Add { tag, path } => {
            add_bookmark(tag, path.expect("Default path not implemented"))
        } // TODO: handle no path case by using default path
        _ => {}
    }

    println!("wassup bruv");
}
