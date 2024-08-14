use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib, gio as gio};
use glib::{clone, closure_local};
use crate::{automatic_partitioning_page};
use std::io::BufRead;

pub fn partitioning_page(
    main_carousel: &adw::Carousel,
    partition_method_type_buffer: &gtk::TextBuffer,
    partition_method_automatic_target_buffer:  &gtk::TextBuffer,
    partition_method_automatic_luks_buffer:  &gtk::TextBuffer,
    partition_method_automatic_ratio_buffer: &gtk::TextBuffer,
    partition_method_automatic_seperation_buffer: &gtk::TextBuffer,
    language_changed_action: &gio::SimpleAction
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

    let automatic_method_button = gtk::Button::builder()
        .icon_name("builder")
        .build();

    let manual_method_button = gtk::Button::builder()
        .icon_name("org.gnome.Settings")
        .build();

    automatic_method_button.connect_clicked(
        clone!(
            #[weak]
            partitioning_carousel,
            move |_|
            partitioning_carousel.scroll_to(&partitioning_carousel.nth_page(1), true)
        )
    );

    manual_method_button.connect_clicked(
        clone!(
            #[weak]
            partitioning_carousel,
            move |_|
            partitioning_carousel.scroll_to(&partitioning_carousel.nth_page(2), true)
        )
    );

    content_box.append(&automatic_method_button);
    content_box.append(&manual_method_button);

    partitioning_page.set_child_widget(&content_box);

    //
    language_changed_action.connect_activate(
        clone!(
            #[weak]
            partitioning_page,
            move |_, _| {
                partitioning_page.set_page_title(t!("partitioning"));
                partitioning_page.set_page_subtitle(t!("choose_install_method"));
                partitioning_page.set_back_tooltip_label(t!("back"));
                partitioning_page.set_next_tooltip_label(t!("next"));
                //
                automatic_method_button.set_label(&t!("auto_partition_drive"));
                //
                manual_method_button.set_label(&t!("manual_partition_drive"));
            }
        )
    );
    //

    partitioning_carousel.append(&partitioning_page);
    automatic_partitioning_page::automatic_partitioning_page(
        &partitioning_carousel, 
        &partition_method_type_buffer,
        &partition_method_automatic_target_buffer,
        &partition_method_automatic_luks_buffer,
        &partition_method_automatic_ratio_buffer,
        &partition_method_automatic_seperation_buffer,
        &language_changed_action);

    partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(4), true)
            }
        )
    );

    main_carousel.append(&partitioning_carousel)
}

pub struct BlockDevice {
    pub block_name: String,
    pub block_size: f64,
    pub block_size_pretty: String
}

pub fn get_block_devices() -> Vec<BlockDevice> {
    let mut block_devices = Vec::new();

    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_devices")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn() {
            Ok(t) => t,
            Err(_) => return block_devices
        };

    match command.stdout {
        Some(t) => {
            for blockdev in std::io::BufReader::new(t).lines() {
                match blockdev {
                    Ok(r) => {
                        let block_size = get_block_size(&r);
                        block_devices.push(
                            BlockDevice {
                                block_name: r,
                                block_size: block_size,
                                block_size_pretty: pretty_bytes::converter::convert(block_size)
                            }
                        )
                    }
                    Err(_) => return block_devices
                }
            }
        },
        None => return block_devices
    };

    block_devices
}

fn get_block_size(block_dev: &str) -> f64 {
    let command = match std::process::Command::new("sudo")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_size")
        .arg(block_dev)
        .output() {
            Ok(t) => t,
            Err(_) => return 0.0
        };
    let size = match String::from_utf8(command.stdout) {
        Ok(t) => {
            t.trim().parse::<f64>().unwrap_or(0.0)
        }
        Err(_) => 0.0
    };

    size
}