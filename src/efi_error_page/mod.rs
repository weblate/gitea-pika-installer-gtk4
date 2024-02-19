// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use gettextrs::gettext;

pub fn efi_error_page(window: &adw::ApplicationWindow, content_stack: &gtk::Stack) {
    // the header box for the efi_error page
    let efi_error_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // the header box for the efi_error page
    let efi_error_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the efi_error page
    let efi_error_header_text = gtk::Label::builder()
        .label(gettext("bad_boot_platfrom"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    efi_error_header_text.add_css_class("header_sized_text");

    // the header icon for the efi_error icon
    let efi_error_header_icon = gtk::Image::builder()
        .icon_name("emblem-error")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // make efi_error selection box for choosing installation or live media
    let efi_error_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(15)
        .margin_top(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let efi_error_text = gtk::Label::builder()
        .vexpand(true)
        .hexpand(true)
        .label(gettext("pika_nowork_csm"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    efi_error_text.add_css_class("big_error_text");

    let exit_button = gtk::Button::builder()
        .label(gettext("exit"))
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    // / efi_error_selection_box appends

    // / efi_error_header_box appends
    //// Add the efi_error page header text and icon
    efi_error_header_box.append(&efi_error_header_text);
    efi_error_header_box.append(&efi_error_header_icon);

    // / efi_error_main_box appends
    //// Add the efi_error header to efi_error main box
    efi_error_main_box.append(&efi_error_header_box);
    //// Add the efi_error selection/page content box to efi_error main box
    efi_error_main_box.append(&efi_error_selection_box);

    // Start Appending widgets to boxes

    // / efi_error_selection_box appends
    //// add live and install media button to efi_error page selections
    efi_error_selection_box.append(&efi_error_text);
    efi_error_selection_box.append(&exit_button);

    // / efi_error_header_box appends
    //// Add the efi_error page header text and icon
    efi_error_header_box.append(&efi_error_header_text);
    efi_error_header_box.append(&efi_error_header_icon);

    // / efi_error_main_box appends
    //// Add the efi_error header to efi_error main box
    efi_error_main_box.append(&efi_error_header_box);
    //// Add the efi_error selection/page content box to efi_error main box
    efi_error_main_box.append(&efi_error_selection_box);

    // / Content stack appends
    //// Add the efi_error_main_box as page: efi_error_page, Give it nice title
    content_stack.add_titled(&efi_error_main_box, Some("efi_error_page"), "Welcome");

    exit_button.connect_clicked(clone!(@weak window => move |_| window.close()));
}
