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


pub fn manual_partitioning(window: &adw::ApplicationWindow, partitioning_stack: &gtk::Stack, bottom_next_button: &gtk::Button) -> (gtk::TextBuffer, gtk::TextBuffer, adw::PasswordEntryRow) {
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

    partition_method_manual_chroot_dir_entry.connect_changed(clone!(@weak bottom_next_button, @weak partition_method_manual_luks_password_entry, @weak partition_method_manual_luks_error_label, @weak partition_method_manual_chroot_dir_entry, @weak partition_method_manual_chroot_error_label, @weak partition_method_manual_boot_error_label, @weak partition_method_manual_efi_error_label, @weak partition_method_manual_target_buffer, @weak partition_method_manual_luks_buffer  => move |_| {
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
        let home_not_root_cli = Command::new("sudo")
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
                    let check_home_encryption_cli = Command::new("sudo")
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
            gio::spawn_blocking(clone!(@strong custom_root_mountpoint, @strong luks_passwd, @strong custom_root_mountpoint => move || {
                    let luks_check_cli = Command::new("sudo")
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
        let home_not_boot_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("home_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let root_not_boot_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("root_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let boot_not_efi_cli = Command::new("sudo")
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
        let root_not_efi_cli = Command::new("sudo")
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

    partition_method_manual_luks_password_entry.connect_changed(clone!(@weak bottom_next_button, @weak partition_method_manual_chroot_dir_entry, @weak partition_method_manual_luks_password_entry, @weak partition_method_manual_luks_error_label, @weak partition_method_manual_chroot_error_label, @weak partition_method_manual_boot_error_label, @weak partition_method_manual_efi_error_label, @weak partition_method_manual_target_buffer, @weak partition_method_manual_luks_buffer  => move |_| {
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
        let home_not_root_cli = Command::new("sudo")
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
                    let check_home_encryption_cli = Command::new("sudo")
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
                    let luks_check_cli = Command::new("sudo")
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
        let home_not_boot_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("home_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let root_not_boot_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("root_not_boot")
            .arg(custom_root_mountpoint.clone())
            .output()
            .expect("failed to execute process");
        let boot_not_efi_cli = Command::new("sudo")
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
        let root_not_efi_cli = Command::new("sudo")
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

    partitioning_stack.add_titled(&partition_method_manual_main_box, Some("partition_method_manual_page"), "partition_method_manual_page");

    return(partition_method_manual_target_buffer, partition_method_manual_luks_buffer, partition_method_manual_luks_password_entry)
}