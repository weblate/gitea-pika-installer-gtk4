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
use std::str;

pub fn timezone_page(content_stack: &gtk::Stack) {

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

   // the header box for the timezone page
   let timezone_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the timezone page
   let timezone_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the timezone page
   let timezone_header_text = gtk::Label::builder()
       .label("Select a timezone")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    timezone_header_text.add_css_class("header_sized_text");

   // the header icon for the timezone icon
   let timezone_header_icon = gtk::Image::builder()
       .icon_name("clock")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();

   // make timezone selection box for choosing installation or live media 
   let timezone_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

    // / timezone_header_box appends
    //// Add the timezone page header text and icon
    timezone_header_box.append(&timezone_header_text);
    timezone_header_box.append(&timezone_header_icon);
    
    // / timezone_main_box appends
    //// Add the timezone header to timezone main box
    timezone_main_box.append(&timezone_header_box);
    //// Add the timezone selection/page content box to timezone main box
    timezone_main_box.append(&timezone_selection_box);

    // text above timezone selection box
    let timezone_selection_text = gtk::Label::builder()
        .label("Please select a Time Zone for the system to use")
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    timezone_selection_text.add_css_class("medium_sized_text");

    let timezone_selection_expander_row = adw::ExpanderRow::builder()
        .title("No Time Zone selected")
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label("No Time Zone selected")
        .build();

    let timezone_selection_expander_row_viewport = gtk::ScrolledWindow::builder()
        .height_request(200)
        .build();

    let timezone_selection_expander_row_viewport_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();

    let timezone_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
            .selection_mode(SelectionMode::None)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .build();
    timezone_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    timezone_selection_expander_row_viewport_listbox.append(&timezone_selection_expander_row);

    timezone_selection_expander_row_viewport.set_child(Some(&timezone_selection_expander_row_viewport_box));

    timezone_selection_expander_row.add_row(&timezone_selection_expander_row_viewport);

    let current_timezone_cli = Command::new("timedatectl")
        .arg("show")
        .arg("--va")
        .arg("-p")
        .arg("Timezone")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let current_timezone_output = current_timezone_cli.wait_with_output().unwrap();
    let current_timezone = str::from_utf8(&current_timezone_output.stdout).unwrap().trim();                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                

    let timezone_layout_cli = Command::new("timedatectl")
        .arg("list-timezones")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let timezone_layout_stdout = timezone_layout_cli.stdout.expect("could not get stdout");
    let timezone_layout_reader = BufReader::new(timezone_layout_stdout);

    for timezone_layout in timezone_layout_reader.lines() {
        let timezone_layout = timezone_layout.unwrap();
        let timezone_layout_clone = timezone_layout.clone();
        let timezone_layout_checkbutton = gtk::CheckButton::builder()
            .label(timezone_layout.clone())
            .build();
        timezone_layout_checkbutton.set_group(Some(&null_checkbutton));
        timezone_selection_expander_row_viewport_box.append(&timezone_layout_checkbutton); 
        timezone_layout_checkbutton.connect_toggled(clone!(@weak timezone_layout_checkbutton, @weak timezone_selection_expander_row, @weak bottom_next_button => move |_| {
            if timezone_layout_checkbutton.is_active() == true {
                timezone_selection_expander_row.set_title(&timezone_layout);
                bottom_next_button.set_sensitive(true);
            }
        }));
        if current_timezone.contains(&(timezone_layout_clone)) {
            timezone_layout_checkbutton.set_active(true);
        }
    }

    // / timezone_selection_box appends
    //// add text and and entry to timezone page selections
    timezone_selection_box.append(&timezone_selection_text);
    timezone_selection_box.append(&timezone_selection_expander_row_viewport_listbox);
    
    // / timezone_header_box appends
    //// Add the timezone page header text and icon
    timezone_header_box.append(&timezone_header_text);
    timezone_header_box.append(&timezone_header_icon);
    
    // / timezone_main_box appends
    //// Add the timezone header to timezone main box
    timezone_main_box.append(&timezone_header_box);
    //// Add the timezone selection/page content box to timezone main box
    timezone_main_box.append(&timezone_selection_box);

    timezone_main_box.append(&bottom_box);
    
    // / Content stack appends
    //// Add the timezone_main_box as page: timezone_page, Give it nice title
    content_stack.add_titled(&timezone_main_box, Some("timezone_page"), "Time Zone");

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("keyboard_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("eula_page")
    }));

}
