use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use crate::partitioning_page::{get_block_devices};
use adw::{prelude::*};
use glib::{clone, closure_local, ffi::gboolean};
use std::{rc::Rc, cell::RefCell};

const MINIMUM_EFI_BYTE_SIZE: f64 = 500000000.0;
const MINIMUM_BOOT_BYTE_SIZE: f64 = 1000000000.0;
const MINIMUM_ROOT_BYTE_SIZE: f64 = 39000000000.0;


pub fn manual_partitioning(
    partition_carousel: &adw::Carousel,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<String>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<String>>,
    language_changed_action: &gio::SimpleAction
) {
    let manual_partitioning_page = installer_stack_page::InstallerStackPage::new();
    manual_partitioning_page.set_page_icon("emblem-system-symbolic");
    manual_partitioning_page.set_back_visible(true);
    manual_partitioning_page.set_next_visible(true);
    manual_partitioning_page.set_back_sensitive(true);
    manual_partitioning_page.set_next_sensitive(false);

    //

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

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
                *partition_method_type_refcell.borrow_mut() = String::from("automatic");
                //partition_carousel.scroll_to(&partition_carousel.nth_page(5), true)
                dbg!(partition_method_type_refcell.borrow());
                dbg!(partition_method_manual_fstab_entry_array_refcell.borrow());
                dbg!(partition_method_manual_luks_enabled_refcell.borrow());
                dbg!(partition_method_manual_crypttab_entry_array_refcell.borrow());
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
                    manual_partitioning_page.set_page_title(t!("auto_part_installer"));
                    manual_partitioning_page.set_page_subtitle(t!("choose_drive_auto"));
                    manual_partitioning_page.set_back_tooltip_label(t!("back"));
                    manual_partitioning_page.set_next_tooltip_label(t!("next"));
                }
            )
        );
        //
}