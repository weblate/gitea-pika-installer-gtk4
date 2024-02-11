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
use pretty_bytes::converter::convert;

use std::thread;
use std::time::*;

use std::fs;
use std::path::Path;
use crate::drive_mount_row::DriveMountRow;

fn create_mount_row(listbox: &gtk::ListBox) -> DriveMountRow {
    // Create row
    let row = DriveMountRow::new();

    let listbox_clone = listbox.clone();

    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(move |_row: DriveMountRow| {
            listbox_clone.remove(&_row)
        }),
    );

    // Return row
    row
}

//pub fn manual_partitioning(window: &adw::ApplicationWindow, partitioning_stack: &gtk::Stack, bottom_next_button: &gtk::Button) -> (gtk::TextBuffer, gtk::TextBuffer, adw::PasswordEntryRow) {
pub fn manual_partitioning(window: &adw::ApplicationWindow, partitioning_stack: &gtk::Stack, bottom_next_button: &gtk::Button) {
    let partition_method_manual_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(15)
        .margin_top(15)
        .margin_end(15)
        .margin_start(15)
        .build();

    let partition_method_manual_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the partitioning page
    let partition_method_manual_header_text = gtk::Label::builder()
        .label("Manual Partitioning Installer")
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    partition_method_manual_header_text.add_css_class("header_sized_text");

    // the header icon for the partitioning icon
    let partition_method_manual_header_icon = gtk::Image::builder()
        .icon_name("input-tablet")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    let partition_method_manual_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let partition_method_manual_gparted_button_content_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let partition_method_manual_gparted_button_content_text = gtk::Label::builder()
        .label("Use this utility to partition/mount/format your drives.")
        .build();

    let partition_method_manual_gparted_button_content = adw::ButtonContent::builder()
        .label("Open GPARTED")
        .icon_name("gparted")
        .build();

    let partition_method_manual_gparted_button = gtk::Button::builder()
        .child(&partition_method_manual_gparted_button_content_box)
        .halign(Align::Center)
        .valign(Align::Start)
        .build();

    let drive_mounts_adw_listbox = gtk::ListBox::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    drive_mounts_adw_listbox.add_css_class("boxed-list");

    let drive_mounts_viewport = gtk::ScrolledWindow::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .margin_top(30)
        .margin_bottom(30)
        .margin_start(30)
        .margin_end(30)
        .propagate_natural_height(true)
        .propagate_natural_width(true)
        .hexpand(true)
        .vexpand(true)
        .child(&drive_mounts_adw_listbox)
        .build();

    let drive_mount_add_button = gtk::Button::builder()
        .icon_name("list-add")
        .build();

    partition_method_manual_header_box.append(&partition_method_manual_header_text);
    partition_method_manual_header_box.append(&partition_method_manual_header_icon);
    partition_method_manual_main_box.append(&partition_method_manual_header_box);
    partition_method_manual_main_box.append(&partition_method_manual_selection_box);
    partition_method_manual_gparted_button_content_box.append(&partition_method_manual_gparted_button_content);
    partition_method_manual_gparted_button_content_box.append(&partition_method_manual_gparted_button_content_text);
    partition_method_manual_main_box.append(&partition_method_manual_gparted_button);
    drive_mounts_adw_listbox.append(&drive_mount_add_button);
    partition_method_manual_main_box.append(&drive_mounts_viewport);

    partition_method_manual_gparted_button.connect_clicked(move |_| {
        Command::new("gparted")
            .spawn()
            .expect("gparted failed to start");
    });

    drive_mount_add_button.connect_clicked(clone!(@weak drive_mounts_adw_listbox => move |_|{
        drive_mounts_adw_listbox.append(&create_mount_row(&drive_mounts_adw_listbox))
    }));

    partitioning_stack.add_titled(&partition_method_manual_main_box, Some("partition_method_manual_page"), "partition_method_manual_page");

    //return(partition_method_manual_target_buffer, partition_method_manual_luks_buffer, partition_method_manual_luks_password_entry)
}
