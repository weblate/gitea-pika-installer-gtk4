
// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use crate::config::DISTRO_ICON;


use std::path::Path;
use std::process::Command;

pub fn done_page(done_main_box: &gtk::Box, window: &adw::ApplicationWindow) {

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
        .icon_name(DISTRO_ICON)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // Successful install yard
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

    let installation_successful_big_icon = gtk::Image::builder()
        .icon_name("emblem-default")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .pixel_size(256)
        .margin_top(0)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let installation_successful_text = gtk::Label::builder()
        .label(t!("pika_install_good"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    installation_successful_text.add_css_class("header_sized_text");

    let installation_successful_buttons_line = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(15)
        .margin_top(15)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let installation_successful_exit_button = gtk::Button::builder()
        .label(t!("exit"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();

    let installation_successful_reboot_button = gtk::Button::builder()
        .label(t!("reboot"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();

    // / installation_successful_selection_box appends

    // / installation_successful_main_box appends
    //// Add the installation_successful header to installation_successful main box
    installation_successful_main_box.append(&done_header_box);
    //// Add the installation_successful selection/page content box to installation_successful main box
    installation_successful_main_box.append(&installation_successful_selection_box);

    installation_successful_buttons_line.append(&installation_successful_reboot_button);
    installation_successful_buttons_line.append(&installation_successful_exit_button);

    // Start Appending widgets to boxes

    // / installation_successful_selection_box appends
    //// add live and install media button to installation_successful page selections
    installation_successful_selection_box.append(&installation_successful_big_icon);
    installation_successful_selection_box.append(&installation_successful_text);
    installation_successful_selection_box.append(&installation_successful_buttons_line);

    // / installation_successful_main_box appends
    //// Add the installation_successful selection/page content box to installation_successful main box
    installation_successful_main_box.append(&installation_successful_selection_box);

    installation_successful_exit_button
        .connect_clicked(clone!(@weak window => move |_| window.close()));
    installation_successful_reboot_button.connect_clicked(move |_| {
        Command::new("reboot")
            .spawn()
            .expect("reboot failed to start");
    });

    // Failed install yard
    // the header box for the installation_failed page
    let installation_failed_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // make installation_failed selection box for choosing installation or live media
    let installation_failed_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(15)
        .margin_top(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let installation_failed_big_icon = gtk::Image::builder()
        .icon_name("emblem-default")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .pixel_size(256)
        .margin_top(0)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let installation_failed_text = gtk::Label::builder()
        .label(t!("pika_install_bad"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    installation_failed_text.add_css_class("header_sized_text");

    let installation_failed_buttons_line = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(15)
        .margin_top(15)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let installation_failed_exit_button = gtk::Button::builder()
        .label(t!("exit"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();

    let installation_failed_logs_button = gtk::Button::builder()
        .label(t!("logs"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();

    // / installation_failed_selection_box appends

    // / installation_failed_main_box appends
    //// Add the installation_failed header to installation_failed main box
    installation_failed_main_box.append(&done_header_box);
    //// Add the installation_failed selection/page content box to installation_failed main box
    installation_failed_main_box.append(&installation_failed_selection_box);

    installation_failed_buttons_line.append(&installation_failed_logs_button);
    installation_failed_buttons_line.append(&installation_failed_exit_button);

    // Start Appending widgets to boxes

    // / installation_failed_selection_box appends
    //// add live and install media button to installation_failed page selections
    installation_failed_selection_box.append(&installation_failed_big_icon);
    installation_failed_selection_box.append(&installation_failed_text);
    installation_failed_selection_box.append(&installation_failed_buttons_line);

    // / installation_failed_main_box appends
    //// Add the installation_failed selection/page content box to installation_failed main box
    installation_failed_main_box.append(&installation_failed_selection_box);

    installation_failed_exit_button
        .connect_clicked(clone!(@weak window => move |_| window.close()));
    installation_failed_logs_button.connect_clicked(move |_| {
        Command::new("xdg-open")
            .arg("/tmp/pika-installer-gtk4-log")
            .spawn()
            .expect("xdg-open failed to start");
    });

    // / done_header_box appends
    //// Add the installation_successful page header text and icon
    done_header_box.append(&done_header_text);
    done_header_box.append(&done_header_icon);

    // / done_header_box appends
    //// Add the installation_successful page header text and icon
    done_header_box.append(&done_header_text);
    done_header_box.append(&done_header_icon);

    done_main_box.append(&done_header_box);
    if Path::new("/tmp/pika-installer-gtk4-successful.txt").exists() {
        done_main_box.append(&installation_successful_main_box)
    } else {
        done_main_box.append(&installation_failed_main_box)
    }
}
