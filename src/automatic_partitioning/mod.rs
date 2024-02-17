// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

pub fn automatic_partitioning(
    partitioning_stack: &gtk::Stack,
    bottom_next_button: &gtk::Button,
) -> (gtk::TextBuffer, gtk::TextBuffer) {
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

    let null_checkbutton = gtk::CheckButton::builder().build();

    let devices_selection_expander_row_viewport =
        gtk::ScrolledWindow::builder().height_request(200).build();

    let devices_selection_expander_row_viewport_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    devices_selection_expander_row_viewport
        .set_child(Some(&devices_selection_expander_row_viewport_box));

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

    let partition_method_automatic_get_devices_cli = Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_devices")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));
    let partition_method_automatic_get_devices_reader = BufReader::new(
        partition_method_automatic_get_devices_cli
            .stdout
            .expect("could not get stdout"),
    );

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

    let partition_method_automatic_target_buffer = gtk::TextBuffer::builder().build();

    let partition_method_automatic_luks_buffer = gtk::TextBuffer::builder().build();

    for device in partition_method_automatic_get_devices_reader.lines() {
        let device = device.unwrap();
        let device_size_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("get_block_size")
            .arg(device.clone())
            .output()
            .expect("failed to execute process");
        let device_size = String::from_utf8(device_size_cli.stdout)
            .expect("Failed to create float")
            .trim()
            .parse::<f64>()
            .unwrap();
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

    partitioning_stack.add_titled(
        &partition_method_automatic_main_box,
        Some("partition_method_automatic_page"),
        "partition_method_automatic_page",
    );

    return (
        partition_method_automatic_target_buffer,
        partition_method_automatic_luks_buffer,
    );
}
