use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use crate::partitioning_page::{get_block_devices};
use adw::{prelude::*};
use glib::{clone, closure_local};

pub fn automatic_partitioning_page(
    main_carousel: &adw::Carousel,
    partition_method_type_buffer: &gtk::TextBuffer,
    partition_method_automatic_target_buffer:  &gtk::TextBuffer,
    partition_method_automatic_luks_buffer:  &gtk::TextBuffer,
    partition_method_automatic_ratio_buffer: &gtk::TextBuffer,
    partition_method_automatic_seperation_buffer: &gtk::TextBuffer,
    language_changed_action: &gio::SimpleAction
) {
    let automatic_partitioning_page = installer_stack_page::InstallerStackPage::new();
    automatic_partitioning_page.set_page_icon("builder");
    automatic_partitioning_page.set_back_visible(true);
    automatic_partitioning_page.set_next_visible(true);
    automatic_partitioning_page.set_back_sensitive(true);
    automatic_partitioning_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    automatic_partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(0), true)
            }
        )
    );

    //

    let devices_selection_expander_row = adw::ExpanderRow::builder()
        .name("status:disk=none,")
        .build();

    let devices_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    devices_selection_expander_row_viewport_listbox.add_css_class("boxed-list");

    let null_checkbutton = gtk::CheckButton::builder().build();

    let devices_selection_expander_row_viewport_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    devices_selection_expander_row_viewport_box.add_css_class("boxed-list");
    devices_selection_expander_row_viewport_box.add_css_class("round-all-scroll");

    let devices_selection_expander_row_viewport =
    gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&devices_selection_expander_row_viewport_box)
        .build();

    devices_selection_expander_row_viewport.add_css_class("round-all-scroll");

    devices_selection_expander_row.add_row(&devices_selection_expander_row_viewport);

    let partition_method_automatic_disk_error_label = gtk::Label::builder()
        .name("status:no_disk_specified,")
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();
    partition_method_automatic_disk_error_label.add_css_class("small_error_text");

    let partition_method_automatic_luks_error_label = gtk::Label::builder()
        .name("status:luks_yes_but_empty,")
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_automatic_luks_error_label.add_css_class("small_error_text");

    let partition_method_automatic_luks_error2_label = gtk::Label::builder()
        .name("status:luks_not_match,")
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .vexpand(true)
        .visible(false)
        .build();
    partition_method_automatic_luks_error2_label.add_css_class("small_error_text");

    //

    let partition_method_automatic_luks_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let partition_method_automatic_luks_checkbutton = gtk::CheckButton::builder()
        .label(t!("enable_luks2_enc"))
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
        .title(t!("luks2_password"))
        .hexpand(true)
        .sensitive(false)
        .build();

    let partition_method_automatic_luks_password_confirm_entry = adw::PasswordEntryRow::builder()
        .title(t!("luks2_password_confirm"))
        .hexpand(true)
        .sensitive(true)
        .visible(false)
        .build();

    let _partition_method_automatic_luks_password = partition_method_automatic_luks_password_entry
        .bind_property(
            "sensitive",
            &partition_method_automatic_luks_password_confirm_entry,
            "visible",
        )
        .sync_create()
        .build();

    //

    for device in get_block_devices() {
        let device_button = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        device_button.set_group(Some(&null_checkbutton));
        let device_row = adw::ActionRow::builder()
            .activatable_widget(&device_button)
            .title(&device.block_name)
            .subtitle(&device.block_size_pretty)
            .build();
        device_row.add_prefix(&device_button);
        devices_selection_expander_row_viewport_box.append(&device_row);
        device_button.connect_toggled(
            clone!(
                #[weak]
                device_button,
                #[weak]
                partition_method_automatic_luks_password_entry,
                #[weak]
                devices_selection_expander_row,
                #[weak]
                automatic_partitioning_page,
                #[weak]
                partition_method_automatic_disk_error_label,
                #[weak]
                partition_method_automatic_luks_error_label,
                #[weak]
                partition_method_automatic_luks_checkbutton,
                #[strong]
                partition_method_automatic_target_buffer,
                #[strong]
                partition_method_automatic_luks_buffer,
                move |_| {
                    if device_button.is_active() == true {
                        devices_selection_expander_row.set_title(&device.block_name);
                        if device.block_size > 39000000000.0 {
                            partition_method_automatic_disk_error_label.set_visible(false);
                            if partition_method_automatic_luks_checkbutton.is_active() == true {
                                if partition_method_automatic_luks_error_label.get_visible() {
                                    //
                                } else {
                                    automatic_partitioning_page.set_next_sensitive(true);
                                }
                            }  else {
                                partition_method_automatic_target_buffer.set_text(&device.block_name);
                                partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                                automatic_partitioning_page.set_next_sensitive(true);
                            }
                        } else {
                            partition_method_automatic_disk_error_label.set_visible(true);
                            partition_method_automatic_disk_error_label.set_widget_name("status:disk=small,");
                            partition_method_automatic_disk_error_label.set_label(&t!("disk_auto_target_small"));
                            automatic_partitioning_page.set_next_sensitive(false);
                        }
                    }
                }));
    }

    partition_method_automatic_luks_checkbutton.connect_toggled(
        clone!(
            #[strong]
            partition_method_automatic_luks_error2_label,
            #[strong]
            partition_method_automatic_luks_checkbutton,
            #[strong]
            partition_method_automatic_luks_password_confirm_entry,
            #[strong]
            partition_method_automatic_luks_password_entry,
            #[strong]
            partition_method_automatic_disk_error_label,
            #[strong]
            partition_method_automatic_luks_error_label,
            #[weak]
            automatic_partitioning_page,
            #[strong]
            partition_method_automatic_luks_buffer,
            move |_| {
                if partition_method_automatic_luks_checkbutton.is_active() == true {
                    partition_method_automatic_luks_password_entry.set_sensitive(true);
                    if partition_method_automatic_luks_password_entry.text() != partition_method_automatic_luks_password_confirm_entry.text() {
                        partition_method_automatic_luks_error2_label.set_visible(true)
                    } else {
                        partition_method_automatic_luks_error2_label.set_visible(false)
                    }
                    if partition_method_automatic_luks_password_entry.text().to_string().is_empty() {
                        partition_method_automatic_luks_error_label.set_visible(true);
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    } else {
                        partition_method_automatic_luks_error_label.set_visible(false);
                    if !partition_method_automatic_disk_error_label.get_visible() && !partition_method_automatic_luks_error_label.get_visible() && !partition_method_automatic_luks_error2_label.get_visible() {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(true);
                    } else {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    }
                    }
                } else {
                    partition_method_automatic_luks_password_entry.set_sensitive(false);
                    partition_method_automatic_luks_error_label.set_visible(false);
                    partition_method_automatic_luks_error2_label.set_visible(false);
                    if !partition_method_automatic_disk_error_label.get_visible() && !partition_method_automatic_luks_error_label.get_visible() && !partition_method_automatic_luks_error2_label.get_visible() {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(true);
                    } else {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    }
                }
            }
        )
    );

    partition_method_automatic_luks_password_entry.connect_changed(
        clone!(
            #[weak]
            partition_method_automatic_luks_error2_label,
            #[weak]
            partition_method_automatic_luks_checkbutton,
            #[weak]
            partition_method_automatic_luks_password_confirm_entry,
            #[weak]
            partition_method_automatic_luks_password_entry,
            #[weak] 
            partition_method_automatic_disk_error_label,
            #[weak]
            partition_method_automatic_luks_error_label,
            #[weak]
            automatic_partitioning_page,
            #[strong]
            partition_method_automatic_luks_buffer,
            move |_| {
                if partition_method_automatic_luks_checkbutton.is_active() == true {
                    partition_method_automatic_luks_password_entry.set_sensitive(true);
                    if partition_method_automatic_luks_password_entry.text() != partition_method_automatic_luks_password_confirm_entry.text() {
                        partition_method_automatic_luks_error2_label.set_visible(true)
                    } else {
                        partition_method_automatic_luks_error2_label.set_visible(false)
                    }
                    if partition_method_automatic_luks_password_entry.text().to_string().is_empty() {
                        partition_method_automatic_luks_error_label.set_visible(true);
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    } else {
                        partition_method_automatic_luks_error_label.set_visible(false);
                    if !partition_method_automatic_disk_error_label.get_visible() && !partition_method_automatic_luks_error_label.get_visible() && !partition_method_automatic_luks_error2_label.get_visible() {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(true);
                    } else {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    }
                    }
                } else {
                    partition_method_automatic_luks_password_entry.set_sensitive(false);
                    partition_method_automatic_luks_error_label.set_visible(false);
                    partition_method_automatic_luks_error2_label.set_visible(false);
                    if !partition_method_automatic_disk_error_label.get_visible() && !partition_method_automatic_luks_error_label.get_visible() && !partition_method_automatic_luks_error2_label.get_visible() {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(true);
                    } else {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    }
                }
            }
        )
    );

    partition_method_automatic_luks_password_confirm_entry.connect_changed(
        clone!(
            #[weak]
            partition_method_automatic_luks_error2_label,
            #[weak]
            partition_method_automatic_luks_checkbutton,
            #[weak]
            partition_method_automatic_luks_password_confirm_entry,
            #[weak]
            partition_method_automatic_luks_password_entry,
            #[weak]
            partition_method_automatic_disk_error_label,
            #[weak]
            partition_method_automatic_luks_error_label,
            #[weak]
            automatic_partitioning_page,
            #[strong]
            partition_method_automatic_luks_buffer,
            move |_| {
                if partition_method_automatic_luks_checkbutton.is_active() == true {
                    partition_method_automatic_luks_password_entry.set_sensitive(true);
                    if partition_method_automatic_luks_password_entry.text() != partition_method_automatic_luks_password_confirm_entry.text() {
                        partition_method_automatic_luks_error2_label.set_visible(true)
                    } else {
                        partition_method_automatic_luks_error2_label.set_visible(false)
                    }
                    if partition_method_automatic_luks_password_entry.text().to_string().is_empty() {
                        partition_method_automatic_luks_error_label.set_visible(true);
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    } else {
                        partition_method_automatic_luks_error_label.set_visible(false);
                    if !partition_method_automatic_disk_error_label.get_visible() && !partition_method_automatic_luks_error_label.get_visible() && !partition_method_automatic_luks_error2_label.get_visible() {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(true);
                    } else {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    }
                    }
                } else {
                    partition_method_automatic_luks_password_entry.set_sensitive(false);
                    partition_method_automatic_luks_error_label.set_visible(false);
                    partition_method_automatic_luks_error2_label.set_visible(false);
                    if !partition_method_automatic_disk_error_label.get_visible() && !partition_method_automatic_luks_error_label.get_visible() && !partition_method_automatic_luks_error2_label.get_visible() {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(true);
                    } else {
                        partition_method_automatic_luks_buffer.set_text(&partition_method_automatic_luks_password_entry.text().to_string());
                        automatic_partitioning_page.set_next_sensitive(false);
                    }
                }
            }
        )
    );

    //

    devices_selection_expander_row_viewport_listbox.append(&devices_selection_expander_row);

    partition_method_automatic_luks_listbox.append(&partition_method_automatic_luks_password_entry);
    partition_method_automatic_luks_listbox.append(&partition_method_automatic_luks_password_confirm_entry);

    partition_method_automatic_luks_box.append(&partition_method_automatic_luks_checkbutton);
    partition_method_automatic_luks_box.append(&partition_method_automatic_luks_listbox);

    content_box.append(&devices_selection_expander_row_viewport_listbox);
    content_box.append(&partition_method_automatic_luks_box);
    content_box.append(&partition_method_automatic_luks_error_label);
    content_box.append(&partition_method_automatic_luks_error2_label);
    content_box.append(&partition_method_automatic_disk_error_label);

    //
    language_changed_action.connect_activate(
        clone!(
            #[weak]
            automatic_partitioning_page,
            #[weak]
            devices_selection_expander_row,
            move |_, _| {
                automatic_partitioning_page.set_page_title(t!("auto_part_installer"));
                automatic_partitioning_page.set_page_subtitle(t!("choose_drive_auto"));
                automatic_partitioning_page.set_back_tooltip_label(t!("back"));
                automatic_partitioning_page.set_next_tooltip_label(t!("next"));
                //
                if devices_selection_expander_row.widget_name() == "status:disk=none," {
                    devices_selection_expander_row.set_title(&t!("no_drive_auto_selected"));
                }
                //
                match partition_method_automatic_disk_error_label.widget_name().as_str() {
                    "status:no_disk_specified," => {
                        partition_method_automatic_disk_error_label.set_label(&t!("no_disk_specified"));
                    }
                    "status:disk=small," => {
                        partition_method_automatic_disk_error_label.set_label(&t!("disk_auto_target_small"));
                    }
                    _ => {}
                }
                //
                if partition_method_automatic_luks_error_label.widget_name() == "status:luks_yes_but_empty," {
                    partition_method_automatic_luks_error_label.set_label(&t!("luks_yes_but_empty"));
                }
                //
                if partition_method_automatic_luks_error2_label.widget_name() == "status:luks_not_match," {
                    partition_method_automatic_luks_error2_label.set_label(&t!("luks_not_match"));
                }
            }
        )
    );
    //

    automatic_partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(0), true)
            }
        )
    );

    automatic_partitioning_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            #[strong]
            partition_method_type_buffer,
            #[strong]
            partition_method_automatic_target_buffer,
            #[strong]
            partition_method_automatic_luks_buffer,
            #[strong]
            partition_method_automatic_ratio_buffer,
            #[strong]
            partition_method_automatic_seperation_buffer,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage|
            {;
                //main_carousel.scroll_to(&main_carousel.nth_page(5), true)
                dbg!(partition_method_type_buffer.text(&partition_method_type_buffer.bounds().0, &partition_method_type_buffer.bounds().1, true).to_string());
                dbg!(partition_method_automatic_target_buffer.text(&partition_method_automatic_target_buffer.bounds().0, &partition_method_automatic_target_buffer.bounds().1, true).to_string());
                dbg!(partition_method_automatic_luks_buffer.text(&partition_method_automatic_luks_buffer.bounds().0, &partition_method_automatic_luks_buffer.bounds().1, true).to_string());
                dbg!(partition_method_automatic_ratio_buffer.text(&partition_method_automatic_ratio_buffer.bounds().0, &partition_method_automatic_ratio_buffer.bounds().1, true).to_string());
                dbg!(partition_method_automatic_seperation_buffer.text(&partition_method_automatic_seperation_buffer.bounds().0, &partition_method_automatic_seperation_buffer.bounds().1, true).to_string());
            }
        )
    );


    //


    automatic_partitioning_page.set_child_widget(&content_box);

    main_carousel.append(&automatic_partitioning_page);
}