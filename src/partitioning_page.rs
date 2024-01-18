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

pub fn partitioning_page(content_stack: &gtk::Stack) {
   
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

    let mut partition_method_automatic_get_devices_cli = Command::new("pkexec")
        .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
        .arg("get_block_devices")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));
    let partition_method_automatic_get_devices_reader = BufReader::new(partition_method_automatic_get_devices_cli.stdout.as_mut().expect("could not get stdout"));

    let partition_method_automatic_status_label = gtk::Label::builder()
        .label("No Disk specified")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .build();
    partition_method_automatic_status_label.add_css_class("small_error_text");

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
        // button connect clones
        let device_button_clone = device_button.clone();
        let devices_selection_expander_row_clone = devices_selection_expander_row.clone();
        let bottom_next_button_clone = bottom_next_button.clone();
        let partition_method_automatic_status_label_clone = partition_method_automatic_status_label.clone();
        //
        device_button.connect_toggled(move |_| {
            if device_button_clone.is_active() == true {
                devices_selection_expander_row_clone.set_title(&device);
                if device_size > 39000000000.0 {
                    partition_method_automatic_status_label_clone.hide();
                    bottom_next_button_clone.set_sensitive(true);
                } else {
                    partition_method_automatic_status_label_clone.show();
                    partition_method_automatic_status_label_clone.set_label("Disk Size too small, PikaOS needs 40GB Disk");
                    bottom_next_button_clone.set_sensitive(false);
                }
            }
        });
    }

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
        .build();

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
    partition_method_automatic_main_box.append(&partition_method_automatic_status_label);

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

    let partition_method_manual_luks_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    let partition_method_manual_luks_checkbutton = gtk::CheckButton::builder()
        .label("(CUSTOM_ROOT)/home has LUKS Encryption?")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
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
        .build();

    let partition_method_manual_status_label = gtk::Label::builder()
        .label("No mountpoint specified")
        .halign(Align::Start)
        .valign(Align::End)
        .vexpand(true)
        .build();
    partition_method_manual_status_label.add_css_class("small_error_text");

    partition_method_manual_luks_listbox.append(&partition_method_manual_luks_password_entry);
    partition_method_manual_luks_box.append(&partition_method_manual_luks_checkbutton);
    partition_method_manual_luks_box.append(&partition_method_manual_luks_listbox);
    partition_method_manual_header_box.append(&partition_method_manual_header_text);
    partition_method_manual_header_box.append(&partition_method_manual_header_icon);
    partition_method_manual_selection_box.append(&partition_method_manual_selection_text);
    partition_method_manual_main_box.append(&partition_method_manual_header_box);
    partition_method_manual_main_box.append(&partition_method_manual_selection_box);

    partition_method_manual_main_box.append(&partition_method_manual_luks_box);
    partition_method_manual_main_box.append(&partition_method_manual_status_label);

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
    
    
    let partitioning_stack_clone = partitioning_stack.clone();
    automatic_method_button.connect_clicked(move |_| partitioning_stack_clone.set_visible_child_name("partition_method_automatic_page"));
    let partitioning_stack_clone2 = partitioning_stack.clone();
    manual_method_button.connect_clicked(move |_| partitioning_stack_clone2.set_visible_child_name("partition_method_manual_page"));

    let content_stack_clone = content_stack.clone();
    let content_stack_clone2 = content_stack.clone();
    let partitioning_stack_clone3 = partitioning_stack.clone();
    bottom_next_button.connect_clicked(move |_| {
        content_stack_clone.set_visible_child_name("installation_page")
    });
    bottom_back_button.connect_clicked(move |_| {
        content_stack_clone2.set_visible_child_name("keyboard_page")
    });
    bottom_back_button.connect_clicked(move |_| {
        partitioning_stack_clone3.set_visible_child_name("partition_method_select_page")
    });

}