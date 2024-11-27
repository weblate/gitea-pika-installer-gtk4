use crate::build_ui::BlockDevice;
use crate::config::{MINIMUM_BOOT_BYTE_SIZE, MINIMUM_EFI_BYTE_SIZE, MINIMUM_ROOT_BYTE_SIZE};
use crate::installer_stack_page;
use crate::partitioning_page::get_block_devices;
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, rc::Rc};

pub fn automatic_partitioning_page(
    main_carousel: &adw::Carousel,
    partition_carousel: &adw::Carousel,
    window: adw::ApplicationWindow,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_refcell: &Rc<RefCell<BlockDevice>>,
    partition_method_automatic_target_fs_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_automatic_luks_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_ratio_refcell: &Rc<RefCell<f64>>,
    partition_method_automatic_seperation_refcell: &Rc<RefCell<String>>,
    language_changed_action: &gio::SimpleAction,
    page_done_action: &gio::SimpleAction,
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

    //

    // Advanced

    let advanced_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let advanced_box_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&advanced_box)
        .build();

    //

    let advanced_home_part_ratio_selection_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let advanced_home_part_ratio_label_root = gtk::Label::builder().build();

    advanced_home_part_ratio_label_root.add_css_class("accent-text");

    let advanced_home_part_ratio_label_home = gtk::Label::builder().build();

    advanced_home_part_ratio_label_home.add_css_class("green-text");

    let advanced_home_part_ratio_selection_frame = gtk::Frame::builder()
        .child(&advanced_home_part_ratio_selection_box)
        .hexpand(true)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    let advanced_home_part_ratio_selection_slider = gtk::Scale::builder().draw_value(false).build();

    advanced_home_part_ratio_selection_slider.add_css_class("green-trough");

    let advanced_home_part_ratio_label_root_clone0 = advanced_home_part_ratio_label_root.clone();
    let advanced_home_part_ratio_label_home_clone0 = advanced_home_part_ratio_label_home.clone();
    let partition_method_automatic_ratio_refcell_clone0 =
        partition_method_automatic_ratio_refcell.clone();

    advanced_home_part_ratio_selection_slider.connect_change_value(move |slider, _, value| {
        let home_size: f64 = slider.adjustment().upper() + 10000000000.0 - value;
        advanced_home_part_ratio_label_root_clone0.set_label(&format!(
            "{}: {}",
            t!("advanced_home_part_ratio_label_root_label"),
            pretty_bytes::converter::convert(value)
        ));
        advanced_home_part_ratio_label_home_clone0.set_label(&format!(
            "{}: {}",
            t!("advanced_home_part_ratio_label_home_label"),
            pretty_bytes::converter::convert(home_size)
        ));
        *partition_method_automatic_ratio_refcell_clone0.borrow_mut() = value;
        glib::Propagation::Proceed
    });

    //

    let advanced_home_seperation_selection_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .build();

    let advanced_home_seperation_selection_frame = gtk::Frame::builder()
        .child(&advanced_home_seperation_selection_box)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    *partition_method_automatic_seperation_refcell.borrow_mut() = String::from("subvol");

    let advanced_home_seperation_selection_checkbutton_subvol =
        gtk::CheckButton::builder().active(true).build();

    let advanced_home_seperation_selection_checkbutton_partition =
        gtk::CheckButton::builder().build();

    let advanced_home_seperation_selection_checkbutton_none = gtk::CheckButton::builder().build();

    advanced_home_seperation_selection_checkbutton_partition
        .set_group(Some(&advanced_home_seperation_selection_checkbutton_subvol));
    advanced_home_seperation_selection_checkbutton_none
        .set_group(Some(&advanced_home_seperation_selection_checkbutton_subvol));

    advanced_home_seperation_selection_checkbutton_partition
        .bind_property(
            "active",
            &advanced_home_part_ratio_selection_frame,
            "sensitive",
        )
        .sync_create()
        .build();

    advanced_home_seperation_selection_checkbutton_subvol.connect_toggled(clone!(
        #[strong]
        partition_method_automatic_seperation_refcell,
        move |_| {
            *partition_method_automatic_seperation_refcell.borrow_mut() = String::from("subvol");
        }
    ));

    advanced_home_seperation_selection_checkbutton_partition.connect_toggled(clone!(
        #[strong]
        partition_method_automatic_seperation_refcell,
        move |_| {
            *partition_method_automatic_seperation_refcell.borrow_mut() = String::from("partition");
        }
    ));

    advanced_home_seperation_selection_checkbutton_none.connect_toggled(clone!(
        #[strong]
        partition_method_automatic_seperation_refcell,
        move |_| {
            *partition_method_automatic_seperation_refcell.borrow_mut() = String::from("none");
        }
    ));

    //

    let advanced_filesystem_selection_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .build();

    let advanced_filesystem_selection_frame = gtk::Frame::builder()
        .child(&advanced_filesystem_selection_box)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    *partition_method_automatic_target_fs_refcell.borrow_mut() = String::from("btrfs");

    let advanced_filesystem_selection_checkbutton_btrfs = gtk::CheckButton::builder()
        .label("BTRFS")
        .active(true)
        .build();

    let advanced_filesystem_selection_checkbutton_ext4 =
        gtk::CheckButton::builder().label("EXT4").build();

    let advanced_filesystem_selection_checkbutton_xfs =
        gtk::CheckButton::builder().label("XFS").build();

    advanced_filesystem_selection_checkbutton_btrfs
        .bind_property(
            "active",
            &advanced_home_seperation_selection_checkbutton_subvol,
            "sensitive",
        )
        .sync_create()
        .build();

    advanced_filesystem_selection_checkbutton_btrfs.connect_toggled(clone!(
        #[strong]
        partition_method_automatic_target_fs_refcell,
        move |_| {
            *partition_method_automatic_target_fs_refcell.borrow_mut() = String::from("btrfs");
        }
    ));

    advanced_filesystem_selection_checkbutton_ext4.connect_toggled(clone!(
        #[weak]
        advanced_filesystem_selection_checkbutton_ext4,
        #[weak]
        advanced_home_seperation_selection_checkbutton_subvol,
        #[weak]
        advanced_home_seperation_selection_checkbutton_partition,
        #[strong]
        partition_method_automatic_target_fs_refcell,
        move |_| {
            if advanced_filesystem_selection_checkbutton_ext4.is_active()
                && advanced_home_seperation_selection_checkbutton_subvol.is_active()
            {
                advanced_home_seperation_selection_checkbutton_partition.set_active(true)
            }
            *partition_method_automatic_target_fs_refcell.borrow_mut() = String::from("ext4");
        }
    ));

    advanced_filesystem_selection_checkbutton_xfs.connect_toggled(clone!(
        #[weak]
        advanced_filesystem_selection_checkbutton_xfs,
        #[weak]
        advanced_home_seperation_selection_checkbutton_subvol,
        #[weak]
        advanced_home_seperation_selection_checkbutton_partition,
        #[strong]
        partition_method_automatic_target_fs_refcell,
        move |_| {
            if advanced_filesystem_selection_checkbutton_xfs.is_active()
                && advanced_home_seperation_selection_checkbutton_subvol.is_active()
            {
                advanced_home_seperation_selection_checkbutton_partition.set_active(true)
            }
            *partition_method_automatic_target_fs_refcell.borrow_mut() = String::from("xfs");
        }
    ));

    advanced_filesystem_selection_checkbutton_ext4
        .set_group(Some(&advanced_filesystem_selection_checkbutton_btrfs));
    advanced_filesystem_selection_checkbutton_xfs
        .set_group(Some(&advanced_filesystem_selection_checkbutton_btrfs));

    //

    let advanced_expander = gtk::Expander::builder()
        .child(&advanced_box_viewport)
        .build();

    //

    //

    let devices_selection_button_row_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    devices_selection_button_row_listbox.add_css_class("boxed-list");

    let devices_selection_button_row = adw::ButtonRow::builder()
        .start_icon_name("drive-harddisk-symbolic")
        .build();

    devices_selection_button_row.add_css_class("accent-blink");

    let null_checkbutton = gtk::CheckButton::builder().build();

    let devices_selection_button_row_viewport_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    devices_selection_button_row_viewport_box.add_css_class("boxed-list");
    devices_selection_button_row_viewport_box.add_css_class("no-round-borders");

    let devices_selection_button_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .overflow(gtk::Overflow::Hidden)
        .vexpand(true)
        .hexpand(true)
        .child(&devices_selection_button_row_viewport_box)
        .build();

    devices_selection_button_row_viewport.add_css_class("round-all-scroll-no-padding");

    let devices_selection_button_row_dialog = adw::AlertDialog::builder()
        .extra_child(&devices_selection_button_row_viewport)
        .width_request(600)
        .height_request(600)
        .build();

    devices_selection_button_row_dialog.add_response(
        "devices_selection_button_row_dialog_ok",
        "devices_selection_button_row_dialog_ok",
    );

    devices_selection_button_row.connect_activated(clone!(
        #[weak]
        devices_selection_button_row_dialog,
        move |_| {
            devices_selection_button_row_dialog.present(Some(&window));
        }
    ));

    //devices_selection_button_row.add_row(&devices_selection_button_row_viewport);

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

    let error_labels = [
        partition_method_automatic_disk_nodisk_error_label.clone(),
        partition_method_automatic_disk_small_error_label.clone(),
        partition_method_automatic_luks_empty_error_label.clone(),
        partition_method_automatic_luks_missmatch_error_label.clone(),
    ];

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
            .title(&device.block_model)
            .subtitle(device.clone().block_name + " " + &device.block_size_pretty)
            .build();
        device_row.add_prefix(&device_button);
        devices_selection_button_row_viewport_box.append(&device_row);
        device_button.connect_toggled(clone!(
            #[weak]
            device_button,
            #[weak]
            devices_selection_button_row,
            #[weak]
            partition_method_automatic_disk_nodisk_error_label,
            #[weak]
            partition_method_automatic_disk_small_error_label,
            #[weak]
            advanced_home_part_ratio_selection_slider,
            #[strong]
            partition_method_automatic_target_refcell,
            #[strong]
            device,
            #[strong]
            error_labels,
            #[weak]
            automatic_partitioning_page,
            move |_| {
                disk_check(
                    &device_button,
                    &devices_selection_button_row,
                    &partition_method_automatic_disk_small_error_label,
                    &device.block_model,
                    &device.block_name,
                    &device.block_size_pretty,
                    device.block_size,
                );
                partition_method_automatic_disk_nodisk_error_label.set_visible(false);
                let usable_disk_space =
                    device.block_size - (MINIMUM_EFI_BYTE_SIZE + MINIMUM_BOOT_BYTE_SIZE);
                let default_root_size = if (usable_disk_space * 40.0) / 100.0 > 100000000000.0 {
                    100000000000.0
                } else if (usable_disk_space * 40.0) / 100.0 < MINIMUM_ROOT_BYTE_SIZE {
                    MINIMUM_ROOT_BYTE_SIZE
                } else {
                    (usable_disk_space * 40.0) / 100.0
                };
                advanced_home_part_ratio_selection_slider
                    .set_range(MINIMUM_ROOT_BYTE_SIZE, usable_disk_space - 10000000000.0);
                advanced_home_part_ratio_selection_slider.set_value(default_root_size);
                advanced_home_part_ratio_selection_slider.emit_by_name_with_values(
                    "change_value",
                    &[gtk::ScrollType::None.into(), default_root_size.into()],
                );
                *partition_method_automatic_target_refcell.borrow_mut() = device.clone();
                if check_for_errors(&error_labels) {
                    automatic_partitioning_page.set_next_sensitive(true)
                } else {
                    automatic_partitioning_page.set_next_sensitive(false)
                }
            }
        ));
    }

    partition_method_automatic_luks_checkbutton.connect_toggled(clone!(
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
            luks_check(
                &partition_method_automatic_luks_checkbutton,
                &partition_method_automatic_luks_password_entry,
                &partition_method_automatic_luks_password_confirm_entry,
                &partition_method_automatic_luks_missmatch_error_label,
                &partition_method_automatic_luks_empty_error_label,
            );
            if check_for_errors(&error_labels) {
                automatic_partitioning_page.set_next_sensitive(true)
            } else {
                automatic_partitioning_page.set_next_sensitive(false)
            }
        }
    ));

    partition_method_automatic_luks_password_entry.connect_changed(clone!(
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
            *partition_method_automatic_luks_refcell.borrow_mut() =
                String::from(partition_method_automatic_luks_password_entry.text());
            luks_check(
                &partition_method_automatic_luks_checkbutton,
                &partition_method_automatic_luks_password_entry,
                &partition_method_automatic_luks_password_confirm_entry,
                &partition_method_automatic_luks_missmatch_error_label,
                &partition_method_automatic_luks_empty_error_label,
            );
            if check_for_errors(&error_labels) {
                automatic_partitioning_page.set_next_sensitive(true)
            } else {
                automatic_partitioning_page.set_next_sensitive(false)
            }
        }
    ));

    partition_method_automatic_luks_password_confirm_entry.connect_changed(clone!(
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
            *partition_method_automatic_luks_refcell.borrow_mut() =
                String::from(partition_method_automatic_luks_password_entry.text());
            luks_check(
                &partition_method_automatic_luks_checkbutton,
                &partition_method_automatic_luks_password_entry,
                &partition_method_automatic_luks_password_confirm_entry,
                &partition_method_automatic_luks_missmatch_error_label,
                &partition_method_automatic_luks_empty_error_label,
            );
            if check_for_errors(&error_labels) {
                automatic_partitioning_page.set_next_sensitive(true)
            } else {
                automatic_partitioning_page.set_next_sensitive(false)
            }
        }
    ));

    //

    devices_selection_button_row_listbox.append(&devices_selection_button_row);

    partition_method_automatic_luks_listbox.append(&partition_method_automatic_luks_password_entry);
    partition_method_automatic_luks_listbox
        .append(&partition_method_automatic_luks_password_confirm_entry);

    partition_method_automatic_luks_box.append(&partition_method_automatic_luks_checkbutton);
    partition_method_automatic_luks_box.append(&partition_method_automatic_luks_listbox);

    advanced_home_seperation_selection_box
        .append(&advanced_home_seperation_selection_checkbutton_subvol);
    advanced_home_seperation_selection_box
        .append(&advanced_home_seperation_selection_checkbutton_partition);
    advanced_home_seperation_selection_box
        .append(&advanced_home_seperation_selection_checkbutton_none);

    advanced_filesystem_selection_box.append(&advanced_filesystem_selection_checkbutton_btrfs);
    advanced_filesystem_selection_box.append(&advanced_filesystem_selection_checkbutton_ext4);
    advanced_filesystem_selection_box.append(&advanced_filesystem_selection_checkbutton_xfs);

    advanced_home_part_ratio_selection_box.append(&advanced_home_part_ratio_selection_slider);
    advanced_home_part_ratio_selection_box.append(&advanced_home_part_ratio_label_root);
    advanced_home_part_ratio_selection_box.append(&advanced_home_part_ratio_label_home);

    advanced_box.append(&advanced_home_seperation_selection_frame);
    advanced_box.append(&advanced_filesystem_selection_frame);
    advanced_box.append(&advanced_home_part_ratio_selection_frame);

    content_box.append(&devices_selection_button_row_listbox);
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
            partition_carousel,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage| {
                partition_carousel.scroll_to(&partition_carousel.nth_page(0), true)
            }
        ),
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
            page_done_action,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage| {
                *partition_method_type_refcell.borrow_mut() = String::from("automatic");
                page_done_action.activate(Some(&glib::variant::Variant::from_data_with_type(
                    "partitioning_done",
                    glib::VariantTy::STRING,
                )));
                main_carousel.scroll_to(&main_carousel.nth_page(6), true)
            }
        ),
    );
    //

    automatic_partitioning_page.set_child_widget(&content_box);

    partition_carousel.append(&automatic_partitioning_page);

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
        #[weak]
        advanced_filesystem_selection_frame,
        #[weak]
        advanced_home_seperation_selection_frame,
        #[weak]
        advanced_home_seperation_selection_checkbutton_subvol,
        #[weak]
        advanced_home_seperation_selection_checkbutton_partition,
        #[weak]
        advanced_home_seperation_selection_checkbutton_none,
        #[strong]
        devices_selection_button_row_dialog,
        move |_, _| {
            automatic_partitioning_page.set_page_title(t!("automatic_partitioning_page_title"));
            automatic_partitioning_page
                .set_page_subtitle(t!("automatic_partitioning_page_subtitle"));
            automatic_partitioning_page.set_back_tooltip_label(t!("back"));
            automatic_partitioning_page.set_next_tooltip_label(t!("next"));
            //
            devices_selection_button_row
                .set_title(&t!("devices_selection_button_row_title_no_drive_selected"));
            //
            devices_selection_button_row_dialog
                .set_title(&t!("devices_selection_button_row_dialog_auto_title"));
            devices_selection_button_row_dialog
                .set_body(&t!("devices_selection_button_row_dialog_auto_body"));
            devices_selection_button_row_dialog.set_response_label(
                "devices_selection_button_row_dialog_ok",
                &t!("devices_selection_button_row_dialog_ok_label"),
            );
            //
            partition_method_automatic_disk_nodisk_error_label.set_label(&t!(
                "partition_method_automatic_disk_nodisk_error_label_label"
            ));
            //
            partition_method_automatic_disk_small_error_label.set_label(&t!(
                "partition_method_automatic_disk_small_error_label_label"
            ));
            //
            partition_method_automatic_luks_empty_error_label.set_label(&t!(
                "partition_method_automatic_luks_empty_error_label_label"
            ));
            //
            partition_method_automatic_luks_missmatch_error_label.set_label(&t!(
                "partition_method_automatic_luks_missmatch_error_label_label"
            ));
            //
            partition_method_automatic_luks_checkbutton.set_label(Some(&t!(
                "partition_method_automatic_luks_checkbutton_label"
            )));
            //
            partition_method_automatic_luks_password_entry
                .set_title(&t!("partition_method_automatic_luks_password_entry_label"));
            //
            partition_method_automatic_luks_password_confirm_entry.set_title(&t!(
                "partition_method_automatic_luks_password_entry_label_confirm"
            ));
            //
            advanced_expander.set_label(Some(&t!("advanced_expander_label")));
            //
            advanced_filesystem_selection_frame
                .set_label(Some(&t!("advanced_filesystem_selection_frame_label")));
            //
            advanced_home_seperation_selection_frame
                .set_label(Some(&t!("advanced_home_seperation_selection_frame_label")));
            //
            advanced_home_seperation_selection_checkbutton_subvol.set_label(Some(&t!(
                "advanced_home_seperation_selection_checkbutton_subvol_label"
            )));
            //
            advanced_home_seperation_selection_checkbutton_partition.set_label(Some(&t!(
                "advanced_home_seperation_selection_checkbutton_partition_label"
            )));
            //
            advanced_home_seperation_selection_checkbutton_none.set_label(Some(&t!(
                "advanced_home_seperation_selection_checkbutton_none_label"
            )));
            //
        }
    ));
    //
}

fn disk_check(
    device_button: &gtk::CheckButton,
    devices_selection_button_row: &adw::ButtonRow,
    partition_method_automatic_disk_size_error_label: &gtk::Label,
    device_block_model: &str,
    device_block_name: &str,
    device_block_size_pretty: &str,
    device_block_size: f64,
) {
    if device_button.is_active() {
        devices_selection_button_row.set_title(
            &(device_block_model.to_owned()
                + ": "
                + device_block_name
                + " "
                + device_block_size_pretty),
        );
        devices_selection_button_row.remove_css_class("accent-blink");
        if device_block_size >= MINIMUM_ROOT_BYTE_SIZE {
            partition_method_automatic_disk_size_error_label.set_visible(false);
        } else {
            partition_method_automatic_disk_size_error_label.set_visible(true);
        }
    }
}

fn luks_check(
    partition_method_automatic_luks_checkbutton: &gtk::CheckButton,
    partition_method_automatic_luks_password_entry: &adw::PasswordEntryRow,
    partition_method_automatic_luks_password_confirm_entry: &adw::PasswordEntryRow,
    partition_method_automatic_luks_missmatch_error_label: &gtk::Label,
    partition_method_automatic_luks_empty_error_label: &gtk::Label,
) {
    if partition_method_automatic_luks_checkbutton.is_active() {
        if partition_method_automatic_luks_password_entry.text()
            != partition_method_automatic_luks_password_confirm_entry.text()
        {
            partition_method_automatic_luks_missmatch_error_label.set_visible(true)
        } else {
            partition_method_automatic_luks_missmatch_error_label.set_visible(false)
        }
        if partition_method_automatic_luks_password_entry
            .text()
            .to_string()
            .is_empty()
        {
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
            return false;
        }
    }
    true
}
