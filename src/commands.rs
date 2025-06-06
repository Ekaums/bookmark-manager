use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use toml::Table;

const CONFIG_FILE: &str = "/Users/ekaumsingh/ws/repo/rust/bookmark-manager/bookmarks.toml";
const TABLE_NAME: &str = "bookmarks";

pub fn add_bookmark(tag: String, path: Option<PathBuf>) {
    // TODO: bookmark struct?

    let path: String = resolve_path(path);

    // TODO: make this cleaner with unpacking

    // Check if config file exists or create new one
    if !Path::new(CONFIG_FILE).exists() {
        println!("Creating new {CONFIG_FILE}");
        fs::write(CONFIG_FILE, "[bookmarks]\n").expect("Failed to config create file!");
    }

    // Read existing bookmarks
    // TODO: clean up this with unified fn
    let contents = match fs::read_to_string(CONFIG_FILE) // This is pretty much what .expect() does
    {
        // If success return file contents as String
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read {CONFIG_FILE} for adding");
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
        .get(TABLE_NAME)
        .expect("No bookmarks entry found") // `bookmarks` is borrowing the entry
        .as_table()
        .expect("Could not convert bookmark into table");

    let mut b_cpy = bookmarks.to_owned(); // bookmarks was a reference (borrow) to the entry in the main_table. so now we create a copy so we can insert new entry
    b_cpy.insert(tag, path.into());
    main_table.insert("bookmarks".into(), toml::Value::Table(b_cpy));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(CONFIG_FILE)
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
        None => env::current_dir().unwrap().to_string_lossy().into_owned(),
    }
}

// TODO: move to new file?
pub fn list_bookmarks() {
    // Check if config file exists
    if !Path::new(CONFIG_FILE).exists() {
        println!("No bookmarks created :(");
        return;
    }

    /* This can't be combined all into one method chain,
       Because .parse() allocates new memory on the heap for the parsed data.
       If I don't bind a variable to that memory, then I would just be grabbing a reference
       to the memory (with .get()), but that memory goes out of scope by the semicolon.
       (Dangling reference)
       So instead, bind the memory to a variable then grab a reference and use it
    */
    let table = fs::read_to_string(CONFIG_FILE)
        .expect("Could not read {CONFIG_FILE} for listing")
        .parse::<Table>()
        .expect("Could not parse file contents for listing");

    let bookmarks = table
        .get(TABLE_NAME)
        .and_then(|v| v.as_table())
        .expect("Could not convert bookmarks into table");

    for (tag, path) in bookmarks {
        // TODO: how this work
        println!("{:<} → {}", tag, path);
    }
}

pub fn go_bookmark(tag : String){
    // Check if config file exists
    if !Path::new(CONFIG_FILE).exists() {
        println!("No bookmarks created :(");
        return;
    }

    // TODO: Check that this is a valid tag, and get the dir


    // Check that the dir exists that we wanna navigate to
    // TODO: move getting table to new fn (done in multiple places)
    let table = fs::read_to_string(CONFIG_FILE)
        .expect("Could not read {CONFIG_FILE} for listing")
        .parse::<Table>()
        .expect("Could not parse file contents for listing");

    let bookmark = table
        .get(TABLE_NAME)
        .and_then(|v| v.as_table())
        .expect("Could not convert bookmarks into table")
        .get(&tag)
        .unwrap();

    let path = bookmark.as_str().unwrap();

    println!("{}", path);
    // Output this path (to do the cd)
}
