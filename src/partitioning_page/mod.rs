use crate::{build_ui::{FstabEntry, CrypttabEntry, BlockDevice, SubvolDeclaration, Partition}, installer_stack_page, automatic_partitioning_page, manual_partitioning_page};
use glib::{clone, closure_local, Properties};
use gtk::{gio, glib, prelude::*};
use std::io::BufRead;
use std::{cell::RefCell, rc::Rc};

pub fn partitioning_page(
    main_carousel: &adw::Carousel,
    window: adw::ApplicationWindow,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_fs_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_automatic_luks_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_ratio_refcell: &Rc<RefCell<f64>>,
    partition_method_automatic_seperation_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
    language_changed_action: &gio::SimpleAction,
) {
    let partitioning_page = installer_stack_page::InstallerStackPage::new();
    partitioning_page.set_page_icon("media-floppy-symbolic");
    partitioning_page.set_back_sensitive(true);
    partitioning_page.set_back_visible(true);
    partitioning_page.set_next_visible(false);

    let partitioning_carousel = adw::Carousel::builder()
        .allow_long_swipes(false)
        .allow_mouse_drag(false)
        .allow_scroll_wheel(false)
        .interactive(false)
        .vexpand(true)
        .hexpand(true)
        .build();

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .homogeneous(true)
        .build();

    content_box.add_css_class("linked");

    let manual_method_button_icon = gtk::Image::builder()
        .icon_name("emblem-system-symbolic")
        .margin_end(2)
        .halign(gtk::Align::Start)
        .build();

    let manual_method_button_label = gtk::Label::builder()
        .label(t!("manual_method_button_label"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build();

    let manual_method_button_child_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    manual_method_button_child_box.append(&manual_method_button_icon);
    manual_method_button_child_box.append(&manual_method_button_label);

    let automatic_method_button_icon = gtk::Image::builder()
        .icon_name("builder")
        .margin_end(2)
        .halign(gtk::Align::Start)
        .build();

    let automatic_method_button_label = gtk::Label::builder()
        .label(t!("automatic_method_button_label"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build();

    let automatic_method_button_child_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    automatic_method_button_child_box.append(&automatic_method_button_icon);
    automatic_method_button_child_box.append(&automatic_method_button_label);

    let method_labels_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);
    let method_icons_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);

    method_labels_size_group.add_widget(&manual_method_button_label);
    method_labels_size_group.add_widget(&automatic_method_button_label);

    method_icons_size_group.add_widget(&manual_method_button_icon);
    method_icons_size_group.add_widget(&automatic_method_button_icon);

    let manual_method_button = gtk::Button::builder()
        .child(&manual_method_button_child_box)
        .build();

    let automatic_method_button = gtk::Button::builder()
        .child(&automatic_method_button_child_box)
        .build();

    automatic_method_button.connect_clicked(clone!(
        #[weak]
        partitioning_carousel,
        move |_| partitioning_carousel.scroll_to(&partitioning_carousel.nth_page(1), true)
    ));

    manual_method_button.connect_clicked(clone!(
        #[weak]
        partitioning_carousel,
        move |_| partitioning_carousel.scroll_to(&partitioning_carousel.nth_page(2), true)
    ));

    content_box.append(&automatic_method_button);
    content_box.append(&manual_method_button);

    partitioning_page.set_child_widget(&content_box);

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        partitioning_page,
        move |_, _| {
            partitioning_page.set_page_title(t!("partitioning_page_title"));
            partitioning_page.set_page_subtitle(t!("partitioning_page_subtitle"));
            partitioning_page.set_back_tooltip_label(t!("back"));
            partitioning_page.set_next_tooltip_label(t!("next"));
            //
            automatic_method_button_label.set_label(&t!("automatic_method_button_label"));
            //
            manual_method_button_label.set_label(&t!("manual_method_button_label"));
        }
    ));
    //

    partitioning_carousel.append(&partitioning_page);
    automatic_partitioning_page::automatic_partitioning_page(
        &partitioning_carousel,
        &partition_method_type_refcell,
        &partition_method_automatic_target_refcell,
        &partition_method_automatic_target_fs_refcell,
        &partition_method_automatic_luks_enabled_refcell,
        &partition_method_automatic_luks_refcell,
        &partition_method_automatic_ratio_refcell,
        &partition_method_automatic_seperation_refcell,
        &language_changed_action,
    );
    manual_partitioning_page::manual_partitioning_page(
        &partitioning_carousel,
        window,
        &partition_method_type_refcell,
        &partition_method_manual_fstab_entry_array_refcell,
        &partition_method_manual_luks_enabled_refcell,
        &partition_method_manual_crypttab_entry_array_refcell,
        &language_changed_action,
    );

    partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_partitioning_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(4), true)
            }
        ),
    );

    main_carousel.append(&partitioning_carousel)
}

pub fn get_block_devices() -> Vec<BlockDevice> {
    let mut block_devices = Vec::new();

    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_devices")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(t) => t,
        Err(_) => return block_devices,
    };

    match command.stdout {
        Some(t) => {
            for blockdev in std::io::BufReader::new(t).lines() {
                match blockdev {
                    Ok(r) => {
                        let block_size = get_block_size(&r);
                        block_devices.push(BlockDevice {
                            block_name: r,
                            block_size: block_size,
                            block_size_pretty: pretty_bytes::converter::convert(block_size),
                        })
                    }
                    Err(_) => return block_devices,
                }
            }
        }
        None => return block_devices,
    };

    block_devices
}

fn get_block_size(block_dev: &str) -> f64 {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_size")
        .arg(block_dev)
        .output()
    {
        Ok(t) => t,
        Err(_) => return 0.0,
    };
    let size = match String::from_utf8(command.stdout) {
        Ok(t) => t.trim().parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    };

    size
}

pub fn get_partitions() -> Vec<Partition> {
    let mut partitions = Vec::new();

    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_partitions")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(t) => t,
        Err(_) => return partitions,
    };

    match command.stdout {
        Some(t) => {
            for partition in std::io::BufReader::new(t).lines() {
                match partition {
                    Ok(r) => partitions.push(create_parition_struct(&r)),
                    Err(_) => return partitions,
                }
            }
        }
        None => return partitions,
    };

    partitions
}

pub fn create_parition_struct(part_dev: &str) -> Partition {
    let part_size = get_part_size(&part_dev);
    let part_fs = get_part_fs(&part_dev);
    Partition {
        has_encryption: is_encrypted(&part_dev),
        need_mapper: is_needs_mapper(&part_fs),
        part_uuid: get_part_uuid(&part_dev),
        part_name: part_dev.to_string(),
        part_fs: part_fs,
        part_size: part_size,
        part_size_pretty: pretty_bytes::converter::convert(part_size),
    }
}

fn get_part_size(part_dev: &str) -> f64 {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_part_size")
        .arg(part_dev)
        .output()
    {
        Ok(t) => t,
        Err(_) => return 0.0,
    };
    let size = match String::from_utf8(command.stdout) {
        Ok(t) => t.trim().parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    };

    size
}

fn get_part_fs(part_dev: &str) -> String {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_part_fs")
        .arg(part_dev.replace("mapper/", ""))
        .output()
    {
        Ok(t) => t,
        Err(_) => return String::from(t!("fs_unknown")),
    };
    let fs = match String::from_utf8(command.stdout) {
        Ok(t) => t.trim().to_owned(),
        Err(_) => String::from(t!("fs_unknown")),
    };

    fs
}

fn is_needs_mapper(part_fs: &str) -> bool {
    if part_fs.contains("crypto_LUKS") || part_fs.contains("lvm") || part_fs.contains("BitLocker") {
        true
    } else {
        false
    }
}

fn is_encrypted(part_dev: &str) -> bool {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("has_encryption")
        .arg(part_dev)
        .output()
    {
        Ok(t) => t,
        Err(_) => return false,
    };

    if command.status.success() {
        true
    } else {
        false
    }
}

pub fn test_luks_passwd(part_dev: &str, passwd: &str) -> bool {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("test_luks_passwd")
        .arg(part_dev)
        .arg(passwd)
        .output()
    {
        Ok(t) => t,
        Err(_) => return false,
    };

    if command.status.success() {
        true
    } else {
        false
    }
}

fn get_part_uuid(part_dev: &str) -> String {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_part_uuid")
        .arg(part_dev)
        .output()
    {
        Ok(t) => t,
        Err(_) => return String::from(""),
    };
    let fs = match String::from_utf8(command.stdout) {
        Ok(t) => t.trim().to_owned(),
        Err(_) => String::from(""),
    };

    fs
}

pub fn get_luks_uuid(part_dev: &str) -> String {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_luks_uuid")
        .arg(part_dev)
        .output()
    {
        Ok(t) => t,
        Err(_) => return String::from(""),
    };
    let fs = match String::from_utf8(command.stdout) {
        Ok(t) => t.trim().to_owned(),
        Err(_) => String::from(""),
    };

    fs
}
