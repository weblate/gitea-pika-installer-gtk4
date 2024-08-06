use crate::installer_stack_page;
use crate::config;
use gnome_desktop::XkbInfoExt;
use gtk::{prelude::*, glib as glib, gio as gio};
use adw::{prelude::*};
use glib::{clone, closure_local};
use std::{process::Command, env, fs, path::Path};
use gtk::ResponseType::No;

pub fn keyboard_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction
) {
    let keyboard_page = installer_stack_page::InstallerStackPage::new();
    keyboard_page.set_back_visible(true);
    keyboard_page.set_next_visible(true);
    keyboard_page.set_back_sensitive(true);
    keyboard_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .build();

    let keyboard_selection_row_viewport =
        gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .build();

    let keyboard_selection_row_viewport_box = gtk::ListBox::builder().build();
    keyboard_selection_row_viewport_box.add_css_class("boxed-list");

    keyboard_selection_row_viewport
        .set_child(Some(&keyboard_selection_row_viewport_box));

    let keyboard_selection_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    keyboard_selection_row_viewport_listbox.add_css_class("boxed-list");

    let keyboard_search_bar = gtk::SearchEntry::builder()
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .search_delay(500)
        .build();

    keyboard_search_bar.add_css_class("rounded-all-25");

    let keyboard_test_entry_boxed_list = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    keyboard_test_entry_boxed_list.add_css_class("boxed-list");

    let keyboard_test_entry = adw::EntryRow::builder()
        .build();

    keyboard_test_entry_boxed_list.append(&keyboard_test_entry);

    let current_keyboard_cli = Command::new("localectl")
        .arg("status")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));
    let current_keyboard_grep = Command::new("grep")
        .arg("X11 Layout")
        .stdin(std::process::Stdio::from(current_keyboard_cli.stdout.unwrap())) // Pipe through.
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let current_keyboard_cut = Command::new("cut")
        .arg("-d:")
        .arg("-f2")
        .stdin(std::process::Stdio::from(current_keyboard_grep.stdout.unwrap()))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let current_keyboard_output = current_keyboard_cut.wait_with_output().unwrap();
    let current_keymap = String::from_utf8_lossy(&current_keyboard_output.stdout).trim().to_owned();

    let xkbinfo = gnome_desktop::XkbInfo::new();

    let keymap_list = gnome_desktop::XkbInfo::all_layouts(&xkbinfo);

    let keyboard_data_buffer = gtk::TextBuffer::builder().build();

    let keyboard_data_buffer_clone0 = keyboard_data_buffer.clone();

    for keymap in keymap_list.iter() {
        let keymap = keymap.to_string();
        let keymap_name = xkbinfo.layout_info(&keymap).unwrap().0.unwrap().to_string();
        let keymap_clone = keymap.clone();
        let keymap_checkbutton = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        let keymap_row = adw::ActionRow::builder()
            .activatable_widget(&keymap_checkbutton)
            .title(keymap_name)
            .subtitle(keymap.clone())
            .build();
        keymap_row.add_prefix(&keymap_checkbutton);
        keymap_checkbutton.set_group(Some(&null_checkbutton));
        keyboard_selection_row_viewport_box.append(&keymap_row);
        keymap_checkbutton.connect_toggled(clone!(
            #[weak]
            keymap_checkbutton,
            #[weak]
            keyboard_data_buffer_clone0,
            #[weak]
            keyboard_page,
            move |_|
                {
                    if keymap_checkbutton.is_active() == true {
                        keyboard_page.set_next_sensitive(true);
                        keyboard_data_buffer_clone0.set_text(&keymap);
                        Command::new("setxkbmap")
                            .arg("-layout")
                            .arg(keymap.clone())
                            .spawn()
                            .expect("keyboard failed to start");
                    }
                }
        ));
        if current_keymap.contains(&(keymap_clone)) {
            keymap_checkbutton.set_active(true);
        }
    }

    // / content_box appends
    //// add text and and entry to keyboard page selections
    content_box.append(&keyboard_search_bar);
    content_box.append(&keyboard_selection_row_viewport);
    content_box.append(&keyboard_test_entry_boxed_list);

    keyboard_search_bar.connect_search_changed(clone!(
        #[weak]
        keyboard_search_bar,
        #[weak]
        keyboard_selection_row_viewport_box,
        move |_|
        {
            let mut counter = keyboard_selection_row_viewport_box.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !keyboard_search_bar.text().is_empty() {
                        if row.property::<String>("subtitle").to_lowercase().contains(&keyboard_search_bar.text().to_string().to_lowercase()) || row.property::<String>("title").to_lowercase().contains(&keyboard_search_bar.text().to_string().to_lowercase()) {
                            row.set_property("visible", true);
                            keyboard_search_bar.grab_focus();
                        } else {
                            row.set_property("visible", false);
                        }
                    } else {
                        row.set_property("visible", true);
                    }
                }
                counter = row.next_sibling();
            }
        }
    ));

    keyboard_page.set_child_widget(&content_box);

    //
    language_changed_action.connect_activate(
        clone!(
            #[weak]
            keyboard_page,
            #[weak]
            keyboard_search_bar,
            #[weak]
            keyboard_test_entry,
            move |_, _| {
                keyboard_page.set_page_title(t!("keyboard"));
                keyboard_page.set_page_subtitle(t!("select_a_keyboard"));
                keyboard_page.set_page_icon("keyboard-symbolic");
                keyboard_page.set_back_tooltip_label(t!("back"));
                keyboard_page.set_next_tooltip_label(t!("next"));
                //
                keyboard_search_bar.set_placeholder_text(Some(&t!("search_for_keyboard")));
                //
                keyboard_test_entry.set_title(&t!("test_your_keyboard"))
            }
        )
    );
    //

    keyboard_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_keyboard_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(2), true)
            }
        )
    );

    keyboard_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            #[strong]
            language_changed_action,
            move |_keyboard_page: installer_stack_page::InstallerStackPage|
            {
                if Path::new("/tmp/pika-installer-gtk4-keyboard.txt").exists() {
                    fs::remove_file("/tmp/pika-installer-gtk4-keyboard.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-keyboard.txt");
                }
                fs::write("/tmp/pika-installer-gtk4-keyboard.txt", keyboard_data_buffer_clone0.text(&keyboard_data_buffer_clone0.bounds().0, &keyboard_data_buffer_clone0.bounds().1, true).to_string()).expect("Unable to write file");
                main_carousel.scroll_to(&main_carousel.nth_page(4), true)
            }
        )
    );

    main_carousel.append(&keyboard_page);
}