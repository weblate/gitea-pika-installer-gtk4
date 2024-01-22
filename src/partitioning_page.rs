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

use crate::install_page;

pub fn partitioning_page(done_main_box: &gtk::Box, install_main_box: &gtk::Box ,content_stack: &gtk::Stack, window: &adw::ApplicationWindow) {
    
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

   // the header box for the partitioning page
   let partitioning_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the partitioning page
   let partitioning_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the partitioning page
   let partitioning_header_text = gtk::Label::builder()
       .label("Choose an install method")
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
       .icon_name("input-tablet")
       .pixel_size(128)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(15)
       .build();

   let manual_method_button_content_text = gtk::Label::builder()
       .label("Manually Partition The Drive")
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
       .icon_name("media-playlist-shuffle")
       .pixel_size(128)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(15)
       .build();

   let automatic_method_button_content_text = gtk::Label::builder()
       .label("Automatically Partition\nThe Drive")
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
    
    // Automatic Partitioning Yard
    let partition_method_automatic_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(15)
        .margin_top(15)
        .margin_end(15)
        .margin_start(15)
        .build();

    let partition_method_automatic_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the partitioning page
    let partition_method_automatic_header_text = gtk::Label::builder()
        .label("Automatic Partitioning Installer")
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    partition_method_automatic_header_text.add_css_class("header_sized_text");

    // the header icon for the partitioning icon
    let partition_method_automatic_header_icon = gtk::Image::builder()
        .icon_name("media-playlist-shuffle")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    let partition_method_automatic_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let partition_method_automatic_selection_text = gtk::Label::builder()
            .label("Choose the Drive you want to install PikaOS on\nNote: This will erase the entire drive backup your data!")
            .justify(Justification::Center)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .build();
    partition_method_automatic_selection_text.add_css_class("medium_sized_text");

    let devices_selection_expander_row = adw::ExpanderRow::builder()
        .title("No disk selected for selection")
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .build();

    let devices_selection_expander_row_viewport = gtk::ScrolledWindow::builder()
        .height_request(200)
        .build();

    let devices_selection_expander_row_viewport_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();

    devices_selection_expander_row_viewport.set_child(Some(&devices_selection_expander_row_viewport_box));

    let devices_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
            .selection_mode(SelectionMode::None)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .build();
    devices_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    devices_selection_expander_row_viewport_listbox.append(&devices_selection_expander_row);

    devices_selection_expander_row.add_row(&devices_selection_expander_row_viewport);

    let partition_method_automatic_get_devices_cli = Command::new("pkexec")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_devices")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));
    let partition_method_automatic_get_devices_reader = BufReader::new(partition_method_automatic_get_devices_cli.stdout.expect("could not get stdout"));

    let partition_method_automatic_disk_error_label = gtk::Label::builder()
        .label("No Disk specified.")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .build();
    partition_method_automatic_disk_error_label.add_css_class("small_error_text");

    let partition_method_automatic_luks_error_label = gtk::Label::builder()
        .label("LUKS Encryption Enabled but no password provided.")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_automatic_luks_error_label.add_css_class("small_error_text");

    let partition_method_automatic_luks_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    let partition_method_automatic_luks_checkbutton = gtk::CheckButton::builder()
        .label("Enable LUKS2 Disk Encryption")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let partition_method_automatic_luks_listbox = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();
    partition_method_automatic_luks_listbox.add_css_class("boxed-list");

    let partition_method_automatic_luks_password_entry = adw::PasswordEntryRow::builder()
        .title("LUKS Password")
        .hexpand(true)
        .sensitive(false)
        .build();

    let partition_method_automatic_target_buffer = gtk::TextBuffer::builder()
        .build();

    let partition_method_automatic_luks_buffer = gtk::TextBuffer::builder()
        .build();

    for device in partition_method_automatic_get_devices_reader.lines() {
        let device = device.unwrap();
        let device_size_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("get_block_size")
            .arg(device.clone())
            .output()
            .expect("failed to execute process");
        let device_size = String::from_utf8(device_size_cli.stdout).expect("Failed to create float").trim().parse::<f64>().unwrap();
        let device_button = gtk::CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        device_button.set_group(Some(&null_checkbutton));
        let device_row = adw::ActionRow::builder()
            .activatable_widget(&device_button)
            .title(device.clone())
            .subtitle(pretty_bytes::converter::convert(device_size))
            .build();
        device_row.add_prefix(&device_button);
        devices_selection_expander_row_viewport_box.append(&device_row);
        device_button.connect_toggled(clone!(@weak device_button,@weak partition_method_automatic_luks_password_entry, @weak devices_selection_expander_row, @weak bottom_next_button, @weak partition_method_automatic_disk_error_label, @weak partition_method_automatic_luks_error_label, @weak partition_method_automatic_luks_checkbutton, @weak partition_method_automatic_target_buffer, @weak partition_method_automatic_luks_buffer => move |_| {
            if device_button.is_active() == true {
                devices_selection_expander_row.set_title(&device);
                if device_size > 39000000000.0 {
                    partition_method_automatic_disk_error_label.set_visible(false);
                    if partition_method_automatic_luks_checkbutton.is_active() == true {
                        if partition_method_automatic_luks_error_label.get_visible() {
                            //
                        } else {
                            bottom_next_button.set_sensitive(true);
                        }
                    }  else {
                        partition_method_automatic_target_buffer.set_text(&device);
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        bottom_next_button.set_sensitive(true);
                    } 
                } else {
                    partition_method_automatic_disk_error_label.set_visible(true);
                    partition_method_automatic_disk_error_label.set_label("Disk Size too small, PikaOS needs 40GB Disk");
                    bottom_next_button.set_sensitive(false);
                }
            }
        }));
    }

        partition_method_automatic_luks_checkbutton.connect_toggled(clone!(@weak partition_method_automatic_luks_checkbutton, @weak partition_method_automatic_luks_password_entry, @weak partition_method_automatic_disk_error_label, @weak partition_method_automatic_luks_error_label, @weak bottom_next_button, @weak partition_method_automatic_target_buffer, @weak partition_method_automatic_luks_buffer => move |_| {
            if partition_method_automatic_luks_checkbutton.is_active() == true {
                partition_method_automatic_luks_password_entry.set_sensitive(true);
                if partition_method_automatic_luks_password_entry.text().to_string().is_empty() {
                    partition_method_automatic_luks_error_label.set_visible(true);
                    bottom_next_button.set_sensitive(false);
                } else {
                    partition_method_automatic_luks_error_label.set_visible(false);
                    if partition_method_automatic_disk_error_label.get_visible() {
                        //
                    } else {
                        bottom_next_button.set_sensitive(true);
                    }
                }
            } else {
                partition_method_automatic_luks_password_entry.set_sensitive(false);
                partition_method_automatic_luks_error_label.set_visible(false);
                if partition_method_automatic_disk_error_label.get_visible() {
                    //
                } else {
                    bottom_next_button.set_sensitive(true);
                }
            }
        }));

    partition_method_automatic_luks_password_entry.connect_changed(clone!(@weak partition_method_automatic_luks_checkbutton, @weak partition_method_automatic_luks_password_entry, @weak partition_method_automatic_disk_error_label, @weak partition_method_automatic_luks_error_label, @weak bottom_next_button, @weak partition_method_automatic_luks_buffer => move |_| {
        if partition_method_automatic_luks_checkbutton.is_active() == true {
            partition_method_automatic_luks_password_entry.set_sensitive(true);
            if partition_method_automatic_luks_password_entry.text().to_string().is_empty() {
                partition_method_automatic_luks_error_label.set_visible(true);
                bottom_next_button.set_sensitive(false);
            } else {
                partition_method_automatic_luks_error_label.set_visible(false);
                if partition_method_automatic_disk_error_label.get_visible() {
                    //
                } else {
                    partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                    bottom_next_button.set_sensitive(true);
                }
            }
        } else {
            partition_method_automatic_luks_password_entry.set_sensitive(false);
            partition_method_automatic_luks_error_label.set_visible(false);
            if partition_method_automatic_disk_error_label.get_visible() {
                //
            } else {
                partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                bottom_next_button.set_sensitive(true);
            }
        }
    }));

    partition_method_automatic_luks_listbox.append(&partition_method_automatic_luks_password_entry);
    partition_method_automatic_luks_box.append(&partition_method_automatic_luks_checkbutton);
    partition_method_automatic_luks_box.append(&partition_method_automatic_luks_listbox);
    partition_method_automatic_header_box.append(&partition_method_automatic_header_text);
    partition_method_automatic_header_box.append(&partition_method_automatic_header_icon);
    partition_method_automatic_selection_box.append(&partition_method_automatic_selection_text);
    partition_method_automatic_main_box.append(&partition_method_automatic_header_box);
    partition_method_automatic_main_box.append(&partition_method_automatic_selection_box);
    partition_method_automatic_main_box.append(&devices_selection_expander_row_viewport_listbox);
    partition_method_automatic_main_box.append(&partition_method_automatic_luks_box);
    partition_method_automatic_main_box.append(&partition_method_automatic_luks_error_label);
    partition_method_automatic_main_box.append(&partition_method_automatic_disk_error_label);

    // Manual Partitioning Yard
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

    let partition_method_manual_selection_text = gtk::Label::builder()
            .label(" - Mount your custom root drive somewhere.\n - Mount all your additional mountpoints relative to it.\n - Make sure to have the the following mountpoints:\n    (CUSTOM_ROOT)/boot ~ 1000mb ext4\n    (CUSTOM_ROOT)/boot/efi ~ 512mb vfat/fat32\n- If (CUSTOM_ROOT)/home has LUKS encryption, make sure to enable it here.\n - Note: This doesn't erase any data automatically, format your drives manually.")
            .halign(gtk::Align::Center)
            .hexpand(true)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .build();
    partition_method_manual_selection_text.add_css_class("medium_sized_text");

    let partition_method_manual_chroot_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    
    let partition_method_manual_chroot_listbox = gtk::ListBox::builder()
        .build();
    partition_method_manual_chroot_listbox.add_css_class("boxed-list");

    let partition_method_manual_chroot_dir_file_dialog = gtk::FileChooserNative::new(
        Some("Open File"),
        gtk::Window::NONE,
        gtk::FileChooserAction::SelectFolder,
        Some("Open"),
        Some("Cancel"),
    );
   
    partition_method_manual_chroot_dir_file_dialog.set_transient_for(Some(window));
    
    let partition_method_manual_chroot_dir_entry = adw::EntryRow::builder()
        .title("Custom Root Mountpoint")
        .hexpand(true)
        .build();

    let partition_method_manual_chroot_dir_button_content = adw::ButtonContent::builder()
        .label("Open")
        .icon_name("folder-open")
        .build();

    let partition_method_manual_chroot_dir_button = gtk::Button::builder()
        .child(&partition_method_manual_chroot_dir_button_content)
        .margin_start(10)
        .build();

    let partition_method_manual_luks_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    let partition_method_manual_luks_listbox = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();
    partition_method_manual_luks_listbox.add_css_class("boxed-list");

    let partition_method_manual_luks_password_entry = adw::PasswordEntryRow::builder()
        .title("LUKS Password")
        .hexpand(true)
        .sensitive(false)
        .build();

    let partition_method_manual_chroot_error_label = gtk::Label::builder()
        .label("No mountpoint specified.")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .build();
    partition_method_manual_chroot_error_label.add_css_class("small_error_text");

    let partition_method_manual_boot_error_label = gtk::Label::builder()
        .label("No boot partition found in chroot, mount (CUSTOM_ROOT)/boot.")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_manual_boot_error_label.add_css_class("small_error_text");

    let partition_method_manual_efi_error_label = gtk::Label::builder()
        .label("No EFI partition found in chroot, mount (CUSTOM_ROOT)/boot/efi.")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_manual_efi_error_label.add_css_class("small_error_text");

    let partition_method_manual_luks_error_label = gtk::Label::builder()
        .label("Home partition encrypted, but no LUKS password provided.")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_manual_luks_error_label.add_css_class("small_error_text");

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
        .valign(Align::End)
        .build();

    partition_method_manual_luks_listbox.append(&partition_method_manual_luks_password_entry);
    partition_method_manual_luks_box.append(&partition_method_manual_luks_listbox);
    partition_method_manual_header_box.append(&partition_method_manual_header_text);
    partition_method_manual_header_box.append(&partition_method_manual_header_icon);
    partition_method_manual_selection_box.append(&partition_method_manual_selection_text);
    partition_method_manual_main_box.append(&partition_method_manual_header_box);
    partition_method_manual_main_box.append(&partition_method_manual_selection_box);
    partition_method_manual_chroot_listbox.append(&partition_method_manual_chroot_dir_entry);
    partition_method_manual_chroot_box.append(&partition_method_manual_chroot_listbox);
    partition_method_manual_chroot_box.append(&partition_method_manual_chroot_dir_button);
    partition_method_manual_gparted_button_content_box.append(&partition_method_manual_gparted_button_content);
    partition_method_manual_gparted_button_content_box.append(&partition_method_manual_gparted_button_content_text);
    partition_method_manual_main_box.append(&partition_method_manual_chroot_box);
    partition_method_manual_main_box.append(&partition_method_manual_luks_box);
    partition_method_manual_main_box.append(&partition_method_manual_luks_error_label);
    partition_method_manual_main_box.append(&partition_method_manual_chroot_error_label);
    partition_method_manual_main_box.append(&partition_method_manual_boot_error_label);
    partition_method_manual_main_box.append(&partition_method_manual_efi_error_label);
    partition_method_manual_main_box.append(&partition_method_manual_gparted_button);

    // clone partition_method_manual_chroot_dir_file_dialog as rust becuase glib breaks it show function for some reason
    let partition_method_manual_chroot_dir_file_dialog_clone = partition_method_manual_chroot_dir_file_dialog.clone();
    partition_method_manual_chroot_dir_button.connect_clicked(move |_| {
        partition_method_manual_chroot_dir_file_dialog_clone.set_visible(true);
    });

    partition_method_manual_chroot_dir_file_dialog.connect_response(clone!(@weak partition_method_manual_chroot_dir_file_dialog, @weak partition_method_manual_chroot_dir_entry => move |_, response| {
        if response == gtk::ResponseType::Accept {
            if partition_method_manual_chroot_dir_file_dialog.file().is_some() {
                partition_method_manual_chroot_dir_entry.set_text(&partition_method_manual_chroot_dir_file_dialog.file().expect("FILE PATHING FAIL").path().expect("PATH STRINGING FAIL").into_os_string().into_string().unwrap());
            }
        }
    }));    

    let partition_method_manual_target_buffer = gtk::TextBuffer::builder()
        .build();

    let partition_method_manual_luks_buffer = gtk::TextBuffer::builder()
        .build(); 

    partition_method_manual_chroot_dir_entry.connect_changed(clone!(@weak bottom_next_button, @weak partition_method_manual_chroot_dir_entry, @weak partition_method_manual_luks_password_entry, @weak partition_method_manual_luks_error_label, @weak partition_method_manual_chroot_error_label, @weak partition_method_manual_boot_error_label, @weak partition_method_automatic_target_buffer, @weak partition_method_automatic_luks_buffer, @weak partition_method_manual_efi_error_label, @weak partition_method_manual_target_buffer, @weak partition_method_manual_luks_buffer  => move |_| {
        bottom_next_button.set_sensitive(false);
        let custom_root_mountpoint = partition_method_manual_chroot_dir_entry.text().to_string();
        // Mountpoint Check
        if custom_root_mountpoint.is_empty() {
            partition_method_manual_chroot_error_label.set_label("No mountpoint specified.");
            partition_method_manual_chroot_error_label.set_visible(true);
        } else if custom_root_mountpoint.contains("/dev") {
            partition_method_manual_chroot_error_label.set_label("This Installer Takes mountpoints not devices.");
            partition_method_manual_chroot_error_label.set_visible(true);
        } else {
            partition_method_manual_chroot_error_label.set_visible(false);
        }
        // Home partition Check
        let home_not_root_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("home_not_root")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        if home_not_root_cli.status.success() {
            // Home encryption Checking
            let (luks_manual_is_encrypt_sender, luks_manual_is_encrypt_receiver) = async_channel::unbounded();
            let luks_manual_is_encrypt_sender = luks_manual_is_encrypt_sender.clone();
            // The long running operation runs now in a separate thread
            gio::spawn_blocking(clone!(@strong custom_root_mountpoint => move || {
                    let check_home_encryption_cli = Command::new("pkexec")
                        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                        .arg("check_home_encryption")
                        .arg(custom_root_mountpoint)
                        .output()
                        .expect("failed to execute process");
                    if check_home_encryption_cli.status.success() {
                        luks_manual_is_encrypt_sender
                        .send_blocking(true)
                        .expect("The channel needs to be open.");
                    } else {
                        luks_manual_is_encrypt_sender
                        .send_blocking(false)
                        .expect("The channel needs to be open.");
                    }
            }));
            let luks_manual_is_encrypt_main_context = MainContext::default();
            // The main loop executes the asynchronous block
            luks_manual_is_encrypt_main_context.spawn_local(clone!(@weak partition_method_manual_luks_password_entry => async move {
                while let Ok(state) = luks_manual_is_encrypt_receiver.recv().await {
                    partition_method_manual_luks_password_entry.set_sensitive(state);
                }
            }));
            // Luks Password Checking
            let luks_passwd = partition_method_manual_luks_password_entry.text().to_string();
            let (luks_manual_password_sender, luks_manual_password_receiver) = async_channel::unbounded();
            let luks_manual_password_sender = luks_manual_password_sender.clone();
            // The long running operation runs now in a separate thread
            gio::spawn_blocking(clone!(@strong custom_root_mountpoint, @strong luks_passwd => move || {
                    let luks_check_cli = Command::new("pkexec")
                        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                        .arg("check_home_luks_passwd")
                        .arg(custom_root_mountpoint)
                        .arg(luks_passwd)
                        .output()
                        .expect("failed to execute process");
                    if luks_check_cli.status.success() {
                        luks_manual_password_sender
                        .send_blocking(false)
                        .expect("The channel needs to be open.");
                    } else {
                        luks_manual_password_sender
                        .send_blocking(true)
                        .expect("The channel needs to be open.");
                    }
            }));
            let luks_manual_password_main_context = MainContext::default();
            // The main loop executes the asynchronous block
            luks_manual_password_main_context.spawn_local(clone!(@weak partition_method_manual_luks_error_label, @weak bottom_next_button => async move {
                while let Ok(state) = luks_manual_password_receiver.recv().await {
                    partition_method_manual_luks_error_label.set_visible(state);
                    bottom_next_button.set_sensitive(!state);
                }
            }));
        }
        // Boot partition Checks
        let home_not_boot_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("home_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let root_not_boot_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("root_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let boot_not_efi_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("boot_not_efi")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");

        if home_not_boot_cli.status.success() && root_not_boot_cli.status.success() && boot_not_efi_cli.status.success() {
            partition_method_manual_boot_error_label.set_visible(false)
        } else {
            if home_not_boot_cli.status.success() {
                partition_method_manual_boot_error_label.set_visible(false);
            } else {
                partition_method_manual_boot_error_label.set_label("the /home and /boot partitions are the same.");
                partition_method_manual_boot_error_label.set_visible(true);
            }
            if boot_not_efi_cli.status.success() {
                partition_method_manual_boot_error_label.set_visible(false);
            } else {
                partition_method_manual_boot_error_label.set_label("the /boot/efi and /boot partitions are the same.");
                partition_method_manual_boot_error_label.set_visible(true);
            }
            if root_not_boot_cli.status.success() {
                partition_method_manual_boot_error_label.set_visible(false);
            } else {
                partition_method_manual_boot_error_label.set_label("No boot partition found in chroot, mount (CUSTOM_ROOT)/boot.");
                partition_method_manual_boot_error_label.set_visible(true);
            }
        }
        // EFI partition Checks
        let root_not_efi_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("root_not_efi")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        if root_not_efi_cli.status.success() {
            partition_method_manual_efi_error_label.set_visible(false);
        } else {
            partition_method_manual_efi_error_label.set_label("No EFI partition found in chroot, mount (CUSTOM_ROOT)/boot/efi.");
            partition_method_manual_efi_error_label.set_visible(true);
        }
        if partition_method_manual_chroot_error_label.get_visible() == false && partition_method_manual_luks_error_label.get_visible() == false && partition_method_manual_boot_error_label.get_visible() == false && partition_method_manual_efi_error_label.get_visible() == false {
            partition_method_manual_target_buffer.set_text(&custom_root_mountpoint);
            bottom_next_button.set_sensitive(true);
        } 
    }));

    partition_method_manual_luks_password_entry.connect_changed(clone!(@weak bottom_next_button, @weak partition_method_manual_chroot_dir_entry, @weak partition_method_manual_luks_password_entry, @weak partition_method_manual_luks_error_label, @weak partition_method_manual_chroot_error_label, @weak partition_method_manual_boot_error_label, @weak partition_method_automatic_target_buffer, @weak partition_method_automatic_luks_buffer, @weak partition_method_manual_efi_error_label, @weak partition_method_manual_target_buffer, @weak partition_method_manual_luks_buffer  => move |_| {
        bottom_next_button.set_sensitive(false);
        let custom_root_mountpoint = partition_method_manual_chroot_dir_entry.text().to_string();
        // Mountpoint Check
        if custom_root_mountpoint.is_empty() {
            partition_method_manual_chroot_error_label.set_label("No mountpoint specified.");
            partition_method_manual_chroot_error_label.set_visible(true);
        } else if custom_root_mountpoint.contains("/dev") {
            partition_method_manual_chroot_error_label.set_label("This Installer Takes mountpoints not devices.");
            partition_method_manual_chroot_error_label.set_visible(true);
        } else {
            partition_method_manual_chroot_error_label.set_visible(false);
        }
        // Home partition Check
        let home_not_root_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("home_not_root")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        if home_not_root_cli.status.success() {
            // Home encryption Checking
            let (luks_manual_is_encrypt_sender, luks_manual_is_encrypt_receiver) = async_channel::unbounded();
            let luks_manual_is_encrypt_sender = luks_manual_is_encrypt_sender.clone();
            // The long running operation runs now in a separate thread
            gio::spawn_blocking(clone!(@strong custom_root_mountpoint => move || {
                    let check_home_encryption_cli = Command::new("pkexec")
                        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                        .arg("check_home_encryption")
                        .arg(custom_root_mountpoint)
                        .output()
                        .expect("failed to execute process");
                    if check_home_encryption_cli.status.success() {
                        luks_manual_is_encrypt_sender
                        .send_blocking(true)
                        .expect("The channel needs to be open.");
                    } else {
                        luks_manual_is_encrypt_sender
                        .send_blocking(false)
                        .expect("The channel needs to be open.");
                    }
            }));
            let luks_manual_is_encrypt_main_context = MainContext::default();
            // The main loop executes the asynchronous block
            luks_manual_is_encrypt_main_context.spawn_local(clone!(@weak partition_method_manual_luks_password_entry => async move {
                while let Ok(state) = luks_manual_is_encrypt_receiver.recv().await {
                    partition_method_manual_luks_password_entry.set_sensitive(state);
                }
            }));
            // Luks Password Checking
            let luks_passwd = partition_method_manual_luks_password_entry.text().to_string();
            let (luks_manual_password_sender, luks_manual_password_receiver) = async_channel::unbounded();
            let luks_manual_password_sender = luks_manual_password_sender.clone();
            // The long running operation runs now in a separate thread
            gio::spawn_blocking(clone!(@strong custom_root_mountpoint, @strong luks_passwd => move || {
                    let luks_check_cli = Command::new("pkexec")
                        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                        .arg("check_home_luks_passwd")
                        .arg(custom_root_mountpoint)
                        .arg(luks_passwd)
                        .output()
                        .expect("failed to execute process");
                    if luks_check_cli.status.success() {
                        luks_manual_password_sender
                        .send_blocking(false)
                        .expect("The channel needs to be open.");
                    } else {
                        luks_manual_password_sender
                        .send_blocking(true)
                        .expect("The channel needs to be open.");
                    }
            }));
            let luks_manual_password_main_context = MainContext::default();
            // The main loop executes the asynchronous block
            luks_manual_password_main_context.spawn_local(clone!(@weak partition_method_manual_luks_error_label, @weak bottom_next_button => async move {
                while let Ok(state) = luks_manual_password_receiver.recv().await {
                    partition_method_manual_luks_error_label.set_visible(state);
                    bottom_next_button.set_sensitive(!state);
                }
            }));
        }
        // Boot partition Checks
        let home_not_boot_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("home_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let root_not_boot_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("root_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let boot_not_efi_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("boot_not_efi")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");

        if home_not_boot_cli.status.success() && root_not_boot_cli.status.success() && boot_not_efi_cli.status.success() {
            partition_method_manual_boot_error_label.set_visible(false)
        } else {
            if home_not_boot_cli.status.success() {
                partition_method_manual_boot_error_label.set_visible(false);
            } else {
                partition_method_manual_boot_error_label.set_label("the /home and /boot partitions are the same.");
                partition_method_manual_boot_error_label.set_visible(true);
            }
            if boot_not_efi_cli.status.success() {
                partition_method_manual_boot_error_label.set_visible(false);
            } else {
                partition_method_manual_boot_error_label.set_label("the /boot/efi and /boot partitions are the same.");
                partition_method_manual_boot_error_label.set_visible(true);
            }
            if root_not_boot_cli.status.success() {
                partition_method_manual_boot_error_label.set_visible(false);
            } else {
                partition_method_manual_boot_error_label.set_label("No boot partition found in chroot, mount (CUSTOM_ROOT)/boot.");
                partition_method_manual_boot_error_label.set_visible(true);
            }
        }
        // EFI partition Checks
        let root_not_efi_cli = Command::new("pkexec")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("root_not_efi")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        if root_not_efi_cli.status.success() {
            partition_method_manual_efi_error_label.set_visible(false);
        } else {
            partition_method_manual_efi_error_label.set_label("No EFI partition found in chroot, mount (CUSTOM_ROOT)/boot/efi.");
            partition_method_manual_efi_error_label.set_visible(true);
        }
        if partition_method_manual_chroot_error_label.get_visible() == false && partition_method_manual_luks_error_label.get_visible() == false && partition_method_manual_boot_error_label.get_visible() == false && partition_method_manual_efi_error_label.get_visible() == false {
            partition_method_manual_target_buffer.set_text(&custom_root_mountpoint);
            bottom_next_button.set_sensitive(true);
        } 
    }));

    partition_method_manual_gparted_button.connect_clicked(move |_| {
        Command::new("gparted")
            .spawn()
            .expect("gparted failed to start");
    });

    /// add all pages to partitioning stack
    partitioning_stack.add_titled(&partitioning_method_main_box, Some("partition_method_select_page"), "partition_method_select_page");
    partitioning_stack.add_titled(&partition_method_automatic_main_box, Some("partition_method_automatic_page"), "partition_method_automatic_page");
    partitioning_stack.add_titled(&partition_method_manual_main_box, Some("partition_method_manual_page"), "partition_method_manual_page");

    // add everything to the main box
    partitioning_main_box.append(&partitioning_stack);
    partitioning_main_box.append(&bottom_box);

    // / Content stack appends
    //// Add the partitioning_main_box as page: partitioning_page, Give it nice title
    content_stack.add_titled(&partitioning_main_box, Some("partitioning_page"), "Partitioning");
    
    automatic_method_button.connect_clicked(clone!(@weak partitioning_stack => move |_| partitioning_stack.set_visible_child_name("partition_method_automatic_page")));
    manual_method_button.connect_clicked(clone!(@weak partitioning_stack => move |_| partitioning_stack.set_visible_child_name("partition_method_manual_page")));

    let partition_method_automatic_target_buffer_clone = partition_method_automatic_target_buffer.clone();

    let partition_method_automatic_luks_buffer_clone = partition_method_automatic_luks_buffer.clone();

    let partition_method_manual_target_buffer_clone = partition_method_manual_target_buffer.clone();

    let partition_method_manual_luks_buffer_clone = partition_method_manual_luks_buffer.clone(); 

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("install_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack, @weak partitioning_stack, @weak partitioning_main_box, @weak bottom_next_button => move |_| {
        content_stack.set_visible_child_name("keyboard_page");
        partitioning_stack.set_visible_child_name("partition_method_select_page");
        bottom_next_button.set_sensitive(false);
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
        if partitioning_stack.visible_child_name() == Some(GString::from_string_unchecked("partition_method_automatic_page".into())) {
            fs::write("/tmp/pika-installer-gtk4-target-auto.txt", partition_method_automatic_target_buffer_clone.text(&partition_method_automatic_target_buffer_clone.bounds().0, &partition_method_automatic_target_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
            let automatic_luks_result = partition_method_automatic_luks_buffer_clone.text(&partition_method_automatic_luks_buffer_clone.bounds().0, &partition_method_automatic_luks_buffer_clone.bounds().1, true).to_string();
            if automatic_luks_result.is_empty() {
                //
            } else {
                fs::write("/tmp/pika-installer-gtk4-target-automatic-luks.txt", automatic_luks_result);
            }
            install_page(&done_main_box, &install_main_box, &content_stack, &window);
            content_stack.set_visible_child_name("install_page");
        } else {
            fs::write("/tmp/pika-installer-gtk4-target-manual.txt", partition_method_manual_target_buffer_clone.text(&partition_method_manual_target_buffer_clone.bounds().0, &partition_method_manual_target_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
            partition_method_manual_luks_buffer_clone.set_text(&partition_method_manual_luks_password_entry.text().to_string());
            let manual_luks_result = partition_method_manual_luks_buffer_clone.text(&partition_method_manual_luks_buffer_clone.bounds().0, &partition_method_manual_luks_buffer_clone.bounds().1, true).to_string();
            if manual_luks_result.is_empty() {
                //
            } else {
                fs::write("/tmp/pika-installer-gtk4-target-manual-luks.txt", manual_luks_result);
            }
            install_page(&done_main_box, &install_main_box, &content_stack, &window);
            content_stack.set_visible_child_name("install_page");
        }
    }));

}