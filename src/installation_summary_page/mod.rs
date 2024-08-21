use crate::{build_ui::{PikaLocale, PikaKeymap, FstabEntry, CrypttabEntry}, installer_stack_page};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, fs, path::Path, process::Command, rc::Rc};

pub fn installation_summary_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction,
    installation_summary_text_refcell: &Rc<RefCell<PikaLocale>>,
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
    install_confirm_button.add_css_class("circular");

    // / content_box appends
    //// add text and and entry to installation_summary page selections
    content_box.append(&installation_summary_row_viewport_listbox);
    content_box.append(&install_confirm_button);

    installation_summary_page.set_child_widget(&content_box);

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
