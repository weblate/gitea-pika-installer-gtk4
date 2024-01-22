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

use std::fs;
use std::path::Path;

pub fn done_page(done_main_box: &gtk::Box, content_stack: &gtk::Stack, window: &adw::ApplicationWindow) {
   // the header box for the installation_successful page
   let done_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the installation_successful page
   let done_header_text = gtk::Label::builder()
       .label("We're done!")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    done_header_text.add_css_class("header_sized_text");

   // the header icon for the installation_successful icon
   let done_header_icon = gtk::Image::builder()
       .icon_name("debian-swirl")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();
   
   // the header box for the installation_successful page
   let installation_successful_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // make installation_successful selection box for choosing installation or live media 
   let installation_successful_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .margin_bottom(15)
       .margin_top(15)
       .margin_start(15)
       .margin_end(15)
       .build();

   
   let installation_successful_text = gtk::Label::builder()
        .vexpand(true)
        .hexpand(true)
        .label("The installation of PikaOS has been completed sucessfully.")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    installation_successful_text.add_css_class("header_sized_text");
   
   let installation_successful_exit_button = gtk::Button::builder()
        .label("Exit")
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    // / installation_successful_selection_box appends
    
    // / done_header_box appends
    //// Add the installation_successful page header text and icon
    done_header_box.append(&done_header_text);
    done_header_box.append(&done_header_icon);
    
    // / installation_successful_main_box appends
    //// Add the installation_successful header to installation_successful main box
    installation_successful_main_box.append(&done_header_box);
    //// Add the installation_successful selection/page content box to installation_successful main box
    installation_successful_main_box.append(&installation_successful_selection_box);
    
    // Start Appending widgets to boxes

    // / installation_successful_selection_box appends
    //// add live and install media button to installation_successful page selections
    installation_successful_selection_box.append(&installation_successful_text);
    installation_successful_selection_box.append(&installation_successful_exit_button);
    
    // / done_header_box appends
    //// Add the installation_successful page header text and icon
    done_header_box.append(&done_header_text);
    done_header_box.append(&done_header_icon);
    
    // / installation_successful_main_box appends
    //// Add the installation_successful selection/page content box to installation_successful main box
    installation_successful_main_box.append(&installation_successful_selection_box);

    done_main_box.append(&done_header_box);
    if Path::new("/tmp/pika-installer-gtk4-successful.txt").exists() {
        done_main_box.append(&installation_successful_main_box)
    }

    // / Content stack appends
    //// Add the installation_successful_main_box as page: installation_successful_page, Give it nice title

    installation_successful_exit_button.connect_clicked(clone!(@weak window => move |_| window.close()));
}