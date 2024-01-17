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

pub fn eula_page(content_stack: &gtk::Stack) {

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

   // the header box for the eula page
   let eula_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the eula page
   let eula_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the eula page
   let eula_header_text = gtk::Label::builder()
       .label("PikaOS User license Agreement")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    eula_header_text.add_css_class("header_sized_text");

   // the header icon for the eula icon
   let eula_header_icon = gtk::Image::builder()
       .icon_name("error-correct")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();

   // make eula selection box for choosing installation or live media 
   let eula_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

    // / eula_header_box appends
    //// Add the eula page header text and icon
    eula_header_box.append(&eula_header_text);
    eula_header_box.append(&eula_header_icon);
    
    // / eula_main_box appends
    //// Add the eula header to eula main box
    eula_main_box.append(&eula_header_box);
    //// Add the eula selection/page content box to eula main box
    eula_main_box.append(&eula_selection_box);

    // text above eula selection box
    let eula_selection_text = gtk::Label::builder()
        .label("Please carefully read and consent to the follwing before installing PikaOS:")
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    eula_selection_text.add_css_class("medium_sized_text");

    let eula_buffer = gtk::TextBuffer::builder()
        .text("WE OWN YOU\nWE OWN YOUR SOUL\nWE OWN YOUR WIFE\nWE OWN YOUR FIRST BORN\nWE OWN YOUR HOUSE\nWE OWN YOUR FOOD\nWE OWN YOUR CAR\nWE WILL TRACK YOU\nWE WILL FIND YOU\nTHEN WE WILL KILL YOU")
        .build();

    let eula_selection_text_view = gtk::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .editable(false)
        .buffer(&eula_buffer)
        .build();

    let eula_accept_checkbutton = gtk::CheckButton::builder()
        .label("I Agree and Accept the User license Agreement")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    // / eula_selection_box appends
    //// add text and and entry to eula page selections
    eula_selection_box.append(&eula_selection_text);
    eula_selection_box.append(&eula_selection_text_view);
    eula_selection_box.append(&eula_accept_checkbutton);
    
    // / eula_header_box appends
    //// Add the eula page header text and icon
    eula_header_box.append(&eula_header_text);
    eula_header_box.append(&eula_header_icon);
    
    // / eula_main_box appends
    //// Add the eula header to eula main box
    eula_main_box.append(&eula_header_box);
    //// Add the eula selection/page content box to eula main box
    eula_main_box.append(&eula_selection_box);

    eula_main_box.append(&bottom_box);
    
    // / Content stack appends
    //// Add the eula_main_box as page: eula_page, Give it nice title
    content_stack.add_titled(&eula_main_box, Some("eula_page"), "EULA");

    let eula_accept_checkbutton_clone = eula_accept_checkbutton.clone();
    let bottom_next_button_clone = bottom_next_button.clone();
    eula_accept_checkbutton.connect_toggled(move |_| {
        if eula_accept_checkbutton_clone.is_active() == true {
            bottom_next_button_clone.set_sensitive(true);
        } else {
            bottom_next_button_clone.set_sensitive(false)
        }
    });

    let content_stack_clone = content_stack.clone();
    let content_stack_clone2 = content_stack.clone();
    bottom_next_button.connect_clicked(move |_| {
        content_stack_clone.set_visible_child_name("keyboard_page")
    });
    bottom_back_button.connect_clicked(move |_| {
        content_stack_clone2.set_visible_child_name("language_page")
    });

}
