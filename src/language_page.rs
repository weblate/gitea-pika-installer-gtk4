use gtk::pango::Language;
// Use libraries
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use glib::*;
use gdk::Display;
use gtk::subclass::layout_child;

use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

pub fn language_page(content_stack: &gtk::Stack) {

    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();
    
    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label("Back")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    let bottom_next_button = gtk::Button::builder()
        .label("Next")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::End)
        .hexpand(true)
        .sensitive(false)
        .build();
    
    // Start Applying css classes
    bottom_next_button.add_css_class("suggested-action");
    
    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);
    bottom_box.append(&bottom_next_button);

   // the header box for the language page
   let language_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the language page
   let language_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the language page
   let language_header_text = gtk::Label::builder()
       .label("Select a language")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    language_header_text.add_css_class("header_sized_text");

   // the header icon for the language icon
   let language_header_icon = gtk::Image::builder()
       .icon_name("locale")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();

   // make language selection box for choosing installation or live media 
   let language_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);
    
    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    // text above language selection box
    let language_selection_text = gtk::Label::builder()
        .label("Please select a locale for the system to use")
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_text.add_css_class("medium_sized_text");

    let language_selection_expander_row = adw::ExpanderRow::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .title("Locale")
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label("No locale selected")
        .build();

    let language_selection_expander_row_viewport = gtk::ScrolledWindow::builder()
        .height_request(200)
        .build();

    let language_selection_expander_row_viewport_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();

    language_selection_expander_row_viewport.set_child(Some(&language_selection_expander_row_viewport_box));

    language_selection_expander_row.add_row(&language_selection_expander_row_viewport);

    language_selection_expander_row_viewport_box.append(&null_checkbutton);

    let null_checkbutton_clone = null_checkbutton.clone();
    let language_selection_expander_row_clone2 = language_selection_expander_row.clone();
    let bottom_next_button_clone = bottom_next_button.clone();


    null_checkbutton.connect_toggled(move |_| {
        if null_checkbutton_clone.is_active() == true {
            language_selection_expander_row_clone2.set_title("No locale selected");
            bottom_next_button_clone.set_sensitive(false);
        }
    });

    let mut output = Command::new("locale")
        .arg("-a")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let stdout = output.stdout.as_mut().expect("could not get stdout");
    let reader = BufReader::new(stdout);

    for locale in reader.lines() {
        let locale = locale.unwrap();
        let locale_checkbutton = gtk::CheckButton::builder()
            .label(locale.clone())
            .build();
        locale_checkbutton.set_group(Some(&null_checkbutton));
        language_selection_expander_row_viewport_box.append(&locale_checkbutton); 
        let language_selection_expander_row_clone = language_selection_expander_row.clone();
        let locale_checkbutton_clone = locale_checkbutton.clone();
        let bottom_next_button_clone2 = bottom_next_button.clone();
        locale_checkbutton.connect_toggled(move |_| {
            if locale_checkbutton_clone.is_active() == true {
                language_selection_expander_row_clone.set_title(&locale);
                bottom_next_button_clone2.set_sensitive(true);
            }
        });
    }

    // / language_selection_box appends
    //// add text and and entry to language page selections
    language_selection_box.append(&language_selection_text);
    language_selection_box.append(&language_selection_expander_row);
    
    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);
    
    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    language_main_box.append(&bottom_box);
    
    // / Content stack appends
    //// Add the language_main_box as page: language_page, Give it nice title
    content_stack.add_titled(&language_main_box, Some("language_page"), "Language");

    let content_stack_clone = content_stack.clone();
    let content_stack_clone2 = content_stack.clone();
    bottom_next_button.connect_clicked(move |_| {
        content_stack_clone.set_visible_child_name("keyboard_page")
    });
    bottom_back_button.connect_clicked(move |_| {
        content_stack_clone2.set_visible_child_name("welcome_page")
    });

}
