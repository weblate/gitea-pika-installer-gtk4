use crate::drive_mount_row::DriveMountRow;
use crate::installer_stack_page;
use crate::partitioning_page::{get_partitions, CrypttabEntry, FstabEntry, Partition};
use adw::gio;
use adw::prelude::*;
use glib::{clone, closure_local, ffi::gboolean};
use gtk::{glib, prelude::*, Orientation};
use std::{cell::RefCell, rc::Rc};

mod func;

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

    let open_disk_utility_button = gtk::Button::builder()
        .label(t!("open_disk_utility_button_label"))
        .margin_top(10)
        .margin_end(5)
        .halign(gtk::Align::Start)
        .build();

    let filesystem_table_refresh_button = gtk::Button::builder()
        .label(t!("filesystem_table_refresh_button_label"))
        .margin_top(10)
        .margin_end(5)
        .halign(gtk::Align::Start)
        .build();
    filesystem_table_refresh_button.add_css_class("destructive-action");

    let filesystem_table_validate_button = gtk::Button::builder()
        .label(t!("filesystem_table_validate_button_label"))
        .margin_top(10)
        .hexpand(true)
        .halign(gtk::Align::End)
        .build();
    filesystem_table_validate_button.add_css_class("suggested-action");

    let utility_buttons_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .build();

    utility_buttons_box.append(&open_disk_utility_button);
    utility_buttons_box.append(&filesystem_table_refresh_button);
    utility_buttons_box.append(&filesystem_table_validate_button);

    open_disk_utility_button.connect_clicked(clone!(
        #[weak]
        filesystem_table_refresh_button,
        move |_|
            {
                let command = std::process::Command::new("blivet-gui").status();
                if command.unwrap().success() {
                    filesystem_table_refresh_button.emit_by_name("clicked", &[])
                }
            }
        )
    );

    filesystem_table_refresh_button.connect_clicked(clone!(
        #[weak]
        drive_mounts_adw_listbox,
        #[weak]
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
        #[strong]
        partition_method_manual_fstab_entry_array_refcell,
        #[strong]
        partition_method_manual_luks_enabled_refcell,
        #[strong]
        partition_method_manual_crypttab_entry_array_refcell,
        move |_|
            {
                while let Some(row) = drive_mounts_adw_listbox.last_child() {
                    drive_mounts_adw_listbox.remove(&row);
                }

                (*partition_method_manual_fstab_entry_array_refcell.borrow_mut()) = Vec::new();
                (*partition_method_manual_luks_enabled_refcell.borrow_mut()) = false;
                (*partition_method_manual_crypttab_entry_array_refcell.borrow_mut()) = Vec::new();
                (*used_partition_array_refcell.borrow_mut()) = Vec::new();
                (*subvol_partition_array_refcell.borrow_mut()) = Vec::new();
                create_hardcoded_rows(
                    &drive_mounts_adw_listbox,
                    &drive_rows_size_group,
                    &partition_array_refcell,
                    &partition_changed_action,
                    &language_changed_action,
                    &used_partition_array_refcell,
                    &subvol_partition_array_refcell,
                );
            }
        )
    );
    
    content_box.append(&drive_mounts_viewport);
    content_box.append(&utility_buttons_box);

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

    func::create_efi_row(
        &drive_mounts_adw_listbox,
        &drive_rows_size_group,
        &partition_array_refcell.borrow(),
        &partition_changed_action,
        &language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
    );
    func::create_boot_row(
        &drive_mounts_adw_listbox,
        &drive_rows_size_group,
        &partition_array_refcell.borrow(),
        &partition_changed_action,
        &language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
    );
    func::create_root_row(
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
            func::create_mount_row(
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
