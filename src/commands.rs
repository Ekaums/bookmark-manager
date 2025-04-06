use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use toml::Table;

pub fn add_bookmark(tag: String, path: Option<PathBuf>) {
    // TODO: bookmark struct?

    let path: String = resolve_path(path);

    // TODO: make a constant? if thats a thing
    let config_file = "bookmarks.toml";
    let bookmarks_table = "bookmarks";

    // Check if config file exists or create new one
    if !Path::new(config_file).exists() {
        println!("Creating new {config_file}");
        fs::write(config_file, "[bookmarks]\n").expect("Failed to config create file!");
    }

    // Read existing bookmarks
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
    let mut main_table: Table = contents
        .parse::<Table>()
        .expect("Could not parse file contents");

    // Query the hash map for the bookmarks entry (returns enum to the multiple types that a toml can have)
    let bookmarks = main_table
        .get(bookmarks_table)
        .expect("No bookmarks entry found"); // `bookmarks` is borrowing the entry
    
    let bookmarks = bookmarks
        .as_table()
        .expect("Could not convert bookmark into table");

    let mut b_cpy = bookmarks.to_owned(); // bookmarks was a reference (borrow) to the entry in the main_table. so now we create a copy so we can insert new entry
    b_cpy.insert(tag, path.into()); // from/into traits define how to convert from one type to another (in this case, from String to Value)
    main_table.insert("bookmarks".into(), toml::Value::Table(b_cpy));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file)
        .unwrap();

    file.write_all(main_table.to_string().as_bytes()).unwrap();
}

// Ensure path is valid
fn resolve_path(path: Option<PathBuf>) -> String {
    match path {
        // If user provided path
        Some(path_v) => {
            if !path_v.is_dir() {
                eprintln!("Error: Provided path was not a directory");
                exit(1);
            }
            path_v
                .canonicalize()
                .unwrap()
                .to_string_lossy() // Cow can either return a reference (if path can be cleanly converted into String) or a new string (which is owned)
                .into_owned() // Get absolute path
        }
        // No path provided (use current dir)
        None => {
            env::current_dir()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        }
    }
}
