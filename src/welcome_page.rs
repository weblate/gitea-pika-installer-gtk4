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

pub fn welcome_page(content_stack: &gtk::Stack) {
   // the header box for the welcome page
   let welcome_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the welcome page
   let welcome_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the welcome page
   let welcome_header_text = gtk::Label::builder()
       .label("Welcome to PikaOS")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();

   // the header icon for the welcome icon
   let welcome_header_icon = gtk::Image::builder()
       .icon_name("debian-swirl")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();

   // make welcome selection box for choosing installation or live media 
   let welcome_selection_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .spacing(360)
       .build();

   let live_media_button_content_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   let live_media_button_content_image = gtk::Image::builder()
       .icon_name("drive-optical")
       .pixel_size(128)
       .margin_top(15)
       .margin_bottom(0)
       .margin_start(15)
       .margin_end(15)
       .build();

   let live_media_button = gtk::Button::builder()
       .child(&live_media_button_content_box)
       .vexpand(true)
       .hexpand(true)
       .halign(gtk::Align::End)
       .valign(gtk::Align::Center)
       .build();


   let install_media_button = gtk::Button::builder()
       .vexpand(true)
       .hexpand(true)
       .halign(gtk::Align::Start)
       .valign(gtk::Align::Center)
       .build();

    welcome_header_text.add_css_class("header_sized_text");

    // / live_media_button_content_box appends
    //// add image and text to the live_media_button
    live_media_button_content_box.append(&live_media_button_content_image);

    // / welcome_selection_box appends
    //// add live and install media button to welcome page selections
    welcome_selection_box.append(&live_media_button);
    welcome_selection_box.append(&install_media_button);
    
    // / welcome_header_box appends
    //// Add the welcome page header text and icon
    welcome_header_box.append(&welcome_header_text);
    welcome_header_box.append(&welcome_header_icon);
    
    // / welcome_main_box appends
    //// Add the welcome header to welcome main box
    welcome_main_box.append(&welcome_header_box);
    //// Add the welcome selection/page content box to welcome main box
    welcome_main_box.append(&welcome_selection_box);
    
    welcome_header_text.add_css_class("header_sized_text");
    
    // Start Appending widgets to boxes

    // / live_media_button_content_box appends
    //// add image and text to the live_media_button
    live_media_button_content_box.append(&live_media_button_content_image);


    // / welcome_selection_box appends
    //// add live and install media button to welcome page selections
    welcome_selection_box.append(&live_media_button);
    welcome_selection_box.append(&install_media_button);
    
    // / welcome_header_box appends
    //// Add the welcome page header text and icon
    welcome_header_box.append(&welcome_header_text);
    welcome_header_box.append(&welcome_header_icon);
    
    // / welcome_main_box appends
    //// Add the welcome header to welcome main box
    welcome_main_box.append(&welcome_header_box);
    //// Add the welcome selection/page content box to welcome main box
    welcome_main_box.append(&welcome_selection_box);
    
    // / Content stack appends
    //// Add the welcome_main_box as page: welcome_page, Give it nice title
    content_stack.add_titled(&welcome_main_box, Some("welcome_page"), "Welcome");
}