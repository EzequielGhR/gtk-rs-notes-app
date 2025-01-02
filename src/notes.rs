use std::path;
use std::fs;
use std::io::{Read, Write};
use::gtk::Label;


// generic constants
pub const NOTES_PATH: &str = "./notes/";
pub const MAX_NOTES: usize = 5;

// Error messages
const NOTE_ALREADY_EXISTS: &str = "Note already exists";
const ERROR_LABEL_TEXT: &str = "<ERROR: Could not read note content>";


/**
Load notes from default path
# Return
A vector with the title of the notes stored or an empty vector.
 */
pub fn load_notes() -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(MAX_NOTES);

    if !path::Path::new(NOTES_PATH).exists() {
        println!("Creating a new directory {NOTES_PATH}");

        // Handle possible errors creating the directory
        if let Err(e) = fs::create_dir(NOTES_PATH) {
            eprintln!("load_notes: Failed to create dirrectory {NOTES_PATH}: {e}");
            return result;
        }
    }
    
    let dir_iterator = match fs::read_dir(NOTES_PATH) {
        Ok(iterator) => iterator,
        Err(e) => {
            eprintln!("load_notes: Error reading directory {NOTES_PATH}: {e}");
            return result;
        }
    };

    for file in dir_iterator {
        if result.len() >= MAX_NOTES {
            break;
        }

        let file_name = match file {
            Ok(entry) =>  entry.file_name(),
            Err(e) => {
                eprintln!("load_notes: Error reading file entry: {e}");
                continue;
            }
        };

        let note_title = match file_name.into_string() {
            Ok(title) => title,
            Err(e) => {
                eprintln!("load_notes: Error getting file name as string: {e:?}");
                continue;
            }
        };

        if let Some(title) = note_title.strip_suffix(".txt") {
            result.push(title.trim().to_string());
        } else {
            eprintln!("load_notes: File is not a .txt file: {note_title}");
            continue;
        }
    }

    result
}


/**
Create a new note on the default path as a txt file.
# Parameters:
* `note_title`:The title for the note without extension
* `contents`: The note contents.

# Return:
Boolean stating if the note creation was successful.
 */
pub fn create_a_note(note_title: &str, contents: &str) -> bool {
    let note_path = path::PathBuf::from(NOTES_PATH).join(format!("{note_title}.txt"));
    if note_path.exists() {
        eprintln!("{}", NOTE_ALREADY_EXISTS);
        return false;
    }

    let mut new_note = match fs::File::create(&note_path) {
        Ok(fstream) => fstream,
        Err(e) => {
            eprintln!("create_a_note: Error crating note {note_title} at path {note_path:?}: {e}");
            return false;
        }
    };

    // We don't care about the amount of bytes written so we use write_all.
    if let Err(e) = new_note.write_all(contents.as_bytes()) {
        eprintln!("create_a_note: Error writing to new note at path {note_path:?}: {e}");
        return false;
    }

    true
}


/**
Delete a note from the default path.
# Parameters:
*`note_title`: The title of the note to be deleted.
# Return:
A boolean stating if the operation went successfully.
 */
pub fn delete_a_note(note_title: &str) -> bool {
    let note_path = path::PathBuf::from(NOTES_PATH).join(format!("{note_title}.txt"));
    if !note_path.exists() {
        eprintln!("delete_a_note: Note with path {note_path:?} does not exist");
        return false;
    }

    if let Err(e) = fs::remove_file(&note_path) {
        eprintln!("delete_a_note: Error deleting note at path {note_path:?}: {e}");
        return false;
    }
    
    true
}


/**
Display contents of a note on the desired gtk label.

# Parameters:
*`file_name`: The name of the note to display on the desired label.
*`label`: A reference to a gtk label well the note's contents will be displayed
 */
pub fn display_file_contents(file_name:&str, label: &Label) {
    let file_path = path::PathBuf::from(NOTES_PATH).join(format!("{file_name}.txt"));
    let mut fstream = match fs::File::open(&file_path) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("display_file_contents: Error opening file at path {file_path:?}: {e}");
            label.set_text(ERROR_LABEL_TEXT);
            return;
        }   
    };

    let mut buffer = String::with_capacity(1024);
    if let Err(e) = fstream.read_to_string(&mut buffer) {
        eprintln!("display_file_contents: Error reading file at path {file_path:?}: {e}");
        label.set_text(ERROR_LABEL_TEXT);
        return;
    }
    
    label.set_text(&format!("{buffer} ..."));
}