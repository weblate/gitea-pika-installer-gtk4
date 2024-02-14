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
use std::env;

use std::fs;
use std::path::Path;


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
        .title("No locale selected")
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

    let language_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    language_selection_expander_row_viewport_listbox.append(&language_selection_expander_row);

    language_selection_expander_row.add_row(&language_selection_expander_row_viewport);

    let current_locale = match env::var_os("LANG") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$LANG is not set")
    };

    let locale_cli = Command::new("locale")
        .arg("-a")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));
    let locale_cli_cut = Command::new("cut")
        .arg("-d.")
        .arg("-f1")
        .stdin(Stdio::from(locale_cli.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let locale_cli_sort = Command::new("sort")
        .arg("-u")
        .stdin(Stdio::from(locale_cli_cut.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let locale_reader = BufReader::new(locale_cli_sort.stdout.expect("could not get stdout"));

    let lang_data_buffer = gtk::TextBuffer::builder()
        .build();

    for locale in locale_reader.lines() {
        let locale = locale.unwrap();
        let locale_clone = locale.clone();
        let locale_checkbutton = gtk::CheckButton::builder()
            .label(locale.clone())
            .build();
        locale_checkbutton.set_group(Some(&null_checkbutton));
        language_selection_expander_row_viewport_box.append(&locale_checkbutton);
        locale_checkbutton.connect_toggled(clone!(@weak locale_checkbutton, @weak language_selection_expander_row, @weak bottom_next_button, @weak lang_data_buffer => move |_| {
            if locale_checkbutton.is_active() == true {
                language_selection_expander_row.set_title(&locale);
                bottom_next_button.set_sensitive(true);
                lang_data_buffer.set_text(&locale);
            }
        }));
        if current_locale.contains(&(locale_clone)) {
            locale_checkbutton.set_active(true);
        }
    }

    // / language_selection_box appends
    //// add text and and entry to language page selections
    language_selection_box.append(&language_selection_text);
    language_selection_box.append(&language_selection_expander_row_viewport_listbox);

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

    let lang_data_buffer_clone = lang_data_buffer.clone();

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        if Path::new("/tmp/pika-installer-gtk4-lang.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-lang.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-lang.txt");
        }
        fs::write("/tmp/pika-installer-gtk4-lang.txt", lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
        Command::new("sudo")
        .arg("localectl")
        .arg("set-locale")
        .arg("LANG=".to_owned() + &lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string() + ".UTF-8")
        .spawn()
        .expect("locale failed to start");
        content_stack.set_visible_child_name("eula_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("welcome_page")
    }));
}
