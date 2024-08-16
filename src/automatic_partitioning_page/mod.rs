use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use crate::partitioning_page::{get_block_devices};
use adw::{prelude::*};
use glib::{clone, closure_local, ffi::gboolean};
use std::{rc::Rc, cell::RefCell};

const BOOT_AND_EFI_BYTE_SIZE: f64 = 1611661312.0;
const MINIMUM_ROOT_BYTE_SIZE: f64 = 40000000000.0;

pub fn automatic_partitioning_page(
    main_carousel: &adw::Carousel,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_refcell:  &Rc<RefCell<String>>,
    partition_method_automatic_target_fs_refcell:  &Rc<RefCell<String>>,
    partition_method_automatic_luks_enabled_refcell:  &Rc<RefCell<bool>>,
    partition_method_automatic_luks_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_ratio_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_seperation_refcell: &Rc<RefCell<String>>,
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

    // Advanced

    let advanced_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let advanced_box_viewport =
        gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .child(&advanced_box)
            .build();

    //

    let advanced_home_part_ratio_selection_box =  gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let advanced_home_part_ratio_label_root = gtk::Label::builder()
        .build();

    advanced_home_part_ratio_label_root.add_css_class("accent-text");

    let advanced_home_part_ratio_label_home = gtk::Label::builder()
        .build();

    advanced_home_part_ratio_label_home.add_css_class("green-text");

    let advanced_home_part_ratio_selection_frame =  gtk::Frame::builder()
        .label("/ to /home ratio")
        .child(&advanced_home_part_ratio_selection_box)
        .hexpand(true)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    let advanced_home_part_ratio_selection_slider= gtk::Scale::builder()
        .draw_value(false)
        .build();

    advanced_home_part_ratio_selection_slider.add_css_class("green-trough");
        
    let advanced_home_part_ratio_label_root_clone0 = advanced_home_part_ratio_label_root.clone();
    let advanced_home_part_ratio_label_home_clone0 = advanced_home_part_ratio_label_home.clone();

    advanced_home_part_ratio_selection_slider.connect_change_value(move |slider, _, value| {
        let home_size: f64 = slider.adjustment().upper() + 10000000000.0 - value;
        advanced_home_part_ratio_label_root_clone0.set_label(&format!("{}: {}", t!("Root Part Size"), pretty_bytes::converter::convert(value.into())));
        advanced_home_part_ratio_label_home_clone0.set_label(&format!("{}: {}", t!("Home Part Size"), pretty_bytes::converter::convert(home_size.into())));
        glib::Propagation::Proceed
    });

    //

    let advanced_home_seperation_selection_box =  gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .build();

    let advanced_home_seperation_selection_frame =  gtk::Frame::builder()
        .label("/home seperation")
        .child(&advanced_home_seperation_selection_box)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    let advanced_home_seperation_selection_checkbutton_subvol = gtk::CheckButton::builder()
        .label("subvol")
        .active(true)
        .build();

    let advanced_home_seperation_selection_checkbutton_partition = gtk::CheckButton::builder()
        .label("partition")
        .build();

    let advanced_home_seperation_selection_checkbutton_none = gtk::CheckButton::builder()
        .label("none")
        .build();

    advanced_home_seperation_selection_checkbutton_partition.set_group(Some(&advanced_home_seperation_selection_checkbutton_subvol));
    advanced_home_seperation_selection_checkbutton_none.set_group(Some(&advanced_home_seperation_selection_checkbutton_subvol));

    advanced_home_seperation_selection_checkbutton_partition
        .bind_property(
            "active",
            &advanced_home_part_ratio_selection_frame,
            "sensitive",
        )
    .sync_create()
    .build();

    //

    let advanced_filesystem_selection_box =  gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .build();

    let advanced_filesystem_selection_frame =  gtk::Frame::builder()
        .label("Filesystem")
        .child(&advanced_filesystem_selection_box)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    let advanced_filesystem_selection_checkbutton_btrfs = gtk::CheckButton::builder()
        .label("BTRFS")
        .active(true)
        .build();

    let advanced_filesystem_selection_checkbutton_ext4 = gtk::CheckButton::builder()
        .label("EXT4")
        .build();

    let advanced_filesystem_selection_checkbutton_xfs = gtk::CheckButton::builder()
        .label("XFS")
        .build();

    advanced_filesystem_selection_checkbutton_btrfs
        .bind_property(
            "active",
            &advanced_home_seperation_selection_checkbutton_subvol,
            "sensitive",
        )
    .sync_create()
    .build();

    advanced_filesystem_selection_checkbutton_ext4.connect_toggled(clone!(
        #[weak]
        advanced_filesystem_selection_checkbutton_ext4,
        #[weak]
        advanced_home_seperation_selection_checkbutton_subvol,
        #[weak]
        advanced_home_seperation_selection_checkbutton_partition,
        move |_|
            {
                if advanced_filesystem_selection_checkbutton_ext4.is_active() && advanced_home_seperation_selection_checkbutton_subvol.is_active() {
                    advanced_home_seperation_selection_checkbutton_partition.set_active(true)
                }
            }
     )
    );

    advanced_filesystem_selection_checkbutton_xfs.connect_toggled(clone!(
        #[weak]
        advanced_filesystem_selection_checkbutton_xfs,
        #[weak]
        advanced_home_seperation_selection_checkbutton_subvol,
        #[weak]
        advanced_home_seperation_selection_checkbutton_partition,
        move |_|
            {
                if advanced_filesystem_selection_checkbutton_xfs.is_active() && advanced_home_seperation_selection_checkbutton_subvol.is_active() {
                    advanced_home_seperation_selection_checkbutton_partition.set_active(true)
                }
            }
     )
    );


    advanced_filesystem_selection_checkbutton_ext4.set_group(Some(&advanced_filesystem_selection_checkbutton_btrfs));
    advanced_filesystem_selection_checkbutton_xfs.set_group(Some(&advanced_filesystem_selection_checkbutton_btrfs));

    //

    let advanced_expander = gtk::Expander::builder()
        .child(&advanced_box_viewport)
        .build();

    //

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

    let partition_method_automatic_disk_nodisk_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .build();
    partition_method_automatic_disk_nodisk_error_label.add_css_class("small_error_text");

    let partition_method_automatic_disk_small_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_automatic_disk_small_error_label.add_css_class("small_error_text");

    let partition_method_automatic_luks_empty_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_automatic_luks_empty_error_label.add_css_class("small_error_text");

    let partition_method_automatic_luks_missmatch_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_automatic_luks_missmatch_error_label.add_css_class("small_error_text");

    //

    let error_labels = [partition_method_automatic_disk_nodisk_error_label.clone(), partition_method_automatic_disk_small_error_label.clone(), partition_method_automatic_luks_empty_error_label.clone(), partition_method_automatic_luks_missmatch_error_label.clone()];

    //

    let partition_method_automatic_luks_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let partition_method_automatic_luks_checkbutton = gtk::CheckButton::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .valign(gtk::Align::Start)
        .build();

    let partition_method_automatic_luks_listbox = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();
    partition_method_automatic_luks_listbox.add_css_class("boxed-list");

    let partition_method_automatic_luks_password_entry = adw::PasswordEntryRow::builder()
        .hexpand(true)
        .sensitive(false)
        .build();

    let partition_method_automatic_luks_password_confirm_entry = adw::PasswordEntryRow::builder()
        .hexpand(true)
        .sensitive(true)
        .visible(false)
        .build();

    partition_method_automatic_luks_password_entry
        .bind_property(
            "sensitive",
            &partition_method_automatic_luks_password_confirm_entry,
            "visible",
        )
        .sync_create()
        .build();

    partition_method_automatic_luks_checkbutton
        .bind_property(
            "active",
            &partition_method_automatic_luks_password_entry,
            "sensitive",
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
                devices_selection_expander_row,
                #[weak]
                partition_method_automatic_disk_nodisk_error_label,
                #[weak]
                partition_method_automatic_disk_small_error_label,
                #[weak]
                advanced_home_part_ratio_selection_slider,
                #[strong]
                partition_method_automatic_target_refcell,
                #[strong]
                error_labels,
                #[weak]
                automatic_partitioning_page,
                move |_| {
                    disk_check(&device_button, &devices_selection_expander_row, &partition_method_automatic_disk_small_error_label, &device.block_name, device.block_size);
                    partition_method_automatic_disk_nodisk_error_label.set_visible(false);
                    let usable_disk_space = device.block_size - BOOT_AND_EFI_BYTE_SIZE;
                    let default_root_size = if (usable_disk_space * 40.0) / 100.0 > 100000000000.0 {
                        100000000000.0
                    } else {
                        MINIMUM_ROOT_BYTE_SIZE
                    };
                    advanced_home_part_ratio_selection_slider.set_range(MINIMUM_ROOT_BYTE_SIZE, device.block_size - 10000000000.0);
                    advanced_home_part_ratio_selection_slider.set_value(default_root_size);
                    advanced_home_part_ratio_selection_slider.emit_by_name_with_values("change_value", &[gtk::ScrollType::None.into(), default_root_size.into()]);
                    *partition_method_automatic_target_refcell.borrow_mut() = String::from(&device.block_name);
                    if check_for_errors(&error_labels) {
                        automatic_partitioning_page.set_next_sensitive(true)
                    } else {
                        automatic_partitioning_page.set_next_sensitive(false)
                    }
                }
            )
        );
    }

    partition_method_automatic_luks_checkbutton.connect_toggled(
        clone!(
            #[strong]
            partition_method_automatic_luks_missmatch_error_label,
            #[strong]
            partition_method_automatic_luks_checkbutton,
            #[strong]
            partition_method_automatic_luks_password_confirm_entry,
            #[strong]
            partition_method_automatic_luks_password_entry,
            #[strong]
            partition_method_automatic_luks_empty_error_label,
            #[strong]
            partition_method_automatic_luks_enabled_refcell,
            #[strong]
            error_labels,
            #[weak]
            automatic_partitioning_page,
            move |_| {
                match partition_method_automatic_luks_checkbutton.is_active() {
                    true => *partition_method_automatic_luks_enabled_refcell.borrow_mut() = true,
                    false => *partition_method_automatic_luks_enabled_refcell.borrow_mut() = false,
                }
                luks_check(&partition_method_automatic_luks_checkbutton, &partition_method_automatic_luks_password_entry, &partition_method_automatic_luks_password_confirm_entry, &partition_method_automatic_luks_missmatch_error_label, &partition_method_automatic_luks_empty_error_label);
                if check_for_errors(&error_labels) {
                    automatic_partitioning_page.set_next_sensitive(true)
                } else {
                    automatic_partitioning_page.set_next_sensitive(false)
                }
            }
        )
    );

    partition_method_automatic_luks_password_entry.connect_changed(
        clone!(
            #[weak]
            partition_method_automatic_luks_missmatch_error_label,
            #[weak]
            partition_method_automatic_luks_checkbutton,
            #[weak]
            partition_method_automatic_luks_password_confirm_entry,
            #[weak]
            partition_method_automatic_luks_password_entry,
            #[weak]
            partition_method_automatic_luks_empty_error_label,
            #[strong]
            partition_method_automatic_luks_refcell,
            #[strong]
            error_labels,
            #[weak]
            automatic_partitioning_page,
            move |_| {
                *partition_method_automatic_luks_refcell.borrow_mut() = String::from(partition_method_automatic_luks_password_entry.text());
                luks_check(&partition_method_automatic_luks_checkbutton, &partition_method_automatic_luks_password_entry, &partition_method_automatic_luks_password_confirm_entry, &partition_method_automatic_luks_missmatch_error_label, &partition_method_automatic_luks_empty_error_label);
                if check_for_errors(&error_labels) {
                    automatic_partitioning_page.set_next_sensitive(true)
                } else {
                    automatic_partitioning_page.set_next_sensitive(false)
                }
            }
        )
    );

    partition_method_automatic_luks_password_confirm_entry.connect_changed(
        clone!(
            #[weak]
            partition_method_automatic_luks_missmatch_error_label,
            #[weak]
            partition_method_automatic_luks_checkbutton,
            #[weak]
            partition_method_automatic_luks_password_confirm_entry,
            #[weak]
            partition_method_automatic_luks_password_entry,
            #[weak]
            partition_method_automatic_luks_empty_error_label,
            #[strong]
            partition_method_automatic_luks_refcell,
            #[strong]
            error_labels,
            #[weak]
            automatic_partitioning_page,
            move |_| {
                *partition_method_automatic_luks_refcell.borrow_mut() = String::from(partition_method_automatic_luks_password_entry.text());
                luks_check(&partition_method_automatic_luks_checkbutton, &partition_method_automatic_luks_password_entry, &partition_method_automatic_luks_password_confirm_entry, &partition_method_automatic_luks_missmatch_error_label, &partition_method_automatic_luks_empty_error_label);
                if check_for_errors(&error_labels) {
                    automatic_partitioning_page.set_next_sensitive(true)
                } else {
                    automatic_partitioning_page.set_next_sensitive(false)
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

    advanced_home_seperation_selection_box.append(&advanced_home_seperation_selection_checkbutton_subvol);
    advanced_home_seperation_selection_box.append(&advanced_home_seperation_selection_checkbutton_partition);
    advanced_home_seperation_selection_box.append(&advanced_home_seperation_selection_checkbutton_none);

    advanced_filesystem_selection_box.append(&advanced_filesystem_selection_checkbutton_btrfs);
    advanced_filesystem_selection_box.append(&advanced_filesystem_selection_checkbutton_ext4);
    advanced_filesystem_selection_box.append(&advanced_filesystem_selection_checkbutton_xfs);

    advanced_home_part_ratio_selection_box.append(&advanced_home_part_ratio_selection_slider);
    advanced_home_part_ratio_selection_box.append(&advanced_home_part_ratio_label_root);
    advanced_home_part_ratio_selection_box.append(&advanced_home_part_ratio_label_home);

    advanced_box.append(&advanced_home_seperation_selection_frame);
    advanced_box.append(&advanced_filesystem_selection_frame);
    advanced_box.append(&advanced_home_part_ratio_selection_frame);

    content_box.append(&devices_selection_expander_row_viewport_listbox);
    content_box.append(&partition_method_automatic_luks_box);
    content_box.append(&partition_method_automatic_luks_empty_error_label);
    content_box.append(&partition_method_automatic_luks_missmatch_error_label);
    content_box.append(&partition_method_automatic_disk_nodisk_error_label);
    content_box.append(&partition_method_automatic_disk_small_error_label);
    content_box.append(&advanced_expander);

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
            partition_method_type_refcell,
            #[strong]
            partition_method_automatic_target_refcell,
            #[strong]
            partition_method_automatic_target_fs_refcell,
            #[strong]
            partition_method_automatic_luks_refcell,
            #[strong]
            partition_method_automatic_luks_enabled_refcell,
            #[strong]
            partition_method_automatic_ratio_refcell,
            #[strong]
            partition_method_automatic_seperation_refcell,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                //main_carousel.scroll_to(&main_carousel.nth_page(5), true)
                dbg!(partition_method_type_refcell.borrow());
                dbg!(partition_method_automatic_target_fs_refcell.borrow());
                dbg!(partition_method_automatic_target_refcell.borrow());
                dbg!(partition_method_automatic_luks_enabled_refcell.borrow());
                dbg!(partition_method_automatic_luks_refcell.borrow());
                dbg!(partition_method_automatic_ratio_refcell.borrow());
                dbg!(partition_method_automatic_seperation_refcell.borrow());
            }
        )
    );
    //

    automatic_partitioning_page.set_child_widget(&content_box);

    main_carousel.append(&automatic_partitioning_page);

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        partition_method_automatic_disk_nodisk_error_label,
        #[weak]
        partition_method_automatic_disk_small_error_label,
        #[weak]
        partition_method_automatic_luks_empty_error_label,
        #[weak]
        partition_method_automatic_luks_missmatch_error_label,
            move |_, _| {
                automatic_partitioning_page.set_page_title(t!("auto_part_installer"));
                automatic_partitioning_page.set_page_subtitle(t!("choose_drive_auto"));
                automatic_partitioning_page.set_back_tooltip_label(t!("back"));
                automatic_partitioning_page.set_next_tooltip_label(t!("next"));
                //
                devices_selection_expander_row.set_title(&t!("no_drive_auto_selected"));
                //
                partition_method_automatic_disk_nodisk_error_label.set_label(&t!("no_disk_specified"));
                //
                partition_method_automatic_disk_small_error_label.set_label(&t!("disk_auto_target_small"));
                //
                partition_method_automatic_luks_empty_error_label.set_label(&t!("luks_yes_but_empty"));
                //
                partition_method_automatic_luks_missmatch_error_label.set_label(&t!("luks_not_match"));
                //
                partition_method_automatic_luks_checkbutton.set_label(Some(&t!("enable_luks2_enc")));
                //
                partition_method_automatic_luks_password_entry.set_title(&t!("luks2_password"));
                //
                partition_method_automatic_luks_password_confirm_entry.set_title(&t!("luks2_password_confirm"));
                //
                advanced_expander.set_label(Some(&t!("advanced_options")));
            }
        )
    );
    //
}

fn disk_check(device_button: &gtk::CheckButton ,devices_selection_expander_row: &adw::ExpanderRow, partition_method_automatic_disk_size_error_label: &gtk::Label, device_block_name: &str, device_block_size: f64) {
    if device_button.is_active() == true {
        devices_selection_expander_row.set_title(device_block_name);
        if device_block_size > 39000000000.0 {
            partition_method_automatic_disk_size_error_label.set_visible(false);
        } else {
            partition_method_automatic_disk_size_error_label.set_visible(true);
        }
    }
}

fn luks_check(partition_method_automatic_luks_checkbutton: &gtk::CheckButton, partition_method_automatic_luks_password_entry: &adw::PasswordEntryRow, partition_method_automatic_luks_password_confirm_entry: &adw::PasswordEntryRow, partition_method_automatic_luks_missmatch_error_label: &gtk::Label, partition_method_automatic_luks_empty_error_label: &gtk::Label) {
    if partition_method_automatic_luks_checkbutton.is_active() == true {
        if partition_method_automatic_luks_password_entry.text() != partition_method_automatic_luks_password_confirm_entry.text() {
            partition_method_automatic_luks_missmatch_error_label.set_visible(true)
        } else {
            partition_method_automatic_luks_missmatch_error_label.set_visible(false)
        }
        if partition_method_automatic_luks_password_entry.text().to_string().is_empty() {
            partition_method_automatic_luks_empty_error_label.set_visible(true);
        } else {
            partition_method_automatic_luks_empty_error_label.set_visible(false);
        }
    } else {
        partition_method_automatic_luks_empty_error_label.set_visible(false);
        partition_method_automatic_luks_missmatch_error_label.set_visible(false);
    }
}

fn check_for_errors(error_labels: &[gtk::Label]) -> bool {
    for label in error_labels {
        if label.is_visible() {
            return false
        }
    }
    true
}