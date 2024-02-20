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
use std::{str};

use std::fs;
use std::path::Path;

pub fn timezone_page(content_stack: &gtk::Stack,
                     timezone_main_box: &gtk::Box,
) {

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

    // the header box for the timezone page
    let timezone_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the timezone page
    let timezone_header_text = gtk::Label::builder()
        .label(t!("select_a_timezone"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    timezone_header_text.add_css_class("header_sized_text");

    // the header icon for the timezone icon
    let timezone_header_icon = gtk::Image::builder()
        .icon_name("alarm-clock")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // make timezone selection box for choosing installation or live media
    let timezone_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / timezone_header_box appends
    //// Add the timezone page header text and icon
    timezone_header_box.append(&timezone_header_text);
    timezone_header_box.append(&timezone_header_icon);

    // / timezone_main_box appends
    //// Add the timezone header to timezone main box
    timezone_main_box.append(&timezone_header_box);
    //// Add the timezone selection/page content box to timezone main box
    timezone_main_box.append(&timezone_selection_box);

    // text above timezone selection box
    let timezone_selection_text = gtk::Label::builder()
        .label(t!("please_select_timezone"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    timezone_selection_text.add_css_class("medium_sized_text");

    let timezone_selection_expander_row = adw::ExpanderRow::builder()
        .title(t!("no_timezone_select"))
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label(t!("no_timezone_select"))
        .build();

    let timezone_selection_expander_row_viewport =
        gtk::ScrolledWindow::builder().height_request(420).build();

    let timezone_selection_expander_row_viewport_box = gtk::ListBox::builder().build();
    timezone_selection_expander_row_viewport_box.add_css_class("boxed-list");

    let timezone_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    timezone_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    timezone_selection_expander_row_viewport_listbox.append(&timezone_selection_expander_row);

    timezone_selection_expander_row_viewport
        .set_child(Some(&timezone_selection_expander_row_viewport_box));

    timezone_selection_expander_row.add_row(&timezone_selection_expander_row_viewport);

    let timezone_search_bar = gtk::SearchEntry::builder()
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .search_delay(500)
        .build();

    let current_timezone_cli = Command::new("timedatectl")
        .arg("show")
        .arg("--va")
        .arg("-p")
        .arg("Timezone")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let current_timezone_output = current_timezone_cli.wait_with_output().unwrap();
    let current_timezone = str::from_utf8(&current_timezone_output.stdout)
        .unwrap()
        .trim();

    let timezone_cli = Command::new("timedatectl")
        .arg("list-timezones")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let timezone_stdout = timezone_cli.stdout.expect("could not get stdout");
    let timezone_reader = BufReader::new(timezone_stdout);

    let timezone_data_buffer = gtk::TextBuffer::builder().build();

    for timezone in timezone_reader.lines() {
        let timezone = timezone.unwrap();
        let timezone_clone = timezone.clone();
        let timezone_checkbutton = gtk::CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let timezone_row = adw::ActionRow::builder()
            .activatable_widget(&timezone_checkbutton)
            .title(timezone.clone())
            .build();
        timezone_row.add_prefix(&timezone_checkbutton);
        timezone_checkbutton.set_group(Some(&null_checkbutton));
        timezone_selection_expander_row_viewport_box.append(&timezone_row);
        timezone_checkbutton.connect_toggled(clone!(@weak timezone_checkbutton, @weak timezone_selection_expander_row, @weak bottom_next_button, @weak timezone_data_buffer => move |_| {
            if timezone_checkbutton.is_active() == true {
                timezone_selection_expander_row.set_title(&timezone);
                bottom_next_button.set_sensitive(true);
                timezone_data_buffer.set_text(&timezone);
            }
        }));
        if current_timezone.contains(&(timezone_clone)) {
            timezone_checkbutton.set_active(true);
        }
    }

    // / timezone_selection_box appends
    //// add text and and entry to timezone page selections
    timezone_selection_box.append(&timezone_selection_text);
    timezone_selection_box.append(&timezone_search_bar);
    timezone_selection_box.append(&timezone_selection_expander_row_viewport_listbox);

    // / timezone_header_box appends
    //// Add the timezone page header text and icon
    timezone_header_box.append(&timezone_header_text);
    timezone_header_box.append(&timezone_header_icon);

    // / timezone_main_box appends
    //// Add the timezone header to timezone main box
    timezone_main_box.append(&timezone_header_box);
    //// Add the timezone selection/page content box to timezone main box
    timezone_main_box.append(&timezone_selection_box);

    timezone_main_box.append(&bottom_box);

    let timezone_data_buffer_clone = timezone_data_buffer.clone();

    timezone_search_bar.connect_search_changed(clone!(@weak timezone_search_bar, @weak timezone_selection_expander_row_viewport_box => move |_| {
        let mut counter = timezone_selection_expander_row_viewport_box.first_child();
        while let Some(row) = counter {
            if row.widget_name() == "AdwActionRow" {
                if !timezone_search_bar.text().is_empty() {
                    if row.property::<String>("subtitle").to_lowercase().contains(&timezone_search_bar.text().to_string().to_lowercase()) || row.property::<String>("title").to_lowercase().contains(&timezone_search_bar.text().to_string().to_lowercase()) {
                        timezone_selection_expander_row.set_expanded(true);
                        //row.grab_focus();
                        //row.add_css_class("highlight-widget");
                        row.set_property("visible", true);
                        timezone_search_bar.grab_focus();
                    } else {
                        row.set_property("visible", false);
                    }
                } else {
                    row.set_property("visible", true);
                }
            }
            counter = row.next_sibling();
        }
    }));

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        if Path::new("/tmp/pika-installer-gtk4-timezone.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-timezone.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-timezone.txt");
        }
        fs::write("/tmp/pika-installer-gtk4-timezone.txt", timezone_data_buffer_clone.text(&timezone_data_buffer_clone.bounds().0, &timezone_data_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
        Command::new("sudo")
        .arg("timedatectl")
        .arg("set-timezone")
        .arg(&timezone_data_buffer_clone.text(&timezone_data_buffer_clone.bounds().0, &timezone_data_buffer_clone.bounds().1, true).to_string())
        .spawn()
        .expect("timezone failed to start");
        content_stack.set_visible_child_name("keyboard_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack, @weak timezone_main_box => move |_| {
        content_stack.set_visible_child_name("eula_page");
    }));
}
