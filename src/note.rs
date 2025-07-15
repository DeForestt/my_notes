use std::fs::OpenOptions;

use serde::{Deserialize, Serialize};

use magic_string_search::struct_rank;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    title: String,
    file_path: String,
    children: Vec<Note>,
}

impl Note {
    pub fn new(file_path: &str ) -> Note {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .expect("Could not create note file");
        let note = match serde_json::from_str(
            &std::fs::read_to_string(&file_path).expect("Could not read not file"),
        ) {
            Ok(note) => note,
            Err(_) => Note {
                title: "MyNotesRoot".to_string(),
                file_path: file_path.to_string(),
                children: Vec::new(),
            },
        };
        return note;
    }

    fn get_lowercase_name(&self) -> String {
        let title = self.title.to_lowercase();
        title
    }

    pub fn save(&self, save_path: &str) {
        let note_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&save_path)
            .expect("Could not create note file");

        // erase the file
        note_file.set_len(0).expect("Could not erase file");

        serde_json::to_writer(note_file, &self).expect("Could not write note");
    }

    pub fn get_child(&self, mut path_parts: Vec<&str>) -> Option<&Note> {
        if path_parts.len() == 0 {
            return None;
        }
        let child_name = path_parts.remove(0);
        if child_name == self.title  && path_parts.len() == 0 {
            return Some(self);
        }
        for child in &self.children {
            match child.get_child(path_parts.clone()) {
                Some(note) => return Some(note),
                None => {},
            }
        }
        return None;
    }

    fn get_child_mut(&mut self, mut path_parts: Vec<&str>) -> Option<&mut Note> {
        if path_parts.len() == 0 {
            return None;
        }
        let child_name = path_parts.remove(0);
        if child_name == self.title  && path_parts.len() == 0 {
            return Some(self);
        }
        for child in &mut self.children {
            match child.get_child_mut(path_parts.clone()) {
                Some(note) => return Some(note),
                None => {},
            }
        }
        return None;
    }

    pub fn add_child_at(&mut self, mut path_parts: Vec<&str>, file_path: &str) -> Result<&str, &str> {
        // the last part of the path is the new note's title
        let new_note_title = match path_parts.pop() {
            Some(title) => title,
            None => return Err("No title provided"),
        };
        // the rest of the path is the path to the parent note
        let parent_path = path_parts;
        // get the parent note
        let parent_note = match self.get_child_mut(parent_path) {
            Some(note) => note,
            None => return Err("Could not find parent note"),
        };

        // create the new note
        let new_note = Note {
            title: new_note_title.to_string(),
            file_path: file_path.to_string(),
            children: Vec::new(),
        };

        // add the new note to the parent note
        parent_note.children.push(new_note);

        return Ok("Success");
    }

    pub fn print_tree(&self, indent: usize) -> () {
        if self.title != "MyNotesRoot" {
            print!("- {}", self.title);
        }
        for child in &self.children {
            println!();
            for _ in 0..indent {
                print!("  ");
            }
            child.print_tree(indent + 1);
        }
    }

    pub fn get_file_path(&self) -> &str {
        return &self.file_path;
    }

    pub fn get_title(&self) -> &str {
        return &self.title;
    }

    pub fn remove_child(&mut self, path: Vec<&str>) -> Result<Vec<&str>, &str> {
        match self.get_child_mut(path) {
            Some(note) => {
                return Ok(note.recurse_remove());
            },
            None => {
                return Err("Could not find note");
            },
        };
    }

    pub fn recurse_remove(&mut self) -> Vec<&str> {
        let mut path_list: Vec<&str> = Vec::new();
        path_list.push(&self.file_path);
        for child in &mut self.children {
            path_list.append(&mut child.recurse_remove());
        }
        return path_list;
    }

    pub fn remove_from_index(&mut self, path: Vec<&str>) -> Result<(), &str> {
        let mut path_parts = path.clone();
        let child_name = path_parts.pop().unwrap();
        if child_name == self.title  && path_parts.len() == 0 {
            return Err("Cannot remove root note");
        }

        let parent = match self.get_child_mut(path_parts) {
            Some(note) => note,
            None => return Err("Could not find parent note"),
        };

        parent.children.retain(|child| child.title != child_name);
        return Ok(());
    }

    fn collect(&self) -> Vec<&Note> {
        let mut note_list: Vec<&Note> = Vec::new();
        note_list.push(self);
        for child in &self.children {
            note_list.append(&mut child.collect());
        }
        return note_list;
    }

    pub fn key_word_search(&self, key_word: &str, limit: &usize) -> Vec<(f64, &Note)> {
        let note_list = self.collect();
        
        let binding = key_word.to_lowercase();
        let key_word = binding.as_str();

        let ranked_list: Vec<(f64, &Note)> = struct_rank(key_word, note_list, |&note| note.get_lowercase_name() )
            .into_iter().filter(|(rank, _)| *rank > 0.0).collect();
        if *limit > 0 {
            return ranked_list.into_iter().take(*limit).collect();
        }
        return ranked_list;
    }

    pub fn collect_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        self.collect_paths_inner(String::new(), &mut paths);
        paths
    }

    fn collect_paths_inner(&self, prefix: String, paths: &mut Vec<String>) {
        let mut current = prefix;
        if self.title != "MyNotesRoot" {
            if !current.is_empty() {
                current.push('.');
            }
            current.push_str(&self.title);
            paths.push(current.clone());
        }
        for child in &self.children {
            child.collect_paths_inner(current.clone(), paths);
        }
    }
}
