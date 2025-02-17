use std::rc::Rc;
use std::path::PathBuf;
use std::str::FromStr;
use gtk::{self, prelude::*};

mod notes;
mod gtk_handlers;


const APP_ID: &str = "org.zeke-desktop-app";
const APP_NAME: &str = "My Notes";
const CSS_PATH: &str = "css/style.css";
const DEFAULT_WIDTH: i32 = 800;

const ADD_NOTE_LABEL: &str = "Add new note";
const REMOVE_NOTE_LABEL: &str = "Remove a note";
const EDIT_NOTE_LABEL: &str = "Edit a note";
const TEXT_BOX_LABEL: &str = "** Your note contents will show here **";


fn main() {
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(create_app_structure);

    println!("Application started...");
    app.run();
    println!("Application terminated.");
}


/**
Creates general app structure with buttons functionalities
# Parameters:
* `app_ref`: A reference to the gtk application.
 */
fn create_app_structure(app_ref: &gtk::Application) {
    // We need to use multiple clones of the application, so we'll use a
    // smart reference-counted pointer, this will be done with multiple elements.
    // Sometimes an element has to be dereferenced before referencing. e: Rc<T> => *e: T => &*e: &T 
    let app_ref = Rc::new(app_ref.clone());

    // ** Main container for app elements **
    let vertical_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .css_name(gtk_handlers::MAIN_CONTAINER)
        .build();

    // ** Create base level app elements **
    // The buttons box will also have multiple clones, so we use Rc.
    let buttons_box = Rc::new(gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(5)
        .homogeneous(true)
        .css_name(gtk_handlers::BUTTON_BOX)
        .build());
    
    let add_button = gtk::Button::with_label(ADD_NOTE_LABEL);
    let remove_button = gtk::Button::with_label(REMOVE_NOTE_LABEL);
    let edit_button = gtk::Button::with_label(&format!("{EDIT_NOTE_LABEL} ({})", gtk_handlers::TEXT_EDITOR));

    add_button.style_context().add_class(gtk_handlers::INTERACT_BUTTON_CLASS);
    remove_button.style_context().add_class(gtk_handlers::INTERACT_BUTTON_CLASS);
    edit_button.style_context().add_class(gtk_handlers::INTERACT_BUTTON_CLASS);

    // We'll also use Rc for the text_box so we can create reference counted pointers.
    let text_box = Rc::new(gtk::Label::builder()
        .label(TEXT_BOX_LABEL)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .margin_top(12)
        .height_request(100)
        .use_markup(true)
        .css_name(gtk_handlers::CONTENT_BOX)
        .build());
    
    // Arrange main vertical box for display
    vertical_box.append(&*buttons_box);
    vertical_box.append(&*text_box);
    vertical_box.append(&add_button);
    vertical_box.append(&remove_button);
    vertical_box.append(&edit_button);

    let note_titles = notes::load_notes();

    // Create a button for each note and append it to the horizontal buttons box.
    for note_title in note_titles {
        let note_btn = gtk::Button::with_label(&note_title);
        note_btn.style_context().add_class(gtk_handlers::NOTE_BUTTON_CLASS);

        buttons_box.append(&note_btn);

        // Create a reference clone for each button click event
        let text_box_clone = Rc::clone(&text_box);

        note_btn.connect_clicked(move |_| {
            notes::display_file_contents(&note_title, &text_box_clone);
        });
    }

    // Create a reference clone for the add button
    let mut buttons_box_clone = Rc::clone(&buttons_box);
    let mut app_ref_clone = Rc::clone(&app_ref);
    add_button.connect_clicked(move |_| {
        gtk_handlers::add_button_click_event(&buttons_box_clone, &text_box, &app_ref_clone);
    });

    // Another reference clone for the remove button
    buttons_box_clone = Rc::clone(&buttons_box);
    app_ref_clone = Rc::clone(&app_ref);
    remove_button.connect_clicked(move |_| {
        gtk_handlers::rm_button_click_event(&buttons_box_clone, &app_ref_clone);
    });

    // Another reference clone for the edit button
    buttons_box_clone = Rc::clone(&buttons_box);
    app_ref_clone = Rc::clone(&app_ref);
    edit_button.connect_clicked(move |_| {
        gtk_handlers::edit_button_click_event(&buttons_box_clone, &app_ref_clone);
    });

    // Create window and display it.
    let window = gtk::ApplicationWindow::builder()
        .application(&*app_ref)
        .default_width(DEFAULT_WIDTH)
        .title(APP_NAME)
        .child(&vertical_box)
        .build();

    window.show();
}


/**
Load css styles from a path, or don't load anything on error.
 */
fn load_css() {
    let css_path = match PathBuf::from_str(CSS_PATH) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("load_css: Failed to get file path for {CSS_PATH}: {e}");
            return;
        }
    };

    if !css_path.exists() {
        eprintln!("load_css: css file at path {CSS_PATH} does not exist");
        return;
    }
    
    let provider = gtk::CssProvider::new();
    provider.load_from_path(css_path);

    // Set style context for default display with a high priority
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not load CSS file"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}
