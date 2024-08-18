use crate::installer_stack_page;
use adw::prelude::*;
use glib::{clone, closure_local};
use gnome_desktop::XkbInfoExt;
use gtk::{gio, glib, prelude::*};
use std::{cell::RefCell, fs, path::Path, process::Command, rc::Rc};

pub fn keyboard_page(
    main_carousel: &adw::Carousel,
    keymap_base_data_refcell: &Rc<RefCell<String>>,
    keymap_variant_data_refcell: &Rc<RefCell<String>>,
    language_changed_action: &gio::SimpleAction,
) {
    let keyboard_page = installer_stack_page::InstallerStackPage::new();
    keyboard_page.set_page_icon("keyboard-symbolic");
    keyboard_page.set_back_visible(true);
    keyboard_page.set_next_visible(true);
    keyboard_page.set_back_sensitive(true);
    keyboard_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let null_checkbutton = gtk::CheckButton::builder().build();

    let keyboard_selection_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    keyboard_selection_row_viewport_listbox.add_css_class("boxed-list");
    keyboard_selection_row_viewport_listbox.add_css_class("round-border-only");

    let keyboard_selection_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&keyboard_selection_row_viewport_listbox)
        .build();

    keyboard_selection_row_viewport.add_css_class("round-border-only-top-with-padding");

    let keyboard_search_bar = gtk::SearchEntry::builder()
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .search_delay(500)
        .build();

    keyboard_search_bar.add_css_class("rounded-all-25");

    let keyboard_test_entry_boxed_list = gtk::ListBox::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();

    keyboard_test_entry_boxed_list.add_css_class("boxed-list");

    let keyboard_test_entry = adw::EntryRow::builder().build();

    keyboard_test_entry_boxed_list.append(&keyboard_test_entry);

    keyboard_test_entry_boxed_list.add_css_class("round-border-only-bottom");

    let current_keymap = "us";

    let xkbinfo = gnome_desktop::XkbInfo::new();

    let keymap_list = gnome_desktop::XkbInfo::all_layouts(&xkbinfo);

    for keymap in keymap_list.iter() {
        let keymap = keymap.to_string();
        let keymap_name = xkbinfo.layout_info(&keymap).unwrap().0.unwrap().to_string();
        let keymap_split: Vec<String> = keymap.split("+").map(|s| s.into()).collect();
        let keymap_base = keymap_split.get(0).unwrap().clone();
        let mut keymap_variant = String::new();
        let mut split_index = 0;
        for split in keymap_split {
            split_index += 1;
            if split_index == 1 {
                continue;
            }
            keymap_variant.push_str(&split)
        }
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
        keyboard_selection_row_viewport_listbox.append(&keymap_row);
        keymap_checkbutton.connect_toggled(clone!(
            #[weak]
            keymap_checkbutton,
            #[strong]
            keymap_base_data_refcell,
            #[strong]
            keymap_variant_data_refcell,
            #[weak]
            keyboard_page,
            move |_| {
                if keymap_checkbutton.is_active() == true {
                    keyboard_page.set_next_sensitive(true);
                    if keymap_variant.is_empty() {
                        *keymap_base_data_refcell.borrow_mut() = String::from(&keymap_base);
                        Command::new("setxkbmap")
                            .arg("-layout")
                            .arg(keymap_base.clone())
                            .spawn()
                            .expect("keyboard failed to start");
                    } else {
                        *keymap_base_data_refcell.borrow_mut() = String::from(&keymap_base);
                        *keymap_variant_data_refcell.borrow_mut() = String::from(&keymap_variant);
                        Command::new("setxkbmap")
                            .arg("-layout")
                            .arg(keymap_base.clone())
                            .arg("-variant")
                            .arg(keymap_variant.clone())
                            .spawn()
                            .expect("keyboard failed to start");
                    }
                }
            }
        ));
        if current_keymap == keymap_clone {
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
        keyboard_selection_row_viewport_listbox,
        move |_| {
            let mut counter = keyboard_selection_row_viewport_listbox.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !keyboard_search_bar.text().is_empty() {
                        if row
                            .property::<String>("subtitle")
                            .to_lowercase()
                            .contains(&keyboard_search_bar.text().to_string().to_lowercase())
                            || row
                                .property::<String>("title")
                                .to_lowercase()
                                .contains(&keyboard_search_bar.text().to_string().to_lowercase())
                        {
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
    language_changed_action.connect_activate(clone!(
        #[weak]
        keyboard_page,
        #[weak]
        keyboard_search_bar,
        #[weak]
        keyboard_test_entry,
        move |_, _| {
            keyboard_page.set_page_title(t!("keyboard_page_title"));
            keyboard_page.set_page_subtitle(t!("keyboard_page_subtitle"));
            keyboard_page.set_back_tooltip_label(t!("back"));
            keyboard_page.set_next_tooltip_label(t!("next"));
            //
            keyboard_search_bar.set_placeholder_text(Some(&t!("keyboard_search_bar_placeholder_text")));
            //
            keyboard_test_entry.set_title(&t!("keyboard_test_entry_title"))
        }
    ));
    //

    keyboard_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_keyboard_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(2), true)
            }
        ),
    );

    keyboard_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_keyboard_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(4), true)
            }
        ),
    );

    main_carousel.append(&keyboard_page);
}
