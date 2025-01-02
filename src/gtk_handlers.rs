use std::rc::Rc;
use gtk::prelude::*;
use gtk;

use crate::notes;

// Dialog message defaults
const NEW_NOTE_DIAG: &str = "New Note";
const DELETE_NOTE_DIAG: &str = "Delete Note";

// Error messages
const CANT_ADD_NOTES: &str = "Can't add more notes";
const NO_BUTTONS_TO_DELETE: &str = "There are no buttons to delete";
const NOTE_CANT_BE_EMPTY: &str = "Note data can't be empty";

// CSS Classes
const DIAG_TITLE_CLASS: &str = "diag_title";
const DIAG_CONTENTS_CLASS: &str = "diag_contents";
const DIAG_BUTTON_CLASS: &str = "diag_button";
pub const NOTE_BUTTON_CLASS: &str = "note_btn";
pub const INTERACT_BUTTON_CLASS: &str = "interact_btn";

// CSS Names
const DIALOG_BOX: &str = "diag_box";
pub const MAIN_CONTAINER: &str = "main_container";
pub const BUTTON_BOX: &str = "button_box";
pub const CONTENT_BOX: &str = "content_box";


/**
Click event handler for "add note" button.

# Parameters:
* `bbox_clone_ref`: A reference to the buttons box.
* `text_box_ref`: A reference to the box that displays note's contents.
* `app_ref`: A reference to the gtk application.
 */
pub fn add_button_click_event(
    bbox_clone_ref: &gtk::Box,
    text_box_ref: &Rc<gtk::Label>,
    app_ref: &gtk::Application
) {
    let hchilds = get_hbox_childs(bbox_clone_ref);
    if hchilds.len() >= 5 {
        eprintln!("add_button_click_event: {}", CANT_ADD_NOTES);
        return;
    }

    // Initialize the main dialog window
    let dialog = gtk::Dialog::builder()
        .title(NEW_NOTE_DIAG)
        .transient_for(&app_ref.active_window().unwrap())
        .destroy_with_parent(true)
        .modal(true)
        .css_name(DIALOG_BOX)
        .build();

    // Everything will be on this content area
    let content_area = dialog.content_area();
    let grid = gtk::Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

    // Note title label and entry
    let title_label = gtk::Label::builder()
        .label("Note title")
        .css_classes([DIAG_TITLE_CLASS])
        .build();

    let title_entry = gtk::Entry::builder()
        .width_request(600)
        .placeholder_text("Note title")
        .css_classes([DIAG_TITLE_CLASS])
        .build();

    // Note contents label and Text View
    let content_label = gtk::Label::builder()
        .label("Note Contents")
        .css_classes([DIAG_CONTENTS_CLASS])
        .build();

    let content_text_view = gtk::TextView::builder()
        .width_request(600)
        .height_request(100)
        .wrap_mode(gtk::WrapMode::Word)
        .cursor_visible(true)
        .accepts_tab(true)
        .css_classes([DIAG_CONTENTS_CLASS])
        .build();

    let create_button = gtk::Button::with_label("Create");
    create_button.style_context().add_class(DIAG_BUTTON_CLASS);
    
    grid.attach(&title_label, 0, 0, 1, 1);
    grid.attach(&title_entry, 1, 0, 1, 1);
    grid.attach(&content_label, 0, 1, 1, 1);
    grid.attach(&content_text_view, 1, 1, 1, 3);

    content_area.append(&grid);
    content_area.append(&create_button);

    // Create clones to use inside Fn enclosure
    let bbox_clone = bbox_clone_ref.clone();
    let text_box_clone = text_box_ref.clone();

    dialog.show();

    create_button.connect_clicked(move |_| {
        create_note_button_click_event(
            &title_entry,
            &content_text_view,
            &text_box_clone,
            &bbox_clone,
            &dialog
        );
    });
}


/**
Click event handler for the "remove note" button.
# Parameters:
* `bbox_clone_ref`: A reference to the buttons horizontal box.
* `app_ref`: A reference to the gtk application
 */
pub fn rm_button_click_event(bbox_clone_ref: &gtk::Box, app_ref: &gtk::Application){
    let hchilds = get_hbox_childs(bbox_clone_ref);
    if hchilds.is_empty() {
        eprintln!("rm_button_click_event: {}", NO_BUTTONS_TO_DELETE);
        return;
    }

    // Initialize dialog window
    let dialog = gtk::Dialog::builder()
        .title(DELETE_NOTE_DIAG)
        .transient_for(&app_ref.active_window().unwrap())
        .destroy_with_parent(true)
        .modal(true)
        .css_name(DIALOG_BOX)
        .build();

    // All content will be in this box
    let content_area = dialog.content_area();

    // Input box for note title to identify the note to be deleted.
    let input_box = gtk::Entry::builder()
        .placeholder_text("Note title")
        .css_classes([DIAG_TITLE_CLASS])
        .build();

    let delete_button: gtk::Button = gtk::Button::with_label("Remove");
    delete_button.style_context().add_class(DIAG_BUTTON_CLASS);

    content_area.append(&input_box);
    content_area.append(&delete_button);

    // Create clones to use inside Fn enclosure
    let bbox_clone = bbox_clone_ref.clone();

    dialog.show();

    delete_button.connect_clicked(move |_| {
        let note_title = input_box.text().trim().to_string();
        if note_title.is_empty() {
            eprintln!("rm_button_click_event: {}", NOTE_CANT_BE_EMPTY);
            return;
        }

        for child in hchilds.clone() {
            // Downcast a widget to a button to access it's ;abe;
            let btn = child.downcast::<gtk::Button>().unwrap();
            if note_title != btn.label().expect("Button has no label").trim().to_string() {
                continue;
            }
    
            // Remove the note from the buttons box and from storage.
            bbox_clone.remove(&btn);
            let success = notes::delete_a_note(&note_title);
            if success == false {
                return;
            }
        }

        dialog.close();
        dialog.destroy();
    });
}


/**
"create note" button handler when adding a new note.
# Parameters:
* `title_entry_ref`: A reference to an entry for the created note's title input.
* `content_text_view_ref`: A reference to a text view with the note's content.
* `text_box_ref`: A reference to the label that displays notes contents.
* `bbox_ref`: A reference to the buttons box.
* `dialog_ref`: A reference to the initialized dialog.
 */
fn create_note_button_click_event(
    title_entry_ref: &gtk::Entry,
    content_text_view_ref: &gtk::TextView,
    text_box_ref: &Rc<gtk::Label>,
    bbox_ref: &gtk::Box,
    dialog_ref: &gtk::Dialog
) {
    // Extract the title and contents
    let title = title_entry_ref.text().trim().to_string();
    let buffer = content_text_view_ref.buffer();
    let start_iter = buffer.start_iter();
    let end_iter = buffer.end_iter();
    let contents = buffer.text(&start_iter, &end_iter, false).trim().to_string();

    if title.is_empty() || contents.is_empty() {
        eprintln!("create_note_button_click_event: {}", NOTE_CANT_BE_EMPTY);
        return;
    }

    let success = notes::create_a_note(&title, &contents);
    if !success {
        return;
    }

    // Create a new note button
    let new_button = gtk::Button::with_label(&title);
    new_button.style_context().add_class(NOTE_BUTTON_CLASS);

    // Create another clone for another Fn enclosure
    let tb_clone = text_box_ref.clone();

    new_button.connect_clicked(move |_| {
        notes::display_file_contents(&title, &tb_clone);
    });

    bbox_ref.append(&new_button);
    new_button.show();
    dialog_ref.close();
    dialog_ref.destroy();
}


/**
Get all childs from a gtk box.
# Parameters:
* `hbox`: A reference to a gtk box to get their childs.
# Return:
A vector of gtk widgets. 
 */
fn get_hbox_childs(hbox: &gtk::Box) -> Vec<gtk::Widget> {
    let mut children: Vec<gtk::Widget> = Vec::new();
    let mut sibling = hbox.first_child();

    while let Some(child) = sibling {
        children.push(child.clone());
        sibling = child.next_sibling();
    }

    children
}
