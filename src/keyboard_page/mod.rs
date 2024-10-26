use crate::{
    build_ui::{KBDMap, PikaKeymap},
    installer_stack_page,
};
use adw::prelude::*;
use glib::{clone, closure_local};
use gnome_desktop::XkbInfoExt;
use gtk::{gio, glib};
use std::{cell::RefCell, process::Command, rc::Rc};

pub fn keyboard_page(
    main_carousel: &adw::Carousel,
    keymap_data_refcell: &Rc<RefCell<PikaKeymap>>,
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
    keyboard_selection_row_viewport_listbox.add_css_class("no-round-borders");

    let keyboard_selection_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .overflow(gtk::Overflow::Hidden)
        .child(&keyboard_selection_row_viewport_listbox)
        .build();

    keyboard_selection_row_viewport.add_css_class("round-border-only-top-with-no-padding");

    let keyboard_search_bar = gtk::SearchEntry::builder()
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .search_delay(500)
        .build();

    keyboard_search_bar.add_css_class("rounded-all-25");

    let keyboard_test_entry_boxed_list = gtk::ListBox::builder()
        .margin_bottom(5)
        .build();

    keyboard_test_entry_boxed_list.add_css_class("boxed-list");

    let keyboard_test_entry = adw::EntryRow::builder().build();

    keyboard_test_entry_boxed_list.append(&keyboard_test_entry);

    keyboard_test_entry_boxed_list.add_css_class("round-border-only-bottom");

    let current_keymap = "us";
    let current_keymap_variant: Option<String> = None;

    let kbd_map_list = [
        KBDMap {
            console: "sg".to_string(),
            layout: "ch".to_string(),
            variant: "de_nodeadkeys".to_string(),
        },
        KBDMap {
            console: "nl".to_string(),
            layout: "nl".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "mk-utf".to_string(),
            layout: "mk".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "trq".to_string(),
            layout: "tr".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "trf".to_string(),
            layout: "tr".to_string(),
            variant: "f".to_string(),
        },
        KBDMap {
            console: "uk".to_string(),
            layout: "gb".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "is-latin1".to_string(),
            layout: "is".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "de".to_string(),
            layout: "de".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "la-latin1".to_string(),
            layout: "latam".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "us".to_string(),
            layout: "us".to_string(),
            variant: "".to_string(),
        },
        /*KBDMap {
            console: "ko".to_string(),
            layout: "kr".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "ro_std".to_string(),
            layout: "ro".to_string(),
            variant: "std".to_string(),
        },
        */
        KBDMap {
            console: "slovene".to_string(),
            layout: "si".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "hu".to_string(),
            layout: "hu".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "jp106".to_string(),
            layout: "jp".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "croat".to_string(),
            layout: "hr".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "hu101".to_string(),
            layout: "hu".to_string(),
            variant: "qwerty".to_string(),
        },
        KBDMap {
            console: "sr".to_string(),
            layout: "rs".to_string(),
            variant: "latin".to_string(),
        },
        KBDMap {
            console: "fi".to_string(),
            layout: "fi".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "fr_CH".to_string(),
            layout: "ch".to_string(),
            variant: "fr".to_string(),
        },
        KBDMap {
            console: "dk-latin1".to_string(),
            layout: "dk".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "fr".to_string(),
            layout: "fr".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "ua-utf".to_string(),
            layout: "ua".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "fr-latin9".to_string(),
            layout: "fr".to_string(),
            variant: "latin9".to_string(),
        },
        KBDMap {
            console: "sg-latin1".to_string(),
            layout: "ch".to_string(),
            variant: "de_nodeadkeys".to_string(),
        },
        KBDMap {
            console: "be-latin1".to_string(),
            layout: "be".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "dk".to_string(),
            layout: "dk".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "bg-cp1251".to_string(),
            layout: "bg".to_string(),
            variant: "phonetic".to_string(),
        },
        KBDMap {
            console: "it-ibm".to_string(),
            layout: "it".to_string(),
            variant: "ibm".to_string(),
        },
        KBDMap {
            console: "cz-us-qwertz".to_string(),
            layout: "cz".to_string(),
            variant: "".to_string(),
        },
        /*KBDMap {
            console: "cz-us-qwerty".to_string(),
            layout: "cz".to_string(),
            variant: "qwerty".to_string(),
        },*/
        KBDMap {
            console: "br-abnt2".to_string(),
            layout: "br".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "ro".to_string(),
            layout: "ro".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "us-intl.iso15".to_string(),
            layout: "us".to_string(),
            variant: "intl".to_string(),
        },
        KBDMap {
            console: "pt-latin1".to_string(),
            layout: "pt".to_string(),
            variant: "".to_string(),
        },
        /*KBDMap {
            console: "tj_alt-UTF8".to_string(),
            layout: "tj".to_string(),
            variant: "".to_string(),
        },*/
        KBDMap {
            console: "de-latin1-nodeadkeys".to_string(),
            layout: "de".to_string(),
            variant: "nodeadkeys".to_string(),
        },
        KBDMap {
            console: "no".to_string(),
            layout: "no".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "bg".to_string(),
            layout: "bg".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "dvorak".to_string(),
            layout: "us".to_string(),
            variant: "dvorak".to_string(),
        },
        KBDMap {
            console: "ru".to_string(),
            layout: "ru".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "cz-lat2".to_string(),
            layout: "cz".to_string(),
            variant: "qwerty".to_string(),
        },
        KBDMap {
            console: "pl".to_string(),
            layout: "pl".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "es".to_string(),
            layout: "es".to_string(),
            variant: "".to_string(),
        },
        /*KBDMap {
            console: "ie".to_string(),
            layout: "ie".to_string(),
            variant: "".to_string(),
        },*/
        KBDMap {
            console: "et".to_string(),
            layout: "ee".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "sk-qwerty".to_string(),
            layout: "sk".to_string(),
            variant: "qwerty".to_string(),
        },
        KBDMap {
            console: "sk-qwertz".to_string(),
            layout: "sk".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "fr_CH-latin1".to_string(),
            layout: "ch".to_string(),
            variant: "fr".to_string(),
        },
        KBDMap {
            console: "cf".to_string(),
            layout: "ca".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "se-latin1".to_string(),
            layout: "se".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "sr-cy".to_string(),
            layout: "rs".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "gr".to_string(),
            layout: "gr".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "by".to_string(),
            layout: "by".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "il".to_string(),
            layout: "il".to_string(),
            variant: "".to_string(),
        },
        /*KBDMap {
            console: "kazakh".to_string(),
            layout: "kz".to_string(),
            variant: "".to_string(),
        },*/
        KBDMap {
            console: "lt".to_string(),
            layout: "lt".to_string(),
            variant: "".to_string(),
        },
        /*KBDMap {
            console: "khmer".to_string(),
            layout: "kh".to_string(),
            variant: "".to_string(),
        },
        KBDMap {
            console: "dvorak-es".to_string(),
            layout: "es".to_string(),
            variant: "dvorak".to_string(),
        },
        */
        KBDMap {
            console: "lv-latin4".to_string(),
            layout: "lv".to_string(),
            variant: "apostrophe".to_string(),
        },
        KBDMap {
            console: "lv-latin7".to_string(),
            layout: "lv".to_string(),
            variant: "tilde".to_string(),
        },
    ];

    let xkbinfo = gnome_desktop::XkbInfo::new();

    let mut sorted_keymap_vec = Vec::new();
    for kbd_map in kbd_map_list {
        let map = kbd_map.clone();
        if map.variant.is_empty() {
            sorted_keymap_vec.push(PikaKeymap {
                kbdmap: kbd_map,
                pretty_name: xkbinfo
                    .layout_info(&map.layout)
                    .unwrap()
                    .0
                    .unwrap()
                    .to_string(),
            })
        } else {
            sorted_keymap_vec.push(PikaKeymap {
                kbdmap: kbd_map,
                pretty_name: xkbinfo
                    .layout_info(format!("{}+{}", map.layout, map.variant).as_str())
                    .unwrap()
                    .0
                    .unwrap()
                    .to_string(),
            })
        }
    }
    sorted_keymap_vec.sort_by_key(|k| k.pretty_name.clone());

    for pika_keymap in sorted_keymap_vec {
        let keymap_clone0 = pika_keymap.clone();
        let keymap = pika_keymap.kbdmap.layout;
        let keymap_name = pika_keymap.pretty_name;
        let keymap_variant = if pika_keymap.kbdmap.variant.is_empty() {
            None
        } else {
            Some(pika_keymap.kbdmap.variant)
        };
        let keymap_variant_clone0 = keymap_variant.clone();
        let keymap_checkbutton = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        let keymap_row = adw::ActionRow::builder()
            .activatable_widget(&keymap_checkbutton)
            .title(keymap_name)
            .build();
        match &keymap_variant {
            Some(v) => {
                keymap_row.set_subtitle(&format!("{}+{}", keymap, v).to_string());
            }
            None => {
                keymap_row.set_subtitle(&keymap);
            }
        }
        keymap_row.add_prefix(&keymap_checkbutton);
        keymap_checkbutton.set_group(Some(&null_checkbutton));
        keyboard_selection_row_viewport_listbox.append(&keymap_row);
        keymap_checkbutton.connect_toggled(clone!(
            #[weak]
            keymap_checkbutton,
            #[strong]
            keymap_data_refcell,
            #[strong]
            keymap_clone0,
            #[weak]
            keyboard_page,
            move |_| {
                if keymap_checkbutton.is_active() {
                    *keymap_data_refcell.borrow_mut() = keymap_clone0.clone();
                    keyboard_page.set_next_sensitive(true);
                    match keymap_variant.clone() {
                        Some(t) => {
                            Command::new("setxkbmap")
                                .arg("-layout")
                                .arg(&keymap)
                                .arg("-variant")
                                .arg(t)
                                .spawn()
                                .expect("keyboard failed to start");
                        }
                        None => {
                            Command::new("setxkbmap")
                                .arg("-layout")
                                .arg(&keymap)
                                .spawn()
                                .expect("keyboard failed to start");
                        }
                    }
                }
            }
        ));
        if current_keymap == keymap_clone0.kbdmap.layout
            && current_keymap_variant == keymap_variant_clone0
        {
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
            keyboard_search_bar
                .set_placeholder_text(Some(&t!("keyboard_search_bar_placeholder_text")));
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
