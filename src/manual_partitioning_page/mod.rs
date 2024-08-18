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

    //

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let drive_mounts_adw_listbox = gtk::ListBox::builder().hexpand(true).vexpand(true).build();
    drive_mounts_adw_listbox.add_css_class("boxed-list");
    
    let drive_mounts_viewport = gtk::ScrolledWindow::builder()
        .margin_top(30)
        .margin_bottom(30)
        .margin_start(30)
        .margin_end(30)
        .hexpand(true)
        .vexpand(true)
        .child(&drive_mounts_adw_listbox)
        .build();

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
            partition_method_manual_fstab_entry_array_refcell,
            move |_|
                {
                    drive_mounts_adw_listbox.append(&create_mount_row(&drive_mounts_adw_listbox, &partition_array_refcell.borrow(), &partition_method_manual_fstab_entry_array_refcell))
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
    partition_array: &Vec<Partition>,
    fstab_refcell_array: &Rc<RefCell<Vec<FstabEntry>>>
) -> DriveMountRow {
    let partition_scroll_child = gtk::ListBox::builder().build();

    let partitions_scroll = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&partition_scroll_child)
        .build();

    // Create row
    let row = DriveMountRow::new_with_scroll(&partitions_scroll);

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
            listbox,
            #[weak]
            partition_button,
            #[strong]
            partition,
            move |_|
                {
                    if partition_button.is_active() == true {
                        let part_name = &partition.part_name;
                        row.set_partition(part_name.to_string());
                    }
                }
            )
        );
        partition_scroll_child.append(&partition_row);
    }

    let listbox_clone = listbox.clone();
    row.connect_closure(
        "row-deleted",
        false,
        closure_local!(
            #[strong]
            row,
            move |row: DriveMountRow| 
                {
                    listbox_clone.remove(&row);
                }
        ),
    );

    // Return row
    row
}