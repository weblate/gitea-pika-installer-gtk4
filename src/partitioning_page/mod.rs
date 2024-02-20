// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
use glob::glob;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;



use crate::automatic_partitioning::automatic_partitioning;
use crate::install_page::install_page;
use crate::manual_partitioning::manual_partitioning;

use std::{fs};
use std::path::Path;

use std::cell::RefCell;
use std::rc::Rc;

use duct::*;

use crate::manual_partitioning;

use manual_partitioning::DriveMount;

pub fn partitioning_page(
    partitioning_main_box: &gtk::Box,
    done_main_box: &gtk::Box,
    install_main_box: &gtk::Box,
    content_stack: &gtk::Stack,
    window: &adw::ApplicationWindow,
) {

    let manual_drive_mount_array: Rc<RefCell<Vec<DriveMount>>> = Default::default();

    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();

    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label(t!("back"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    let bottom_next_button = gtk::Button::builder()
        .label(t!("next"))
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

    // the header box for the partitioning page
    let partitioning_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the partitioning page
    let partitioning_header_text = gtk::Label::builder()
        .label(t!("choose_install_method"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    partitioning_header_text.add_css_class("header_sized_text");

    // the header icon for the partitioning icon
    let partitioning_header_icon = gtk::Image::builder()
        .icon_name("media-floppy")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // a stack for the 2 partitioning methods
    let partitioning_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::SlideLeftRight)
        .build();

    let partitioning_method_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // make partitioning selection box for choosing installation or live media
    let partitioning_selection_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(200)
        .build();

    let manual_method_button_content_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(30)
        .margin_bottom(30)
        .build();

    let manual_method_button_content_image = gtk::Image::builder()
        .icon_name("org.gnome.Settings")
        .pixel_size(128)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let manual_method_button_content_text = gtk::Label::builder()
        .label(t!("manual_partition_drive"))
        .margin_top(0)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    manual_method_button_content_text.add_css_class("medium_sized_text");

    let automatic_method_button_content_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(20)
        .margin_bottom(20)
        .margin_end(15)
        .margin_start(15)
        .build();

    let automatic_method_button_content_image = gtk::Image::builder()
        .icon_name("builder")
        .pixel_size(128)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let automatic_method_button_content_text = gtk::Label::builder()
        .label(t!("auto_partition_drive"))
        .margin_top(0)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    automatic_method_button_content_text.add_css_class("medium_sized_text");

    let manual_method_button = gtk::Button::builder()
        .child(&manual_method_button_content_box)
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::End)
        .valign(gtk::Align::Center)
        .build();

    let automatic_method_button = gtk::Button::builder()
        .child(&automatic_method_button_content_box)
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Center)
        .build();

    // / manual_method_button_content_box appends
    //// add image and text to the manual_method_button
    manual_method_button_content_box.append(&manual_method_button_content_image);
    manual_method_button_content_box.append(&manual_method_button_content_text);

    // / automatic_method_button_content_box appends
    //// add image and text to the automatic_method_button
    automatic_method_button_content_box.append(&automatic_method_button_content_image);
    automatic_method_button_content_box.append(&automatic_method_button_content_text);

    // / partitioning_selection_box appends
    //// add live and install media button to partitioning page selections
    partitioning_selection_box.append(&manual_method_button);
    partitioning_selection_box.append(&automatic_method_button);

    // / partitioning_header_box appends
    //// Add the partitioning page header text and icon
    partitioning_header_box.append(&partitioning_header_text);
    partitioning_header_box.append(&partitioning_header_icon);

    partitioning_method_main_box.append(&partitioning_header_box);
    partitioning_method_main_box.append(&partitioning_selection_box);

    manual_method_button_content_box.append(&manual_method_button_content_image);

    partitioning_stack.add_titled(
        &partitioning_method_main_box,
        Some("partition_method_select_page"),
        "partition_method_select_page",
    );
    let partitioning_page_automatic_partitioning =
        automatic_partitioning(&partitioning_stack, &bottom_next_button);
    let partitioning_page_manual_partitioning = manual_partitioning(
        &partitioning_stack,
        &bottom_next_button,
        &manual_drive_mount_array,
    );

    // add everything to the main box
    partitioning_main_box.append(&partitioning_stack);
    partitioning_main_box.append(&bottom_box);

    automatic_method_button.connect_clicked(clone!(@weak partitioning_stack => move |_| partitioning_stack.set_visible_child_name("partition_method_automatic_page")));
    manual_method_button.connect_clicked(clone!(@weak partitioning_stack => move |_| partitioning_stack.set_visible_child_name("partition_method_manual_page")));

    let partition_method_automatic_target_buffer_clone =
        partitioning_page_automatic_partitioning.0.clone();

    let partition_method_automatic_luks_buffer_clone =
        partitioning_page_automatic_partitioning.1.clone();

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("install_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack, @weak partitioning_stack, @weak partitioning_main_box, @weak bottom_next_button => move |_| {
        content_stack.set_visible_child_name("keyboard_page");
        partitioning_stack.set_visible_child_name("partition_method_select_page");
        bottom_next_button.set_sensitive(false);
        partitioning_page_manual_partitioning.emit_clicked();
    }));

    bottom_next_button.connect_clicked(clone!(@weak content_stack, @weak partitioning_stack, @weak install_main_box, @weak window, @weak done_main_box => move |_| {
        if Path::new("/tmp/pika-installer-gtk4-target-auto.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-target-auto.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-target-auto.txt");
        }
        if Path::new("/tmp/pika-installer-gtk4-target-manual.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-target-manual.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-target-manual.txt");
        }
        if Path::new("/tmp/pika-installer-gtk4-target-automatic-luks.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-target-automatic-luks.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-target-manual.txt");
        }
        if Path::new("/tmp/pika-installer-gtk4-target-manual-luks.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-target-manual-luks.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-target-manual.txt");
        }
        if Path::new("/tmp/pika-installer-gtk4-fail.txt").exists() {
            let _ = cmd!("bash","-c","sudo rm -rfv /tmp/pika-installer-gtk4-fail.txt").run();
        }
        if Path::new("/tmp/pika-installer-gtk4-successful.txt").exists() {
            let _ = cmd!("bash","-c","sudo rm -rfv /tmp/pika-installer-gtk4-successful.txt").run();
        }
        for _status_file in glob("/tmp/pika-installer-gtk4-status*").expect("Failed to read glob pattern") {
            let _ = cmd!("bash","-c","sudo rm -rfv /tmp/pika-installer-gtk4-status*").run();
        }
        for partition_file in glob("/tmp/pika-installer-gtk4-target-manual-p*").expect("Failed to read glob pattern") {
            let partition_file = partition_file.unwrap();
            fs::remove_file(&partition_file).expect(&partition_file.to_str().unwrap());
        }
        for luks_file in glob("/tmp/pika-installer-gtk4-target-manual-luks-p*").expect("Failed to read glob pattern") {
            let luks_file = luks_file.unwrap();
            fs::remove_file(&luks_file).expect(&luks_file.to_str().unwrap());
        }
        if partitioning_stack.visible_child_name() == Some(GString::from_string_unchecked("partition_method_automatic_page".into())) {
            fs::write("/tmp/pika-installer-gtk4-target-auto.txt", partition_method_automatic_target_buffer_clone.text(&partition_method_automatic_target_buffer_clone.bounds().0, &partition_method_automatic_target_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
            let automatic_luks_result = partition_method_automatic_luks_buffer_clone.text(&partition_method_automatic_luks_buffer_clone.bounds().0, &partition_method_automatic_luks_buffer_clone.bounds().1, true).to_string();
            if automatic_luks_result.is_empty() {
               //
            } else {
               let _ = fs::write("/tmp/pika-installer-gtk4-target-automatic-luks.txt", automatic_luks_result);
            }
            install_page(&done_main_box, &install_main_box, &content_stack, &window, &manual_drive_mount_array);
            content_stack.set_visible_child_name("install_page");
        } else {
            fs::write("/tmp/pika-installer-gtk4-target-manual.txt", "").expect("Unable to write file");
            install_page(&done_main_box, &install_main_box, &content_stack, &window, &manual_drive_mount_array);
            content_stack.set_visible_child_name("install_page");
        }
    }));
}
