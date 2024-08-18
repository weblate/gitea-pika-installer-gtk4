use crate::installer_stack_page;
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib, prelude::*};
use std::io::BufRead;
use std::{cell::RefCell, fs, path::Path, process::Command, rc::Rc};

pub fn timezone_page(
    main_carousel: &adw::Carousel,
    timezone_data_refcell: &Rc<RefCell<String>>,
    language_changed_action: &gio::SimpleAction,
) {
    let timezone_page = installer_stack_page::InstallerStackPage::new();
    timezone_page.set_page_icon("alarm-symbolic");
    timezone_page.set_back_visible(true);
    timezone_page.set_next_visible(true);
    timezone_page.set_back_sensitive(true);
    timezone_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let null_checkbutton = gtk::CheckButton::builder().build();

    let timezone_selection_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    timezone_selection_row_viewport_listbox.add_css_class("boxed-list");
    timezone_selection_row_viewport_listbox.add_css_class("round-all-scroll");

    let timezone_selection_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&timezone_selection_row_viewport_listbox)
        .build();

    timezone_selection_row_viewport.add_css_class("round-all-scroll");

    let timezone_search_bar = gtk::SearchEntry::builder()
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .search_delay(500)
        .build();

    timezone_search_bar.add_css_class("rounded-all-25");

    let current_timezone_cli = Command::new("timedatectl")
        .arg("show")
        .arg("--va")
        .arg("-p")
        .arg("Timezone")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let current_timezone_output = current_timezone_cli.wait_with_output().unwrap();
    let current_timezone = String::from_utf8_lossy(&current_timezone_output.stdout)
        .trim()
        .to_owned();

    let timezone_cli = Command::new("timedatectl")
        .arg("list-timezones")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let timezone_stdout = timezone_cli.stdout.expect("could not get stdout");
    let timezone_reader = std::io::BufReader::new(timezone_stdout);

    for timezone in timezone_reader.lines() {
        let timezone = timezone.unwrap();
        let timezone_clone = timezone.clone();
        let timezone_checkbutton = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        let timezone_row = adw::ActionRow::builder()
            .activatable_widget(&timezone_checkbutton)
            .title(timezone.clone())
            .build();
        timezone_row.add_prefix(&timezone_checkbutton);
        timezone_checkbutton.set_group(Some(&null_checkbutton));
        timezone_selection_row_viewport_listbox.append(&timezone_row);
        timezone_checkbutton.connect_toggled(clone!(
            #[weak]
            timezone_checkbutton,
            #[weak]
            timezone_page,
            #[weak]
            timezone_data_refcell,
            move |_| {
                if timezone_checkbutton.is_active() == true {
                    timezone_page.set_next_sensitive(true);
                    *timezone_data_refcell.borrow_mut() = String::from(&timezone);
                }
            }
        ));
        if &current_timezone == &timezone_clone {
            timezone_checkbutton.set_active(true);
        }
    }

    // / content_box appends
    //// add text and and entry to timezone page selections
    content_box.append(&timezone_search_bar);
    content_box.append(&timezone_selection_row_viewport);

    timezone_search_bar.connect_search_changed(clone!(
        #[weak]
        timezone_search_bar,
        #[weak]
        timezone_selection_row_viewport_listbox,
        move |_| {
            let mut counter = timezone_selection_row_viewport_listbox.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !timezone_search_bar.text().is_empty() {
                        if row
                            .property::<String>("subtitle")
                            .to_lowercase()
                            .contains(&timezone_search_bar.text().to_string().to_lowercase())
                            || row
                                .property::<String>("title")
                                .to_lowercase()
                                .contains(&timezone_search_bar.text().to_string().to_lowercase())
                        {
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
        }
    ));

    timezone_page.set_child_widget(&content_box);

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        timezone_page,
        #[weak]
        timezone_search_bar,
        move |_, _| {
            timezone_page.set_page_title(t!("timezone"));
            timezone_page.set_page_subtitle(t!("select_a_timezone"));
            timezone_page.set_back_tooltip_label(t!("back"));
            timezone_page.set_next_tooltip_label(t!("next"));
            //
            timezone_search_bar.set_placeholder_text(Some(&t!("search_for_timezone")));
        }
    ));
    //

    timezone_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_timezone_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(3), true)
            }
        ),
    );

    timezone_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            #[strong]
            timezone_data_refcell,
            move |_timezone_page: installer_stack_page::InstallerStackPage| {
                let timezone = timezone_data_refcell.borrow();
                Command::new("sudo")
                    .arg("timedatectl")
                    .arg("set-timezone")
                    .arg(timezone.to_owned())
                    .spawn()
                    .expect("timezone failed to start");
                main_carousel.scroll_to(&main_carousel.nth_page(5), true)
            }
        ),
    );

    main_carousel.append(&timezone_page);
}
