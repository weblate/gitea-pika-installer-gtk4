// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use gettextrs::{gettext, LocaleCategory};

use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

use std::fs;
use std::path::Path;
use crate::eula_page::eula_page;
use crate::keyboard_page::keyboard_page;
use crate::partitioning_page::partitioning_page;
use crate::timezone_page::timezone_page;

pub fn language_page(content_stack: &gtk::Stack, window: &adw::ApplicationWindow) {
    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();

    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label(gettext("back"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    let bottom_next_button = gtk::Button::builder()
        .label(gettext("next"))
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

    // the header box for the language page
    let language_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // the header box for the language page
    let language_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the language page
    let language_header_text = gtk::Label::builder()
        .label(gettext("select_a_language"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    language_header_text.add_css_class("header_sized_text");

    // the header icon for the language icon
    let language_header_icon = gtk::Image::builder()
        .icon_name("locale")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // make language selection box for choosing installation or live media
    let language_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);

    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    // text above language selection box
    let language_selection_text = gtk::Label::builder()
        .label(gettext("please_select_locale"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_text.add_css_class("medium_sized_text");

    let language_selection_expander_row = adw::ExpanderRow::builder()
        .title(gettext("no_locale_selected"))
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label(gettext("no_locale_selected"))
        .build();

    let language_selection_expander_row_viewport =
        gtk::ScrolledWindow::builder().height_request(420).build();

    let language_selection_expander_row_viewport_box = gtk::ListBox::builder().build();
    language_selection_expander_row_viewport_box.add_css_class("boxed-list");

    language_selection_expander_row_viewport
        .set_child(Some(&language_selection_expander_row_viewport_box));

    let language_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    language_selection_expander_row_viewport_listbox.append(&language_selection_expander_row);

    language_selection_expander_row.add_row(&language_selection_expander_row_viewport);

    let language_search_bar = gtk::SearchEntry::builder()
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .search_delay(500)
        .build();

    let current_locale = match env::var_os("LANG") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$LANG is not set"),
    };

    let locale_cli = Command::new("locale")
        .arg("-a")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));
    let locale_cli_cut = Command::new("cut")
        .arg("-d.")
        .arg("-f1")
        .stdin(Stdio::from(locale_cli.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let locale_cli_sort = Command::new("sort")
        .arg("-u")
        .stdin(Stdio::from(locale_cli_cut.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let locale_reader = BufReader::new(locale_cli_sort.stdout.expect("could not get stdout"));

    let lang_data_buffer = gtk::TextBuffer::builder().build();

    for locale in locale_reader.lines() {
        let locale = locale.unwrap();
        let locale_name_cli =
            Command::new("/usr/lib/pika/pika-installer-gtk4/scripts/locale-name.py")
                .arg(locale.clone())
                .output()
                .expect("failed to execute process");
        let locale_name = String::from_utf8(locale_name_cli.stdout).unwrap();
        let locale_clone = locale.clone();
        let locale_checkbutton = gtk::CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let locale_row = adw::ActionRow::builder()
            .activatable_widget(&locale_checkbutton)
            .title(locale_name)
            .subtitle(locale.clone())
            .build();
        locale_row.add_prefix(&locale_checkbutton);
        locale_checkbutton.set_group(Some(&null_checkbutton));
        language_selection_expander_row_viewport_box.append(&locale_row);
        locale_checkbutton.connect_toggled(clone!(@weak locale_checkbutton, @weak language_selection_expander_row, @weak bottom_next_button, @weak lang_data_buffer => move |_| {
            if locale_checkbutton.is_active() == true {
                language_selection_expander_row.set_title(&locale_row.title());
                bottom_next_button.set_sensitive(true);
                lang_data_buffer.set_text(&locale);
            }
        }));
        if current_locale.contains(&(locale_clone))
            && current_locale != "C.UTF-8"
            && current_locale != "C"
            && current_locale != "C.utf8"
            && current_locale != "POSIX"
        {
            locale_checkbutton.set_active(true);
        }
    }

    // / language_selection_box appends
    //// add text and and entry to language page selections
    language_selection_box.append(&language_selection_text);
    language_selection_box.append(&language_search_bar);
    language_selection_box.append(&language_selection_expander_row_viewport_listbox);

    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);

    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    language_main_box.append(&bottom_box);

    let lang_data_buffer_clone = lang_data_buffer.clone();

    language_search_bar.connect_search_changed(clone!(@weak language_search_bar, @weak language_selection_expander_row_viewport_box => move |_| {
        let mut counter = language_selection_expander_row_viewport_box.first_child();
        while let Some(row) = counter {
            if row.widget_name() == "AdwActionRow" {
                if !language_search_bar.text().is_empty() {
                    if row.property::<String>("subtitle").to_lowercase().contains(&language_search_bar.text().to_string().to_lowercase()) || row.property::<String>("title").to_lowercase().contains(&language_search_bar.text().to_string().to_lowercase()) {
                        language_selection_expander_row.set_expanded(true);
                        //row.grab_focus();
                        //row.add_css_class("highlight-widget");
                        row.set_property("visible", true);
                        language_search_bar.grab_focus();
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


    // / Content stack appends
    //// Add the language_main_box as page: language_page, Give it nice title
    content_stack.add_titled(
        &language_main_box,
        Some("language_page"),
        &gettext("language"),
    );

    // the header box for the eula page
    let eula_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the eula_main_box as page: eula_page, Give it nice title
    content_stack.add_titled(&eula_main_box, Some("eula_page"), &gettext("eula"));

    // the header box for the timezone page
    let timezone_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the keyboard_main_box as page: keyboard_page, Give it nice title
    content_stack.add_titled(
        &timezone_main_box,
        Some("timezone_page"),
        &gettext("timezone"),
    );

    // the header box for the keyboard page
    let keyboard_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the keyboard_main_box as page: keyboard_page, Give it nice title
    content_stack.add_titled(
        &keyboard_main_box,
        Some("keyboard_page"),
        &gettext("keyboard"),
    );

    // Add install_page.rs as a page for content_stack
    let install_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let done_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // the header box for the partitioning page
    let partitioning_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the partitioning_main_box as page: partitioning_page, Give it nice title
    content_stack.add_titled(
        &partitioning_main_box,
        Some("partitioning_page"),
        &gettext("partitioning"),
    );

    //// Add the install_main_box as page: install_page, Give it nice title
    content_stack.add_titled(
        &install_main_box,
        Some("install_page"),
        &gettext("installation"),
    );

    // Add done_page.rs as a page for content_stack
    content_stack.add_titled(&done_main_box, Some("done_page"), &gettext("done"));

    bottom_next_button.connect_clicked(clone!(@weak content_stack, @weak window => move |_| {
        if Path::new("/tmp/pika-installer-gtk4-lang.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-lang.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-lang.txt");
        }
        fs::write("/tmp/pika-installer-gtk4-lang.txt", lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
        Command::new("sudo")
        .arg("localectl")
        .arg("set-locale")
        .arg("LANG=".to_owned() + &lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string() + ".UTF-8")
        .spawn()
        .expect("locale failed to start");
        gettextrs::setlocale(LocaleCategory::LcAll, lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string() + ".UTF-8");
        if gettext("pikaos_installer") == "pikaos_installer" {
            println!("Warning: Current LANG is not supported, using fallback Locale.");
            gettextrs::setlocale(LocaleCategory::LcAll, "en_US.UTF8");
        }
        // Add eula_page.rs as a page for content_stack
        while let Some(widget) = eula_main_box.last_child() {
                eula_main_box.remove(&widget);
        }
        eula_page(&content_stack, &eula_main_box);
        // Add timezone_page.rs as a page for content_stack
        while let Some(widget) = timezone_main_box.last_child() {
                timezone_main_box.remove(&widget);
        }
        timezone_page(&content_stack, &timezone_main_box);
        // Add keyboard_page.rs as a page for content_stack
        while let Some(widget) = keyboard_main_box.last_child() {
                keyboard_main_box.remove(&widget);
        }
        keyboard_page(&content_stack, &keyboard_main_box);
        // Add partitioning_page.rs as a page for content_stack
        while let Some(widget) = partitioning_main_box.last_child() {
                partitioning_main_box.remove(&widget);
        }
        partitioning_page(&partitioning_main_box, &done_main_box, &install_main_box, &content_stack, &window);
        //
        content_stack.set_visible_child_name("eula_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("welcome_page")
    }));
}
