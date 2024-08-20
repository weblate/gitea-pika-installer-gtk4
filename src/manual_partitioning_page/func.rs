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

#[derive(Clone, Debug)]
struct PartitionRow {
    widget: adw::ActionRow,
    swap_fs_error: Rc<std::cell::RefCell<bool>>,
    hardcode_fs_error: Rc<std::cell::RefCell<bool>>,
    used: Rc<std::cell::RefCell<i8>>,
    never: Rc<std::cell::RefCell<bool>>,
}

pub fn create_efi_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
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

    row.set_id(0);

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
            .any(|e| {
                (part_name == &e.partition.part_name && part_name != &row.partition())
                    && (subvol_partition_array_refcell
                        .borrow()
                        .iter()
                        .any(|e| part_name == e))
            })
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
                        .sensitive(true)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(2)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == &e.partition.part_name && part_name != &row.partition())
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
            &null_checkbutton,
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
            partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
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
                (*used_partition_array_refcell.borrow_mut())
                    .retain(|x| &x.partition.part_name != &row.partition());
                partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
            }
        ),
    );
}

pub fn create_boot_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
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

    row.set_id(1);

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
            .any(|e| {
                (part_name == &e.partition.part_name && part_name != &row.partition())
                    && (subvol_partition_array_refcell
                        .borrow()
                        .iter()
                        .any(|e| part_name == e))
            })
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
                        .sensitive(true)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(2)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == &e.partition.part_name && part_name != &row.partition())
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
            &null_checkbutton,
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
            partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
            ));
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
                (*used_partition_array_refcell.borrow_mut())
                    .retain(|x| &x.partition.part_name != &row.partition());
                partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
            }
        ),
    );
}

pub fn create_root_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
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

    row.set_id(2);

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
            .any(|e| {
                (part_name == &e.partition.part_name && part_name != &row.partition())
                    && (subvol_partition_array_refcell
                        .borrow()
                        .iter()
                        .any(|e| part_name == e))
            })
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
                        .sensitive(true)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(2)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == &e.partition.part_name && part_name != &row.partition())
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
            &null_checkbutton,
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
            partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
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
                (*used_partition_array_refcell.borrow_mut())
                    .retain(|x| &x.partition.part_name != &row.partition());
                partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
            }
        ),
    );
}

pub fn create_mount_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    extra_mount_id_refcell: &Rc<RefCell<i32>>,
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

    row.set_id(*extra_mount_id_refcell.borrow());

    (*extra_mount_id_refcell.borrow_mut()) += 1;

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
            .any(|e| {
                (part_name == &e.partition.part_name && part_name != &row.partition())
                    && (subvol_partition_array_refcell
                        .borrow()
                        .iter()
                        .any(|e| part_name == e))
            })
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
                        .sensitive(true)
                        .build();
                    prow
                },
                swap_fs_error: Rc::new(RefCell::new(false)),
                hardcode_fs_error: Rc::new(RefCell::new(false)),
                used: Rc::new(RefCell::new(2)),
                never: Rc::new(RefCell::new(false)),
            }
        } else if used_partition_array_refcell
            .clone()
            .borrow()
            .iter()
            .any(|e| part_name == &e.partition.part_name && part_name != &row.partition())
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
            &null_checkbutton,
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
            partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
            ));
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
                (*used_partition_array_refcell.borrow_mut())
                    .retain(|x| &x.partition.part_name != &row.partition());
                partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
            }
        ),
    );
}

fn post_check_drive_mount(
    row: &DriveMountRow,
    partition_row_struct: &PartitionRow,
    null_checkbutton: &gtk::CheckButton,
    partition_button: &gtk::CheckButton,
    partition_changed_action: &gio::SimpleAction,
    partition: &Partition,
    used_partition_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
) {
    partition_row_struct.widget.add_prefix(partition_button);
    partition_button.connect_toggled(clone!(
        #[weak]
        row,
        #[strong]
        null_checkbutton,
        #[strong]
        partition_row_struct,
        #[weak]
        partition_button,
        #[strong]
        partition_changed_action,
        #[strong]
        used_partition_array_refcell,
        #[strong]
        partition,
        move |_| {
            let (check_delay_sender, check_delay_receiver) = async_channel::unbounded::<bool>();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                check_delay_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
            });

            let check_delay_main_context = glib::MainContext::default();
            check_delay_main_context.spawn_local(clone!(
                #[weak]
                row,
                #[strong]
                null_checkbutton,
                #[strong]
                partition_row_struct,
                #[weak]
                partition_button,
                #[strong]
                partition_changed_action,
                #[strong]
                used_partition_array_refcell,
                #[strong]
                partition,
                async move {
                    while let Ok(_state) = check_delay_receiver.recv().await {
                        if !null_checkbutton.is_active() {
                            if partition_button.is_active() == true {
                                let part_name = &partition.part_name;
                                row.set_partition(part_name.to_string());
                                (*used_partition_array_refcell.borrow_mut())
                                    .push(DriveMountRow::get_fstab_entry(&row));
                            } else {
                                (*used_partition_array_refcell.borrow_mut())
                                    .retain(|x| &x.partition.part_name != &row.partition());
                            }
                            partition_changed_action.activate(Some(
                                &glib::variant::Variant::from_data_with_type(row.id().to_string(), glib::VariantTy::STRING)
                            ));
                        }
                    }
                }
            ));
        }
    ));
    row.connect_mountpoint_notify(clone!(
        #[strong]
        partition_row_struct,
        #[strong]
        null_checkbutton,
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
                    null_checkbutton.set_active(true);
                    partition_row_struct.widget.set_sensitive(false);
                }
            } else {
                if *partition_row_struct.used.borrow() != 1
                    && *partition_row_struct.never.borrow() == false
                    && *partition_row_struct.hardcode_fs_error.borrow() == false
                {
                    (*partition_row_struct.swap_fs_error.borrow_mut()) = false;
                    partition_row_struct.widget.set_sensitive(true);
                }
            }
        }
    ));

    partition_changed_action.connect_activate(clone!(
        #[strong]
        partition_row_struct,
        #[strong]
        null_checkbutton,
        #[strong]
        row,
        #[strong]
        partition,
        #[strong]
        used_partition_array_refcell,
        #[strong]
        subvol_partition_array_refcell,
        move |_, varient| {
            let part_name = &partition.part_name;
            let used_partition_array = used_partition_array_refcell.borrow();
            let subvol_partition_array = subvol_partition_array_refcell.borrow();

            let action_id = String::from_utf8_lossy(varient.unwrap().data()).parse::<i32>().unwrap();

            if used_partition_array.iter().any(|e| {
                (part_name == &e.partition.part_name)
                    && (subvol_partition_array.iter().any(|e| part_name == e))
            }) {
                if *partition_row_struct.never.borrow() == false
                    && *partition_row_struct.swap_fs_error.borrow() == false
                    && *partition_row_struct.hardcode_fs_error.borrow() == false
                {
                    partition_row_struct.widget.set_sensitive(true);
                }
                (*partition_row_struct.used.borrow_mut()) = 2;
            } else if used_partition_array
                .iter()
                .any(|e| part_name == &e.partition.part_name)
            {
                if action_id != row.id() {
                    null_checkbutton.set_active(true);
                    partition_row_struct.widget.set_sensitive(false);
                }
                (*partition_row_struct.used.borrow_mut()) = 1;
            } else {
                if *partition_row_struct.never.borrow() == false
                    && *partition_row_struct.swap_fs_error.borrow() == false
                    && *partition_row_struct.hardcode_fs_error.borrow() == false
                {
                    partition_row_struct.widget.set_sensitive(true);
                }
                (*partition_row_struct.used.borrow_mut()) = 0;
            }
        }
    ));
}
