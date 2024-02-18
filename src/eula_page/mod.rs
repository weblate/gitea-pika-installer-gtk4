// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use gettextrs::{gettext};

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
        .label("Please carefully read and make sure you consent to the following before installing PikaOS:")
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    eula_selection_text.add_css_class("medium_sized_text");

    let eula_buffer = gtk::TextBuffer::builder()
        .text("There are a few things to keep in mind: 
        1 - You understand that this distribution is -NOT- to be considered an ‘Ubuntu Flavor’. 
        2 - This is a hobby distribution, so we will try our best to provide formal support but it will -NOT- be guaranteed. 
        3 - Although PikaOS might provide identical patches and user experience to the Nobara project, we are -NOT- directly a part of them so questions and bug reports should not be sent directly to them (they dont have to deal with it!) 
        4 - While the installer is running DO NOT INTERRUPT IT! or you will end up with a corrupted system. 
        5 - Try to use pikman instead of apt when using the terminal, it is much faster! 
        6 - You understand the xone driver downloads needed binaries locally and does not directly package or distribute any copyrighted firmware or other related data. 
        7 - Automatic partitioning will format all partitons on a drive, so if you want to dualboot make a separate EFI partition for PikaOS and use manual partitioning  
        8 - In case you need the login info for this session: 
         - username: pikaos 
         - password: 
        MEANING: JUST PRESS ENTER")
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

    let eula_selection_text_scroll = gtk::ScrolledWindow::builder()
        .height_request(350)
        .child(&eula_selection_text_view)
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
    eula_selection_box.append(&eula_selection_text_scroll);
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

    eula_accept_checkbutton.connect_toggled(
        clone!(@weak eula_accept_checkbutton, @weak bottom_next_button => move |_| {
            if eula_accept_checkbutton.is_active() == true {
                bottom_next_button.set_sensitive(true);
            } else {
                bottom_next_button.set_sensitive(false)
            }
        }),
    );

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("timezone_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("language_page")
    }));
}
