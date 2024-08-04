// Use libraries
use adw::prelude::*;
use adw::*;
use gtk::glib;
use gtk::glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;
use std::{thread};



use std::cell::RefCell;
use std::rc::Rc;

use duct::cmd;
use std::{
    collections::HashSet,
    hash::Hash,
    io::{BufRead, BufReader},
    process::Command,
    time::Duration,
};

use crate::drive_mount_row::DriveMountRow;
use serde::*;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DriveMount {
    pub partition: String,
    pub mountpoint: String,
    pub mountopt: String,
}

fn create_mount_row(
    listbox: &gtk::ListBox,
    partition_method_manual_error_label: &gtk::Label,
    partition_method_manual_valid_label: &gtk::Label,
    manual_drive_mount_array: &Rc<RefCell<Vec<DriveMount>>>,
    part_table_array: &Rc<RefCell<Vec<String>>>,
    _check_part_unique: &Rc<RefCell<bool>>,
) -> DriveMountRow {
    let partition_scroll_child = gtk::ListBox::builder().build();

    let partitions_scroll = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&partition_scroll_child)
        .build();

    // Create row
    let row = DriveMountRow::new_with_scroll(&partitions_scroll);

    let null_checkbutton = gtk::CheckButton::builder().build();

    let part_table_array_ref = part_table_array.borrow_mut();
    for partition in part_table_array_ref.iter() {
        let partition_size_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("get_part_size")
            .arg(partition.clone())
            .output()
            .expect("failed to execute process");
        let partition_fs_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("get_part_fs")
            .arg(partition.clone().replace("mapper/", ""))
            .output()
            .expect("failed to execute process");
        let partition_size = String::from_utf8(partition_size_cli.stdout)
            .expect("Failed to create float")
            .trim()
            .parse::<f64>()
            .unwrap();
        let partition_fs = String::from_utf8(partition_fs_cli.stdout).expect("Failed read stdout");
        let partition_button = gtk::CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        partition_button.set_group(Some(&null_checkbutton));
        let partition_row: adw::ActionRow =
            if partition_fs.contains("crypto_LUKS") || partition_fs.contains("lvm") {
                let prow = adw::ActionRow::builder()
                    .activatable_widget(&partition_button)
                    .title(partition.clone())
                    .name(partition.clone())
                    .subtitle(t!("part_need_mapper"))
                    .build();
                prow
            } else {
                let prow = adw::ActionRow::builder()
                    .activatable_widget(&partition_button)
                    .title(partition.clone())
                    .name(partition.clone())
                    .subtitle(partition_fs + &pretty_bytes::converter::convert(partition_size))
                    .build();
                prow
            };
        partition_row.add_prefix(&partition_button);
        partition_button.connect_toggled(clone!(@weak row, @weak listbox, @weak partition_button, @strong manual_drive_mount_array, @strong partition=> move |_| {
            let mut manual_drive_mount_array_ref = RefCell::borrow_mut(&manual_drive_mount_array);
            if partition_button.is_active() == true {
                row.set_partition(partition.clone());
            } else {
                let manual_drive_mount_array_ref_index = manual_drive_mount_array_ref.iter().position(|x| x.partition == partition.clone()).unwrap();
                manual_drive_mount_array_ref.remove(manual_drive_mount_array_ref_index);
            }
        }));
        partition_scroll_child.append(&partition_row);
    }

    let listbox_clone = listbox.clone();
    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(@strong partition_method_manual_error_label ,@strong partition_method_manual_valid_label, @strong row as _row => move |_row: DriveMountRow| {
                    listbox_clone.remove(&_row);
                    partition_method_manual_error_label.set_label("");
                    partition_method_manual_error_label.set_widget_name("");
                    partition_method_manual_error_label.set_visible(false);
                    partition_method_manual_valid_label.set_label("");
                    partition_method_manual_valid_label.set_visible(false);
        }),
    );

    // Return row
    row
}

//pub fn manual_partitioning(window: &adw::ApplicationWindow, partitioning_stack: &gtk::Stack, bottom_next_button: &gtk::Button) -> (gtk::TextBuffer, gtk::TextBuffer, adw::PasswordEntryRow) {
pub fn manual_partitioning(
    partitioning_stack: &gtk::Stack,
    bottom_next_button: &gtk::Button,
    manual_drive_mount_array: &Rc<RefCell<Vec<DriveMount>>>,
) -> gtk::Button {

    let part_table_array: Rc<RefCell<Vec<String>>> = Default::default();

    let check_part_unique = Rc::new(RefCell::new(true));

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
        .label(t!("manual_part_installer"))
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
        .icon_name("org.gnome.Settings")
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
        .label(t!("use_utility_manual"))
        .build();

    let partition_method_manual_gparted_button_content = adw::ButtonContent::builder()
        .label(t!("open_gparted"))
        .icon_name("gparted")
        .build();

    let partition_method_manual_gparted_button = gtk::Button::builder()
        .child(&partition_method_manual_gparted_button_content_box)
        .halign(Align::Center)
        .valign(Align::Start)
        .build();

    let drive_mounts_adw_listbox = gtk::ListBox::builder().hexpand(true).vexpand(true).build();
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
        .min_content_height(200)
        .min_content_width(200)
        .hexpand(true)
        .vexpand(true)
        .child(&drive_mounts_adw_listbox)
        .build();

    let partition_method_manual_selection_text = gtk::Label::builder()
        .label(t!("manual_part_note"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    partition_method_manual_selection_text.add_css_class("medium_sized_text");

    let partition_refresh_button = gtk::Button::builder()
        .label(t!("refresh_part_table"))
        .halign(gtk::Align::End)
        .build();
    partition_refresh_button.add_css_class("destructive-action");

    let fstab_valid_check = gtk::Button::builder()
        .label(t!("validate_fs_table"))
        .halign(gtk::Align::Start)
        .build();
    fstab_valid_check.add_css_class("valid-action");

    let drive_mount_add_button = gtk::Button::builder()
        .icon_name("list-add")
        .vexpand(true)
        .hexpand(true)
        .build();

    let partition_method_manual_error_label = gtk::Label::builder()
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_manual_error_label.add_css_class("small_error_text");

    let partition_method_manual_valid_label = gtk::Label::builder()
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_manual_valid_label.add_css_class("small_valid_text");

    let partition_method_manual_warn_label = gtk::Label::builder()
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_manual_warn_label.add_css_class("small_warn_text");

    partition_method_manual_header_box.append(&partition_method_manual_header_text);
    partition_method_manual_header_box.append(&partition_method_manual_header_icon);
    partition_method_manual_selection_box.append(&partition_method_manual_selection_text);
    partition_method_manual_selection_box.append(&partition_refresh_button);
    partition_method_manual_main_box.append(&partition_method_manual_header_box);
    partition_method_manual_main_box.append(&partition_method_manual_selection_box);
    partition_method_manual_gparted_button_content_box
        .append(&partition_method_manual_gparted_button_content);
    partition_method_manual_gparted_button_content_box
        .append(&partition_method_manual_gparted_button_content_text);
    partition_method_manual_main_box.append(&partition_method_manual_gparted_button);
    drive_mounts_adw_listbox.append(&drive_mount_add_button);
    partition_method_manual_main_box.append(&drive_mounts_viewport);
    partition_method_manual_main_box.append(&fstab_valid_check);
    partition_method_manual_main_box.append(&partition_method_manual_error_label);
    partition_method_manual_main_box.append(&partition_method_manual_valid_label);
    partition_method_manual_main_box.append(&partition_method_manual_warn_label);

    fstab_valid_check.connect_clicked(clone!(@weak partition_method_manual_error_label, @weak partition_method_manual_valid_label, @strong manual_drive_mount_array, @strong  check_part_unique => move |_| {
        partition_err_check(&partition_method_manual_error_label, &partition_method_manual_valid_label, &manual_drive_mount_array);
    }));

    partition_refresh_button.connect_clicked(clone!(@weak partition_method_manual_error_label, @weak partition_method_manual_valid_label,@weak drive_mounts_adw_listbox,@strong part_table_array, @strong manual_drive_mount_array => move |_| {
        partition_method_manual_error_label.set_label("");
        partition_method_manual_error_label.set_widget_name("");
        partition_method_manual_error_label.set_visible(false);
        partition_method_manual_valid_label.set_label("");
        partition_method_manual_valid_label.set_visible(false);
        while let Some(row) = drive_mounts_adw_listbox.last_child() {
                if row.widget_name() == "DriveMountRow" {
                    drive_mounts_adw_listbox.remove(&row);
                } else {
                break
            }
        }
        let partition_method_manual_get_partitions_lines = BufReader::new(cmd!("bash", "-c", "sudo /usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions").reader().unwrap()).lines();
        let mut part_table_array_ref = part_table_array.borrow_mut();
        part_table_array_ref.clear();
        let mut manual_drive_mount_array_ref = manual_drive_mount_array.borrow_mut();
        manual_drive_mount_array_ref.clear();
        for partition in partition_method_manual_get_partitions_lines {
            part_table_array_ref.push(partition.unwrap());
        }
    }));
    partition_refresh_button.emit_clicked();

    partition_method_manual_gparted_button.connect_clicked(move |_| {
        Command::new("gparted")
            .spawn()
            .expect("gparted failed to start");
    });

    drive_mount_add_button.connect_clicked(clone!(@weak partition_method_manual_error_label, @weak partition_method_manual_valid_label ,@weak drive_mounts_adw_listbox, @strong manual_drive_mount_array, @strong part_table_array, @strong  check_part_unique => move |_| {
        drive_mounts_adw_listbox.append(&create_mount_row(&drive_mounts_adw_listbox, &partition_method_manual_error_label, &partition_method_manual_valid_label, &manual_drive_mount_array, &part_table_array,&check_part_unique))
    }));

    let (anti_dup_partition_sender, anti_dup_partition_receiver) = async_channel::unbounded();
    let anti_dup_partition_sender = anti_dup_partition_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || loop {
        thread::sleep(Duration::from_millis(400));
        anti_dup_partition_sender
            .send_blocking(true)
            .expect("The channel needs to be open.");
    });

    let anti_dup_partition_loop_context = MainContext::default();
    anti_dup_partition_loop_context.spawn_local(clone!(@weak partition_method_manual_error_label, @weak partition_method_manual_valid_label ,@weak drive_mounts_adw_listbox, @weak partitioning_stack, @strong manual_drive_mount_array,@weak bottom_next_button, @strong  check_part_unique => async move {
        while let Ok(_state) = anti_dup_partition_receiver.recv().await {
            let mut counter = drive_mounts_adw_listbox.first_child();

            let mut manual_drive_mount_array_ref = manual_drive_mount_array.borrow_mut();

            // usage of while loop
            manual_drive_mount_array_ref.clear();
            while let Some(row) = counter {
                if row.widget_name() == "DriveMountRow" {
                    let row_mount = DriveMount {
                        partition: row.clone().property("partition"),
                        mountpoint: row.clone().property("mountpoint"),
                        mountopt: row.clone().property("mountopt"),
                    };
                    manual_drive_mount_array_ref.push(row_mount);
                }
                counter = row.next_sibling();
            }

            let mut counter = drive_mounts_adw_listbox.first_child();
            while let Some(ref row) = counter {
                if row.widget_name() == "DriveMountRow" {
                    let mut counter_scrw = row.property::<gtk::ScrolledWindow>("partitionscroll").child().unwrap().first_child().unwrap().first_child();
                    while let Some(ref row_scrw) = counter_scrw {
                        if manual_drive_mount_array_ref.iter().any(|e| {
                            if !e.partition.is_empty() {
                                row_scrw.widget_name().contains(&e.partition)
                            } else {
                                return false
                            }
                        }) {

                            if *check_part_unique.borrow_mut() == true {
                                row_scrw.set_sensitive(false)
                            }  else if row_scrw.property::<String>("subtitle").contains(&t!("part_need_mapper").to_string()) {
                                row_scrw.set_sensitive(false)
                            } else {
                                row_scrw.set_sensitive(true)
                            }
                        }
                        else if row_scrw.property::<String>("subtitle").contains(&t!("part_need_mapper").to_string()) {
                            row_scrw.set_sensitive(false)
                        } else {
                            row_scrw.set_sensitive(true)
                        }
                        counter_scrw = row_scrw.next_sibling();
                    }
                }
                counter = row.next_sibling();
            }
            let manual_drive_mount_array_ref_clone = manual_drive_mount_array_ref.clone();

            *check_part_unique.borrow_mut() = true;
            for mountopts in manual_drive_mount_array_ref
                .iter()
                .map(|x| x.mountopt.as_str())
                .collect::<HashSet<&str>>()
            {
                if mountopts.contains("subvol") {
                    *check_part_unique.borrow_mut() = false
                }
            }

            if *check_part_unique.borrow_mut() == false {
                partition_method_manual_warn_label
                    .set_label(&t!("fstab_subvol_warn"));
                partition_method_manual_warn_label.set_visible(true);
            } else {
                partition_method_manual_warn_label.set_visible(false);
            }

            if partitioning_stack.visible_child_name() == Some(GString::from_string_unchecked("partition_method_manual_page".into())) {
                if manual_drive_mount_array_ref_clone.iter().any(|x| {if x.mountpoint == "/" {return true} else {return false}}) && manual_drive_mount_array_ref_clone.iter().any(|x| {if x.mountpoint == "/boot" {return true} else {return false}}) && manual_drive_mount_array_ref_clone.iter().any(|x| {if x.mountpoint == "/boot/efi" {return true} else {return false}}) && !partition_method_manual_error_label.is_visible() && partition_method_manual_valid_label.is_visible() {
                    if !bottom_next_button.is_sensitive() {
                        bottom_next_button.set_sensitive(true);
                    }
                } else {
                    if bottom_next_button.is_sensitive() {
                        bottom_next_button.set_sensitive(false);
                    }
                }
            }
        }
    }));

    partitioning_stack.add_titled(
        &partition_method_manual_main_box,
        Some("partition_method_manual_page"),
        "partition_method_manual_page",
    );

    return partition_refresh_button;
}

fn partition_err_check(
    partition_method_manual_error_label: &gtk::Label,
    partition_method_manual_valid_label: &gtk::Label,
    manual_drive_mount_array: &Rc<RefCell<Vec<DriveMount>>>,
) {
    let mut empty_mountpoint = false;
    let manual_drive_mount_array_ref = manual_drive_mount_array.borrow_mut();
    for mountpoint in manual_drive_mount_array_ref
        .iter()
        .map(|x| x.mountpoint.as_str())
        .collect::<HashSet<&str>>()
    {
        if empty_mountpoint == false {
            if mountpoint.is_empty() {
                empty_mountpoint = true
            }
        }
    }

    let mut empty_partition = false;
    for partition in manual_drive_mount_array_ref
        .iter()
        .map(|x| x.partition.as_str())
        .collect::<HashSet<&str>>()
    {
        if empty_partition == false {
            if partition.is_empty() {
                empty_partition = true
            }
        }
    }

    if empty_mountpoint == false {
        if &partition_method_manual_error_label.widget_name() == "err1" {
            partition_method_manual_error_label.set_visible(false);
        }
        if manual_drive_mount_array_ref.len()
            - manual_drive_mount_array_ref
                .iter()
                .map(|x| x.mountpoint.as_str())
                .collect::<HashSet<&str>>()
                .len()
            > 0
        {
            if !partition_method_manual_error_label.is_visible() {
                partition_method_manual_error_label
                    .set_label(&t!("fstab_multiple_part_mountpoint_err"));
                partition_method_manual_error_label.set_visible(true);
                partition_method_manual_error_label.set_widget_name("err0");
            }
        } else {
            if &partition_method_manual_error_label.widget_name() == "err0" {
                partition_method_manual_error_label.set_visible(false);
            }
        }
    } else {
        if !partition_method_manual_error_label.is_visible() {
            partition_method_manual_error_label.set_label(&t!("fstab_no_mountpoint_err"));
            partition_method_manual_error_label.set_widget_name("err1");
            partition_method_manual_error_label.set_visible(true);
        }
    }

    if empty_partition == true {
        if !partition_method_manual_error_label.is_visible() {
            partition_method_manual_error_label.set_label(&t!("fstab_no_partition_err"));
            partition_method_manual_error_label.set_widget_name("err2");
            partition_method_manual_error_label.set_visible(true);
        }
    } else {
        if partition_method_manual_error_label.widget_name() == "err2" {
            partition_method_manual_error_label.set_visible(false);
        }
    }

    for drivemounts in manual_drive_mount_array_ref
        .iter()
        .map(|x| x)
        .collect::<HashSet<&DriveMount>>()
    {
        if !drivemounts.partition.is_empty() {
            let partition_size_cli = Command::new("sudo")
                .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                .arg("get_part_size")
                .arg(drivemounts.partition.clone())
                .output()
                .expect("failed to execute process");
            let partition_fs_cli = Command::new("sudo")
                .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                .arg("get_part_fs")
                .arg(drivemounts.partition.replace("mapper/", ""))
                .output()
                .expect("failed to execute process");
            let partition_size = String::from_utf8(partition_size_cli.stdout)
                .expect("Failed to create float")
                .trim()
                .parse::<f64>()
                .unwrap();
            let partition_fs = String::from_utf8(partition_fs_cli.stdout)
                .expect("Failed to create string")
                .trim()
                .parse::<String>()
                .unwrap();
            if drivemounts.mountpoint == "/boot/efi" {
                if partition_size < 500000000.0 {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_small_efi_err").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_small_efi_size").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err3");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err3" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
                if partition_fs != "vfat" {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_badfs").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_badfs_efi").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err4");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err4" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
            }
            if drivemounts.mountpoint == "/boot" {
                if partition_size < 1000000000.0 {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_small_boot_err").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_small_boot_size").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err5");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err5" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
                if partition_fs == "vfat" {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_badfs").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_badfs_boot").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err6");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err6" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
            }
            if drivemounts.mountpoint == "/" {
                if partition_size < 25000000000.0 {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_small_root_err").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_small_root_size").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err7")
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err7" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
                if partition_fs == "vfat"
                    || partition_fs == "ntfs"
                    || partition_fs == "swap"
                    || partition_fs == "exfat"
                {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_badfs").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_badfs_root").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err8");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err8" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
            }
            if drivemounts.mountpoint == "/home" {
                if partition_size < 10000000000.0 {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_small_home_err").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_small_home_size").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err9");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err9" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
                if partition_fs == "vfat"
                    || partition_fs == "ntfs"
                    || partition_fs == "swap"
                    || partition_fs == "exfat"
                {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_badfs").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_badfs_home").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err10");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err10" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
            }
            if drivemounts.mountpoint == "[SWAP]" {
                if partition_fs != "swap" {
                    if !partition_method_manual_error_label.is_visible() {
                        partition_method_manual_error_label.set_label(
                            &(t!("fstab_badfs").to_string()
                                + &drivemounts.partition
                                + &t!("fstab_badfs_swap").to_string()),
                        );
                        partition_method_manual_error_label.set_visible(true);
                        partition_method_manual_error_label.set_widget_name("err11");
                    }
                } else {
                    if &partition_method_manual_error_label.widget_name() == "err11" {
                        partition_method_manual_error_label.set_visible(false);
                    }
                }
            }

            if empty_mountpoint == false
                && !drivemounts.mountpoint.starts_with("/")
                && drivemounts.mountpoint != "[SWAP]"
            {
                if !partition_method_manual_error_label.is_visible() {
                    partition_method_manual_error_label.set_label(
                        &(t!("fstab_bad_mountpoint").to_string()
                            + &drivemounts.mountpoint
                            + &t!("fstab_bad_mountpoint_msg").to_string()),
                    );
                    partition_method_manual_error_label.set_visible(true);
                    partition_method_manual_error_label.set_widget_name("err12");
                }
            } else {
                if &partition_method_manual_error_label.widget_name() == "err12" {
                    partition_method_manual_error_label.set_visible(false);
                }
            }
            if !partition_method_manual_error_label.is_visible() {
                partition_method_manual_valid_label.set_label(&t!("fstab_status_valid"));
                partition_method_manual_valid_label.set_visible(true)
            } else {
                partition_method_manual_valid_label.set_visible(false)
            }
        }
    }
}
