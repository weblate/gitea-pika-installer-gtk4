use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use crate::partitioning_page::{get_partitions, CrypttabEntry, FstabEntry, Partition};
use crate::drive_mount_row::{DriveMountRow};
use adw::{prelude::*};
use glib::{clone, closure_local, ffi::gboolean};
use std::{rc::Rc, cell::RefCell};

const MINIMUM_EFI_BYTE_SIZE: f64 = 500000000.0;
const MINIMUM_BOOT_BYTE_SIZE: f64 = 1000000000.0;
const MINIMUM_ROOT_BYTE_SIZE: f64 = 39000000000.0;

pub fn manual_partitioning_page(
    partition_carousel: &adw::Carousel,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
    language_changed_action: &gio::SimpleAction
) {
    let manual_partitioning_page = installer_stack_page::InstallerStackPage::new();
    manual_partitioning_page.set_page_icon("emblem-system-symbolic");
    manual_partitioning_page.set_back_visible(true);
    manual_partitioning_page.set_next_visible(true);
    manual_partitioning_page.set_back_sensitive(true);
    manual_partitioning_page.set_next_sensitive(false);

    let partition_array_refcell = Rc::new(RefCell::new(get_partitions()));
    let used_partition_array_refcell: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::default());
    let do_used_part_check_refcell = Rc::new(RefCell::new(true));

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

    let drive_mounts_viewport =
    gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&drive_mounts_adw_listbox)
        .build();

    drive_mounts_viewport.add_css_class("round-all-scroll");

    let drive_mount_add_button = gtk::Button::builder()
        .icon_name("list-add")
        .vexpand(true)
        .hexpand(true)
        .build();

    drive_mounts_adw_listbox.append(&drive_mount_add_button);
    content_box.append(&drive_mounts_viewport);


    drive_mount_add_button.connect_clicked(clone!(
        #[weak]
        drive_mounts_adw_listbox,
        #[strong]
        partition_array_refcell,
        #[strong]
        language_changed_action,
        #[strong]
        partition_changed_action,
        move |_|
            {
                create_mount_row(&drive_mounts_adw_listbox, &drive_rows_size_group, &partition_array_refcell.borrow(), &partition_changed_action, &language_changed_action, &used_partition_array_refcell, &do_used_part_check_refcell);
            }
        )    
    );

    //
    manual_partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            partition_carousel,
            move |_manual_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                    partition_carousel.scroll_to(&partition_carousel.nth_page(0), true)
            }
        )
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
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                *partition_method_type_refcell.borrow_mut() = String::from("manual");
                //partition_carousel.scroll_to(&partition_carousel.nth_page(5), true)
                dbg!(partition_method_type_refcell.borrow());
                //dbg!(partition_method_manual_fstab_entry_array_refcell.borrow());
                dbg!(partition_method_manual_luks_enabled_refcell.borrow());
                //dbg!(partition_method_manual_crypttab_entry_array_refcell.borrow());
            }
        )
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
                manual_partitioning_page.set_page_title(t!("manual_part_installer"));
                manual_partitioning_page.set_page_subtitle(t!("manual_part_info"));
                manual_partitioning_page.set_back_tooltip_label(t!("back"));
                manual_partitioning_page.set_next_tooltip_label(t!("next"));
            }
        )
    );
    //
}

fn create_mount_row(
    listbox: &gtk::ListBox,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array: &Vec<Partition>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<String>>>,
    do_used_part_check_refcell: &Rc<RefCell<bool>>,
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
        let partition_row: adw::ActionRow =
            if partition.need_mapper {
                let prow = adw::ActionRow::builder()
                    .activatable_widget(&partition_button)
                    .title(part_name)
                    .name(part_name)
                    .subtitle(t!("part_need_mapper").to_string() + " " + &pretty_bytes::converter::convert(partition.part_size))
                    .sensitive(false)
                    .build();
                prow
            }
            else if used_partition_array_refcell.clone().borrow().iter().any(|e| part_name == e && part_name != &row.partition()) {
                let prow = adw::ActionRow::builder()
                    .activatable_widget(&partition_button)
                    .title(part_name)
                    .name(part_name)
                    .subtitle(String::from(&partition.part_fs) + " " + &pretty_bytes::converter::convert(partition.part_size))
                    .sensitive(false)
                    .build();
                prow
            } else {
                let prow = adw::ActionRow::builder()
                    .activatable_widget(&partition_button)
                    .title(part_name)
                    .name(part_name)
                    .subtitle(String::from(&partition.part_fs) + " " + &pretty_bytes::converter::convert(partition.part_size))
                    .build();
                prow
            };
        partition_row.add_prefix(&partition_button);
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
            move |_|
                {
                    if partition_button.is_active() == true {
                        let part_name = &partition.part_name;
                        row.set_partition(part_name.to_string());
                        (*used_partition_array_refcell.borrow_mut()).push(part_name.to_string());
                    } else {
                        (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &partition.part_name);
                    }
                    partition_changed_action.activate(None);
                }
            )
        );
        partition_changed_action.connect_activate(clone!(
            #[strong]
            partition_row,
            #[strong]
            row,
            #[strong]
            partition,
            #[strong]
            used_partition_array_refcell,
            #[strong]
            do_used_part_check_refcell,
                move |_, _| {
                    if *do_used_part_check_refcell.borrow() == true {
                        let part_name = &partition.part_name;
                        let used_partition_array = used_partition_array_refcell.borrow();
                        if used_partition_array.iter().any(|e| part_name == e && part_name != &row.partition()) {
                            partition_row.set_sensitive(false);
                        } else {
                            partition_row.set_sensitive(true);
                        }
                    }
                }
            )
        );
        partition_scroll_child.append(&partition_row);
    }

    
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
            move |row: DriveMountRow| 
                {
                    listbox.remove(&row);
                    (*used_partition_array_refcell.borrow_mut()).retain(|x| x != &row.partition());
                    partition_changed_action.activate(None);
                }
        ),
    );
}