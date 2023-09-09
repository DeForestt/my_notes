use std::{
    env::{temp_dir, var},
    fs::{File, create_dir_all, OpenOptions, write},
    io::{Read, self, Write},
    process::Command
};

use cli::{Cli, Commands};
use note::Note;
use clap::Parser;

pub mod note;
pub mod cli;

fn get_editor() -> String {
    var("EDITOR").unwrap_or_else(|_| "vim".to_string())
}

fn create_temp_file() -> String {
    let mut file_path = temp_dir();
    file_path.push("editable.md");
    File::create(&file_path).expect("Could not create file");
    file_path.to_str().unwrap().to_string()
}

fn open_editor(file_path: &str) {
    Command::new(get_editor())
        .arg(file_path)
        .status()
        .expect("Something went wrong");
}

fn get_editor_content(path: &str) -> String {

    open_editor(path);

    let mut editable = String::new();
    File::open(path)
        .expect("Could not open file")
        .read_to_string(&mut editable)
        .expect("Could not read file");
    editable
}

fn save_note(path: &str, note_content: &str) {
    let home_dir = var("HOME").expect("Could not get home directory");
    let note_dir = format!("{}/.notes", &home_dir);
    let index_path = format!("{}/index.json", &note_dir);

    create_dir_all(&note_dir).expect("Could not create save path");
    
    let note_path = format!("{}/{}.md", &note_dir, &path);

    let mut note_path_parts: Vec<&str> = path.split(".").collect();
    note_path_parts.insert(0, "MyNotesRoot");

    let mut index = Note::new(&index_path);

    match index.add_child_at(note_path_parts, &note_path) {
        Ok(_) => {},
        Err(e) => {
            println!("Could not create note: {}", e);
            return;
        },
    };

    let mut note_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&note_path)
        .expect("Could not create note file");
    
    index.save(&index_path);

    note_file.write_all(note_content.as_bytes()).expect("Could not write to note file");
}

fn list_tree(path: &Option<String>) {   
    let home_dir = var("HOME").expect("Could not get home directory");
    let note_dir = format!("{}/.notes", &home_dir);
    let index_path = format!("{}/index.json", &note_dir);

    create_dir_all(&note_dir).expect("Could not create save path");
    let index = Note::new(&index_path);

    let root: &Note = match path {
        Some(path) => {
            let mut note_path_parts: Vec<&str> = path.split(".").collect();
            note_path_parts.insert(0, "MyNotesRoot");
            match index.get_child(note_path_parts) {
                Some(note) => note,
                None => {
                    println!("Could not find note");
                    return;
                },
            }
        },
        None => &index,
    };

    root.print_tree(0);
    println!("");
}

fn edit_note(path: &String) {
    let home_dir = var("HOME").expect("Could not get home directory");
    let note_dir = format!("{}/.notes", &home_dir);
    let index_path = format!("{}/index.json", &note_dir);

    create_dir_all(&note_dir).expect("Could not create save path");

    let mut note_path_parts: Vec<&str> = path.split(".").collect();
    note_path_parts.insert(0, "MyNotesRoot");

    let index = Note::new(&index_path);

    let note = match index.get_child(note_path_parts) {
        Some(note) => note,
        None => {
            println!("Could not find note");
            return;
        },
    };

    let note_content = get_editor_content(note.get_file_path());
    
    write(note.get_file_path(), note_content).expect("Could not write to note file");


}

fn echo_note_content(path: &String) {
    let home_dir = var("HOME").expect("Could not get home directory");
    let note_dir = format!("{}/.notes", &home_dir);
    let index_path = format!("{}/index.json", &note_dir);

    create_dir_all(&note_dir).expect("Could not create save path");

    let mut note_path_parts: Vec<&str> = path.split(".").collect();
    note_path_parts.insert(0, "MyNotesRoot");

    let index = Note::new(&index_path);

    let note = match index.get_child(note_path_parts) {
        Some(note) => note,
        None => {
            println!("Could not find note");
            return;
        },
    };

    let mut note_file = OpenOptions::new()
        .read(true)
        .open(note.get_file_path())
        .expect("Could not open note file");

    let mut note_content = String::new();
    note_file.read_to_string(&mut note_content).expect("Could not read note file");

    println!("{}", note_content);
}

fn delete_note(path: &String) {
    let home_dir = var("HOME").expect("Could not get home directory");
    let note_dir = format!("{}/.notes", &home_dir);
    let index_path = format!("{}/index.json", &note_dir);

    create_dir_all(&note_dir).expect("Could not create save path");

    let mut note_path_parts: Vec<&str> = path.split(".").collect();
    note_path_parts.insert(0, "MyNotesRoot");

    let mut index = Note::new(&index_path);

    match index.remove_child(note_path_parts.clone()) {
        Ok(paths) =>   for path in paths {
            match std::fs::remove_file(path) {
                Ok(_) => {},
                Err(e) => {
                    println!("Could not delete note: {}", e);
                    print!("Would you like to remove the note from the index? (y/N): ");
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Could not read input");
                    if input.trim() != "y" {
                        return;
                    }
                },
            }
        },
        Err(e) => {
            println!("Could not delete note: {}", e);
            return;
        },
    };

    index.remove_from_index(note_path_parts).expect("Could not remove note from index");
    index.save(&index_path);
}

fn search_notes(query: &String) {
    let home_dir = var("HOME").expect("Could not get home directory");
    let note_dir = format!("{}/.notes", &home_dir);
    let index_path = format!("{}/index.json", &note_dir);

    create_dir_all(&note_dir).expect("Could not create save path");

    let index = Note::new(&index_path);

    let results = index.key_word_search(query.to_uppercase().as_str());

    for result in results {
        println!("{}\n", result);
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::New { path, content, blank} => {
            if *blank {
                save_note(path, "");
                return;
            }
            match content {
                Some(content) => {
                    save_note(path, content);
                    return;
                },
                None => {
                    let file_path = create_temp_file();
                    let note_content = get_editor_content(&file_path);
                    save_note(path, &note_content);
                }
            }
        },
        Commands::List { path } => {
            list_tree(path);
        },
        Commands::Edit { path } => {
            edit_note(path);
        },
        Commands::Delete { path, force } => {

            let mut input = String::new();
            if !*force {
                print!("Deleting this note will PERMANENTLY delete it and all of its children from the index and your local computer. Are you sure you want to continue? (y/N): ");
                io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).expect("Could not read input");
                if input.to_lowercase().trim() != "y" {
                    return;
                }
            }

            delete_note(path);
        },
        Commands::Search { query } => {
            search_notes(query);
        },
        Commands::Echo { path } => {
            echo_note_content(path);
        }
    }
}