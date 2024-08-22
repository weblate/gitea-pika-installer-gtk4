use crate::{build_ui::{PikaLocale, PikaKeymap, FstabEntry, CrypttabEntry}, installer_stack_page};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, fs, ops::Deref, path::Path, process::Command, rc::Rc};

pub fn installation_summary_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction,
    page_done_action: &gio::SimpleAction,
    language_summary_text_refcell: &Rc<RefCell<PikaLocale>>,
    keymap_selection_text_refcell: &Rc<RefCell<PikaKeymap>>,
    timezone_selection_text_refcell: &Rc<RefCell<String>>,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_fs_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_automatic_luks_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_ratio_refcell: &Rc<RefCell<f64>>,
    partition_method_automatic_seperation_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>
) {
    let installation_summary_page = installer_stack_page::InstallerStackPage::new();
    installation_summary_page.set_page_icon("dialog-warning-symbolic");
    installation_summary_page.set_back_sensitive(true);
    installation_summary_page.set_next_sensitive(false);
    installation_summary_page.set_back_visible(true);
    installation_summary_page.set_next_visible(true);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let installation_summary_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    installation_summary_row_viewport_listbox.add_css_class("boxed-list");
    installation_summary_row_viewport_listbox.add_css_class("round-all-scroll");

    let installation_summary_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&installation_summary_row_viewport_listbox)
        .build();

    installation_summary_row_viewport.add_css_class("round-all-scroll");

    let install_confirm_button = gtk::Button::builder()
    .margin_top(15)
    .margin_bottom(15)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    install_confirm_button.add_css_class("destructive-action");
    install_confirm_button.add_css_class("rounded-all-25-with-padding");

    // / content_box appends
    //// add text and and entry to installation_summary page selections
    content_box.append(&installation_summary_row_viewport);
    content_box.append(&install_confirm_button);

    installation_summary_page.set_child_widget(&content_box);

    //

    page_done_action.connect_activate(clone!(
        #[strong]
        installation_summary_row_viewport_listbox,
        #[strong]
        language_summary_text_refcell,
        #[strong]
        keymap_selection_text_refcell,
        #[strong]
        timezone_selection_text_refcell,
        #[strong]
        partition_method_type_refcell,
        #[strong]
        partition_method_automatic_luks_enabled_refcell,
        #[strong]
        partition_method_manual_luks_enabled_refcell,
        move|_, action_arg|
            {
                let action_arg = String::from_utf8_lossy(action_arg.unwrap().data());
                if action_arg == "partitioning_done" {
                    while let Some(row) = installation_summary_row_viewport_listbox.last_child() {
                        installation_summary_row_viewport_listbox.remove(&row);
                    }
                    //
                    let partition_method_automatic_luks_enabled = partition_method_automatic_luks_enabled_refcell.borrow();
                    let partition_method_manual_luks_enabled = partition_method_manual_luks_enabled_refcell.borrow();
                    //
                    let install_confirm_detail_language = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_language_title"))
                        .subtitle(&language_summary_text_refcell.borrow().pretty_name)
                        .build();
                    install_confirm_detail_language.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_language);
                    //
                    let install_confirm_detail_keymap = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_keymap_title"))
                        .subtitle(&keymap_selection_text_refcell.borrow().pretty_name)
                        .build();
                    install_confirm_detail_keymap.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_keymap);
                    //
                    let install_confirm_detail_timezone = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_timezone_title"))
                        .subtitle(&timezone_selection_text_refcell.borrow().to_string())
                        .build();
                    install_confirm_detail_timezone.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_timezone);
                    //
                    let install_confirm_detail_partition_method_type_subtitle = match &*partition_method_type_refcell.borrow().as_str() {
                        "automatic" => {
                            if *partition_method_automatic_luks_enabled {
                                t!("install_confirm_detail_partition_method_type_subtitle_automatic_luks").to_string()
                            } else {
                                t!("install_confirm_detail_partition_method_type_subtitle_automatic").to_string()
                            }
                        }
                        "manual" => {
                            if *partition_method_manual_luks_enabled {
                                t!("install_confirm_detail_partition_method_type_subtitle_manual_luks").to_string()
                            } else {
                                t!("install_confirm_detail_partition_method_type_subtitle_manual").to_string()
                            }
                        }
                        _ => panic!()
                    };
                    let install_confirm_detail_partition_method_type = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_partition_method_type_title"))
                        .subtitle(&install_confirm_detail_partition_method_type_subtitle)
                        .build();
                    install_confirm_detail_partition_method_type.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_type);
                }
            }
        )
    );

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        installation_summary_page,
        #[weak]
        install_confirm_button,
        move |_, _| {
            installation_summary_page.set_page_title(t!("installation_summary_page_title"));
            installation_summary_page.set_page_subtitle(t!("installation_summary_page_subtitle"));
            installation_summary_page.set_back_tooltip_label(t!("back"));
            installation_summary_page.set_next_tooltip_label(t!("next"));
            //
            install_confirm_button.set_label(&t!("install_confirm_button_label"));
        }
    ));
    //

    installation_summary_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_installation_summary_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(5), true)
            }
        ),
    );

    main_carousel.append(&installation_summary_page);
}
