use crate::drive_mount_row::DriveMountRow;
use crate::installer_stack_page;
use crate::partitioning_page::{get_partitions, CrypttabEntry, FstabEntry, Partition};
use adw::gio;
use adw::prelude::*;
use glib::{clone, closure_local, ffi::gboolean};
use gtk::{glib, prelude::*};
use std::{cell::RefCell, rc::Rc};

const MINIMUM_EFI_BYTE_SIZE: f64 = 500000000.0;
const MINIMUM_BOOT_BYTE_SIZE: f64 = 1000000000.0;
const MINIMUM_ROOT_BYTE_SIZE: f64 = 39000000000.0;

#[derive(Clone)]
struct PartitionRow {
    widget: adw::ActionRow,
    swap_fs_error: Rc<std::cell::RefCell<bool>>,
    hardcode_fs_error: Rc<std::cell::RefCell<bool>>,
    used: Rc<std::cell::RefCell<i8>>,
    never: Rc<std::cell::RefCell<bool>>,
}

pub fn manual_partitioning_page(
    partition_carousel: &adw::Carousel,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
    language_changed_action: &gio::SimpleAction,
) {
    let manual_partitioning_page = installer_stack_page::InstallerStackPage::new();
    manual_partitioning_page.set_page_icon("emblem-system-symbolic");
    manual_partitioning_page.set_back_visible(true);
    manual_partitioning_page.set_next_visible(true);
    manual_partitioning_page.set_back_sensitive(true);
    manual_partitioning_page.set_next_sensitive(false);

    let partition_array_refcell = Rc::new(RefCell::new(get_partitions()));
    let used_partition_array_refcell: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::default());
    let subvol_partition_array_refcell: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::default());

    //

    let partition_changed_action = gio::SimpleAction::new("partition-changed", None);

    //

    let drive_rows_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    //

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let drive_mounts_adw_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    drive_mounts_adw_listbox.add_css_class("boxed-list");
    drive_mounts_adw_listbox.add_css_class("round-all-scroll");

    let drive_mounts_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&drive_mounts_adw_listbox)
        .build();

    drive_mounts_viewport.add_css_class("round-all-scroll");

    create_hardcoded_rows(
        &drive_mounts_adw_listbox,
        &drive_rows_size_group,
        &partition_array_refcell,
        &partition_changed_action,
        &language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
    );

    content_box.append(&drive_mounts_viewport);

    //
    manual_partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            partition_carousel,
            move |_manual_partitioning_page: installer_stack_page::InstallerStackPage| {
                partition_carousel.scroll_to(&partition_carousel.nth_page(0), true)
            }
        ),
    );

    manual_partitioning_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            partition_carousel,
            #[strong]
            partition_method_type_refcell,
            #[strong]
            partition_method_manual_fstab_entry_array_refcell,
            #[strong]
            partition_method_manual_luks_enabled_refcell,
            #[strong]
            partition_method_manual_crypttab_entry_array_refcell,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage| {
                *partition_method_type_refcell.borrow_mut() = String::from("manual");
                //partition_carousel.scroll_to(&partition_carousel.nth_page(5), true)
                dbg!(partition_method_type_refcell.borrow());
                //dbg!(partition_method_manual_fstab_entry_array_refcell.borrow());
                dbg!(partition_method_manual_luks_enabled_refcell.borrow());
                //dbg!(partition_method_manual_crypttab_entry_array_refcell.borrow());
            }
        ),
    );
    //

    //

    manual_partitioning_page.set_child_widget(&content_box);

    partition_carousel.append(&manual_partitioning_page);

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        manual_partitioning_page,
        move |_, _| {
            manual_partitioning_page.set_page_title(t!("manual_partitioning_page_title"));
            manual_partitioning_page.set_page_subtitle(t!("manual_partitioning_page_subtitle"));
            manual_partitioning_page.set_back_tooltip_label(t!("back"));
            manual_partitioning_page.set_next_tooltip_label(t!("next"));
        }
    ));
    //
}

fn create_efi_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    let partition_scroll_child = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let partitions_scroll = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&partition_scroll_child)
        .build();

    // Create row
    let row = DriveMountRow::new_with_scroll(&partitions_scroll);

    row.set_deletable(false);

    row.set_sizegroup(drive_rows_size_group);

    row.set_langaction(language_changed_action);

    row.set_mountpoint("/boot/efi");

    let null_checkbutton = gtk::CheckButton::builder().build();

    for partition in partition_array {
        let part_name = &partition.part_name.to_owned();
        let partition_button = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        partition_button.set_group(Some(&null_checkbutton));
        let partition_row_struct: PartitionRow = if partition.need_mapper {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            t!("partition_row_subtitle_needs_mapper").to_string()
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == e && part_name != &row.partition())
        {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(1)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if partition.part_fs != "vfat" {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(t!("partition_row_subtitle_efi_fs_bad"))
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(true)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if partition.part_size < MINIMUM_EFI_BYTE_SIZE {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(t!("partition_row_subtitle_efi_fs_small"))
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(true)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(true)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(false)),
            }
        };
        post_check_drive_mount(
            &row,
            &partition_row_struct,
            &partition_button,
            &partition_changed_action,
            &partition,
            &used_partition_array_refcell,
            &subvol_partition_array_refcell,
        );
        partition_scroll_child.append(&partition_row_struct.widget);
    }

    row.connect_mountopts_notify(clone!(
        #[strong]
        partition_changed_action,
        #[strong]
        subvol_partition_array_refcell,
        #[strong]
        row,
        move |_| {
            if row.mountopts().contains("subvol=") || row.mountopts().contains("subvolid") {
                (*subvol_partition_array_refcell.borrow_mut()).push(row.partition());
            } else {
                (*subvol_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
            }
            partition_changed_action.activate(None);
        }
    ));

    listbox.append(&row);

    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(
            #[weak]
            listbox,
            #[strong]
            row,
            #[strong]
            used_partition_array_refcell,
            #[strong]
            partition_changed_action,
            move |row: DriveMountRow| {
                listbox.remove(&row);
                (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
                partition_changed_action.activate(None);
            }
        ),
    );
}

fn create_boot_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    let partition_scroll_child = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let partitions_scroll = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&partition_scroll_child)
        .build();

    // Create row
    let row = DriveMountRow::new_with_scroll(&partitions_scroll);

    row.set_deletable(false);

    row.set_sizegroup(drive_rows_size_group);

    row.set_langaction(language_changed_action);

    row.set_mountpoint("/boot");

    let null_checkbutton = gtk::CheckButton::builder().build();

    for partition in partition_array {
        let part_name = &partition.part_name.to_owned();
        let partition_button = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        partition_button.set_group(Some(&null_checkbutton));
        let partition_row_struct: PartitionRow = if partition.need_mapper {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            t!("partition_row_subtitle_needs_mapper").to_string()
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == e && part_name != &row.partition())
        {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(1)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if partition.part_fs != "ext4" {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(t!("partition_row_subtitle_boot_fs_bad"))
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(true)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if partition.part_size < MINIMUM_BOOT_BYTE_SIZE {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(t!("partition_row_subtitle_boot_fs_small"))
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(true)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(true)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(false)),
            }
        };
        post_check_drive_mount(
            &row,
            &partition_row_struct,
            &partition_button,
            &partition_changed_action,
            &partition,
            &used_partition_array_refcell,
            &subvol_partition_array_refcell,
        );
        partition_scroll_child.append(&partition_row_struct.widget);
    }

    row.connect_mountopts_notify(clone!(
        #[strong]
        partition_changed_action,
        #[strong]
        subvol_partition_array_refcell,
        #[strong]
        row,
        move |_| {
            if row.mountopts().contains("subvol=") || row.mountopts().contains("subvolid") {
                (*subvol_partition_array_refcell.borrow_mut()).push(row.partition());
            } else {
                (*subvol_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
            }
            partition_changed_action.activate(None);
        }
    ));

    listbox.append(&row);

    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(
            #[weak]
            listbox,
            #[strong]
            row,
            #[strong]
            used_partition_array_refcell,
            #[strong]
            partition_changed_action,
            move |row: DriveMountRow| {
                listbox.remove(&row);
                (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
                partition_changed_action.activate(None);
            }
        ),
    );
}

fn create_root_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    let partition_scroll_child = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let partitions_scroll = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&partition_scroll_child)
        .build();

    // Create row
    let row = DriveMountRow::new_with_scroll(&partitions_scroll);

    row.set_deletable(false);

    row.set_sizegroup(drive_rows_size_group);

    row.set_langaction(language_changed_action);

    row.set_mountpoint("/");

    let null_checkbutton = gtk::CheckButton::builder().build();

    for partition in partition_array {
        let part_name = &partition.part_name.to_owned();
        let partition_button = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        partition_button.set_group(Some(&null_checkbutton));
        let partition_row_struct: PartitionRow = if partition.need_mapper {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            t!("partition_row_subtitle_needs_mapper").to_string()
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == e && part_name != &row.partition())
        {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(1)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if partition.part_fs == "vfat"
            || partition.part_fs == "ntfs"
            || partition.part_fs == "swap"
            || partition.part_fs == "exfat"
            || partition.part_fs == "BitLocker"
        {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(t!("partition_row_subtitle_root_fs_bad"))
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(true)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if partition.part_size < MINIMUM_ROOT_BYTE_SIZE {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(t!("partition_row_subtitle_root_fs_small"))
                        .sensitive(false)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(true)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(true)
                        .build();
                    prow
                },
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                swap_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(false)),
            }
        };
        post_check_drive_mount(
            &row,
            &partition_row_struct,
            &partition_button,
            &partition_changed_action,
            &partition,
            &used_partition_array_refcell,
            &subvol_partition_array_refcell,
        );
        partition_scroll_child.append(&partition_row_struct.widget);
    }

    row.connect_mountopts_notify(clone!(
        #[strong]
        partition_changed_action,
        #[strong]
        subvol_partition_array_refcell,
        #[strong]
        row,
        move |_| {
            if row.mountopts().contains("subvol=") || row.mountopts().contains("subvolid") {
                (*subvol_partition_array_refcell.borrow_mut()).push(row.partition());
            } else {
                (*subvol_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
            }
            partition_changed_action.activate(None);
        }
    ));

    listbox.append(&row);

    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(
            #[weak]
            listbox,
            #[strong]
            row,
            #[strong]
            used_partition_array_refcell,
            #[strong]
            partition_changed_action,
            move |row: DriveMountRow| {
                listbox.remove(&row);
                (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
                partition_changed_action.activate(None);
            }
        ),
    );
}

fn create_mount_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    let partition_scroll_child = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let partitions_scroll = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&partition_scroll_child)
        .build();

    // Create row
    let row = DriveMountRow::new_with_scroll(&partitions_scroll);

    row.set_deletable(true);

    row.set_sizegroup(drive_rows_size_group);

    row.set_langaction(language_changed_action);

    let null_checkbutton = gtk::CheckButton::builder().build();

    for partition in partition_array {
        let part_name = &partition.part_name.to_owned();
        let partition_button = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        partition_button.set_group(Some(&null_checkbutton));
        let partition_row_struct: PartitionRow = if partition.need_mapper {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            t!("partition_row_subtitle_needs_mapper").to_string()
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                swap_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(true)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == e && part_name != &row.partition())
        {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(false)
                        .build();
                    prow
                },
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                swap_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(1)),
                never: Rc::new(RefCell::new(false)),
            }
        } else {
            PartitionRow {
                widget: {
                    let prow = adw::ActionRow::builder()
                        .activatable_widget(&partition_button)
                        .title(part_name)
                        .subtitle(
                            String::from(&partition.part_fs)
                                + " "
                                + &pretty_bytes::converter::convert(partition.part_size),
                        )
                        .sensitive(true)
                        .build();
                    prow
                },
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                swap_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(0)),
                never: Rc::new(RefCell::new(false)),
            }
        };
        post_check_drive_mount(
            &row,
            &partition_row_struct,
            &partition_button,
            &partition_changed_action,
            &partition,
            &used_partition_array_refcell,
            &subvol_partition_array_refcell,
        );
        partition_scroll_child.append(&partition_row_struct.widget);
    }

    row.connect_mountopts_notify(clone!(
        #[strong]
        partition_changed_action,
        #[strong]
        subvol_partition_array_refcell,
        #[strong]
        row,
        move |_| {
            if row.mountopts().contains("subvol=") || row.mountopts().contains("subvolid") {
                (*subvol_partition_array_refcell.borrow_mut()).push(row.partition());
            } else {
                (*subvol_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
            }
            partition_changed_action.activate(None);
        }
    ));

    listbox.append(&row);

    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(
            #[weak]
            listbox,
            #[strong]
            row,
            #[strong]
            used_partition_array_refcell,
            #[strong]
            partition_changed_action,
            move |row: DriveMountRow| {
                listbox.remove(&row);
                (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
                partition_changed_action.activate(None);
            }
        ),
    );
}

fn post_check_drive_mount(
    row: &DriveMountRow,
    partition_row_struct: &PartitionRow,
    partition_button: &gtk::CheckButton,
    partition_changed_action: &gio::SimpleAction,
    partition: &Partition,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    partition_row_struct.widget.add_prefix(partition_button);
    partition_button.connect_toggled(clone!(
        #[weak]
        row,
        #[weak]
        partition_button,
        #[strong]
        partition_changed_action,
        #[strong]
        used_partition_array_refcell,
        #[strong]
        partition,
        move |_| {
            if partition_button.is_active() == true {
                let part_name = &partition.part_name;
                row.set_partition(part_name.to_string());
                (*used_partition_array_refcell.borrow_mut()).push(part_name.to_string());
            } else {
                (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &partition.part_name);
            }
            partition_changed_action.activate(None);
        }
    ));
    row.connect_mountpoint_notify(clone!(
        #[strong]
        partition_row_struct,
        #[strong]
        partition,
        #[strong]
        row,
        move |_| {
            if row.mountpoint() == "[SWAP]" {
                if partition.part_fs == "linux-swap" {
                    (*partition_row_struct.swap_fs_error.borrow_mut()) = false;
                    if *partition_row_struct.never.borrow() == false
                        && *partition_row_struct.swap_fs_error.borrow() == false
                        && *partition_row_struct.hardcode_fs_error.borrow() == false
                    {
                        partition_row_struct.widget.set_sensitive(true);
                    }
                } else {
                    (*partition_row_struct.swap_fs_error.borrow_mut()) = true;
                    partition_row_struct.widget.set_sensitive(false);
                }
            } else if *partition_row_struct.used.borrow() != 1
                && *partition_row_struct.never.borrow() == false
                && *partition_row_struct.hardcode_fs_error.borrow() == false
            {
                (*partition_row_struct.swap_fs_error.borrow_mut()) = false;
                partition_row_struct.widget.set_sensitive(true);
            }
        }
    ));

    partition_changed_action.connect_activate(clone!(
        #[strong]
        partition_row_struct,
        #[strong]
        row,
        #[strong]
        partition,
        #[strong]
        used_partition_array_refcell,
        #[strong]
        subvol_partition_array_refcell,
        move |_, _| {
            let part_name = &partition.part_name;
            let used_partition_array = used_partition_array_refcell.borrow();
            let subvol_partition_array = subvol_partition_array_refcell.borrow();
            if used_partition_array
                .iter()
                .any(|e| part_name == e && part_name != &row.partition())
            {
                if subvol_partition_array
                    .iter()
                    .any(|e| part_name == e && part_name != &row.partition())
                {
                    if *partition_row_struct.never.borrow() == false
                        && *partition_row_struct.swap_fs_error.borrow() == false
                        && *partition_row_struct.hardcode_fs_error.borrow() == false
                    {
                        partition_row_struct.widget.set_sensitive(true);
                    }
                    (*partition_row_struct.used.borrow_mut()) = 2;
                } else {
                    partition_row_struct.widget.set_sensitive(false);
                    (*partition_row_struct.used.borrow_mut()) = 1;
                }
            } else if *partition_row_struct.never.borrow() == false
                && *partition_row_struct.swap_fs_error.borrow() == false
                && *partition_row_struct.hardcode_fs_error.borrow() == false
            {
                partition_row_struct.widget.set_sensitive(true);
                (*partition_row_struct.used.borrow_mut()) = 0;
            }
        }
    ));
}

fn create_hardcoded_rows(
    drive_mounts_adw_listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array_refcell: &Rc<RefCell<Vec<Partition>>>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    let drive_mount_add_button_icon = gtk::Image::builder()
        .icon_name("list-add")
        .halign(gtk::Align::Start)
        .build();

    let drive_mount_add_button_label = gtk::Label::builder()
        .label(t!("drive_mount_add_button_label"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build();

    let drive_mount_add_button_child_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    drive_mount_add_button_child_box.append(&drive_mount_add_button_icon);
    drive_mount_add_button_child_box.append(&drive_mount_add_button_label);

    let drive_mount_add_button = gtk::Button::builder()
        .child(&drive_mount_add_button_child_box)
        .vexpand(true)
        .hexpand(true)
        .build();

    create_efi_row(
        &drive_mounts_adw_listbox,
        &drive_rows_size_group,
        &partition_array_refcell.borrow(),
        &partition_changed_action,
        &language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
    );
    create_boot_row(
        &drive_mounts_adw_listbox,
        &drive_rows_size_group,
        &partition_array_refcell.borrow(),
        &partition_changed_action,
        &language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
    );
    create_root_row(
        &drive_mounts_adw_listbox,
        &drive_rows_size_group,
        &partition_array_refcell.borrow(),
        &partition_changed_action,
        &language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
    );

    drive_mounts_adw_listbox.append(&drive_mount_add_button);

    drive_mount_add_button.connect_clicked(clone!(
        #[weak]
        drive_mounts_adw_listbox,
        #[strong]
        drive_rows_size_group,
        #[strong]
        partition_array_refcell,
        #[strong]
        partition_changed_action,
        #[strong]
        language_changed_action,
        #[strong]
        used_partition_array_refcell,
        #[strong]
        subvol_partition_array_refcell,
        move |_| {
            create_mount_row(
                &drive_mounts_adw_listbox,
                &drive_rows_size_group,
                &partition_array_refcell.borrow(),
                &partition_changed_action,
                &language_changed_action,
                &used_partition_array_refcell,
                &subvol_partition_array_refcell,
            );
        }
    ));
}
