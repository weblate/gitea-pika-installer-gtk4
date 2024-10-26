use crate::{
    build_ui::{CrypttabEntry, FstabEntry, Partition, SubvolDeclaration},
    drive_mount_row::DriveMountRow,
    installer_stack_page,
    partitioning_page::{get_luks_uuid, get_partitions, test_luks_passwd},
};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib, Orientation};
use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::{atomic::AtomicBool, Arc},
};

mod func;

pub fn manual_partitioning_page(
    main_carousel: &adw::Carousel,
    partition_carousel: &adw::Carousel,
    window: adw::ApplicationWindow,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
    language_changed_action: &gio::SimpleAction,
    page_done_action: &gio::SimpleAction,
) {
    let manual_partitioning_page = installer_stack_page::InstallerStackPage::new();
    manual_partitioning_page.set_page_icon("emblem-system-symbolic");
    manual_partitioning_page.set_back_visible(true);
    manual_partitioning_page.set_next_visible(true);
    manual_partitioning_page.set_back_sensitive(true);
    manual_partitioning_page.set_next_sensitive(false);

    let partition_array_refcell = Rc::new(RefCell::new(get_partitions()));
    let used_partition_array_refcell: Rc<RefCell<Vec<FstabEntry>>> = Rc::new(RefCell::default());
    let subvol_partition_array_refcell: Rc<RefCell<Vec<SubvolDeclaration>>> =
        Rc::new(RefCell::default());
    let extra_mount_id_refcell: Rc<RefCell<i32>> = Rc::new(RefCell::new(3));

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
    drive_mounts_adw_listbox.add_css_class("no-round-borders");

    let drive_mounts_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .overflow(gtk::Overflow::Hidden)
        .child(&drive_mounts_adw_listbox)
        .build();

    drive_mounts_viewport.add_css_class("round-all-scroll-no-padding");

    create_hardcoded_rows(
        &drive_mounts_adw_listbox,
        window.clone(),
        &drive_rows_size_group,
        &partition_array_refcell,
        &partition_changed_action,
        language_changed_action,
        &used_partition_array_refcell,
        &subvol_partition_array_refcell,
        &extra_mount_id_refcell,
    );

    let open_disk_utility_button = gtk::Button::builder()
        .margin_top(10)
        .margin_end(5)
        .halign(gtk::Align::Start)
        .build();

    let filesystem_table_refresh_button = gtk::Button::builder()
        .margin_top(10)
        .margin_end(5)
        .halign(gtk::Align::Start)
        .build();
    filesystem_table_refresh_button.add_css_class("destructive-action");

    let filesystem_table_validate_button = gtk::Button::builder()
        .margin_top(10)
        .hexpand(true)
        .halign(gtk::Align::End)
        .build();
    filesystem_table_validate_button.add_css_class("suggested-action");

    let utility_buttons_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .build();

    let partition_method_manual_mountpoint_empty_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_manual_mountpoint_empty_error_label.add_css_class("small_error_text");

    let partition_method_manual_mountpoint_invalid_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_manual_mountpoint_invalid_error_label.add_css_class("small_error_text");

    let partition_method_manual_partition_empty_error_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_manual_partition_empty_error_label.add_css_class("small_error_text");

    let partition_method_manual_mountpoint_duplicate_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_manual_mountpoint_duplicate_label.add_css_class("small_error_text");

    let partition_method_manual_valid_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .visible(false)
        .build();
    partition_method_manual_valid_label.add_css_class("small_valid_text");

    utility_buttons_box.append(&open_disk_utility_button);
    utility_buttons_box.append(&filesystem_table_refresh_button);
    utility_buttons_box.append(&filesystem_table_validate_button);

    open_disk_utility_button.connect_clicked(clone!(
        #[weak]
        filesystem_table_refresh_button,
        move |_| {
            let command = std::process::Command::new("blivet-gui").status();
            if command.unwrap().success() {
                filesystem_table_refresh_button.emit_by_name("clicked", &[])
            }
        }
    ));

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
        partition_array_refcell,
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
        #[strong]
        extra_mount_id_refcell,
        #[weak]
        partition_method_manual_mountpoint_empty_error_label,
        #[weak]
        partition_method_manual_mountpoint_invalid_error_label,
        #[weak]
        partition_method_manual_partition_empty_error_label,
        #[weak]
        partition_method_manual_mountpoint_duplicate_label,
        #[strong]
        partition_method_manual_valid_label,
        #[strong]
        manual_partitioning_page,
        #[strong]
        window,
        move |_| {
            while let Some(row) = drive_mounts_adw_listbox.last_child() {
                drive_mounts_adw_listbox.remove(&row);
            }

            (*partition_method_manual_fstab_entry_array_refcell.borrow_mut()) = Vec::new();
            (*partition_method_manual_luks_enabled_refcell.borrow_mut()) = false;
            (*partition_method_manual_crypttab_entry_array_refcell.borrow_mut()) = Vec::new();
            (*partition_array_refcell.borrow_mut()) = get_partitions();
            (*used_partition_array_refcell.borrow_mut()) = Vec::new();
            (*subvol_partition_array_refcell.borrow_mut()) = Vec::new();
            (*extra_mount_id_refcell.borrow_mut()) = 3;
            partition_method_manual_mountpoint_empty_error_label.set_visible(false);
            partition_method_manual_mountpoint_invalid_error_label.set_visible(false);
            partition_method_manual_partition_empty_error_label.set_visible(false);
            partition_method_manual_mountpoint_duplicate_label.set_visible(false);
            partition_method_manual_valid_label.set_visible(false);
            manual_partitioning_page.set_next_sensitive(false);
            create_hardcoded_rows(
                &drive_mounts_adw_listbox,
                window.clone(),
                &drive_rows_size_group,
                &partition_array_refcell,
                &partition_changed_action,
                &language_changed_action,
                &used_partition_array_refcell,
                &subvol_partition_array_refcell,
                &extra_mount_id_refcell,
            );
        }
    ));

    filesystem_table_validate_button.connect_clicked(clone!(
        #[weak]
        drive_mounts_adw_listbox,
        #[strong]
        window,
        #[strong]
        partition_method_manual_fstab_entry_array_refcell,
        #[strong]
        manual_partitioning_page,
        #[strong]
        partition_method_manual_luks_enabled_refcell,
        #[strong]
        partition_method_manual_crypttab_entry_array_refcell,
        #[strong]
        subvol_partition_array_refcell,
        #[weak]
        partition_method_manual_mountpoint_empty_error_label,
        #[weak]
        partition_method_manual_mountpoint_invalid_error_label,
        #[weak]
        partition_method_manual_partition_empty_error_label,
        #[weak]
        partition_method_manual_mountpoint_duplicate_label,
        #[strong]
        partition_method_manual_valid_label,
        move |_| {
            let errored = Arc::new(AtomicBool::new(false));

            (*partition_method_manual_fstab_entry_array_refcell.borrow_mut()) = Vec::new();
            (*partition_method_manual_luks_enabled_refcell.borrow_mut()) = false;
            (*partition_method_manual_crypttab_entry_array_refcell.borrow_mut()) = Vec::new();
            let mut seen_mountpoints = HashSet::new();
            let mut seen_partitions = HashSet::new();
            let seen_crypts: Rc<RefCell<HashSet<String>>> = Rc::new(RefCell::new(HashSet::new()));

            partition_method_manual_mountpoint_empty_error_label.set_visible(false);
            partition_method_manual_mountpoint_invalid_error_label.set_visible(false);
            partition_method_manual_partition_empty_error_label.set_visible(false);
            partition_method_manual_mountpoint_duplicate_label.set_visible(false);
            partition_method_manual_valid_label.set_visible(false);
            manual_partitioning_page.set_next_sensitive(false);

            for fs_entry in generate_filesystem_table_array(&drive_mounts_adw_listbox) {
                let fs_entry_clone0 = fs_entry.clone();
                if subvol_partition_array_refcell.borrow().is_empty()
                    && !seen_partitions.insert(fs_entry.clone().partition.part_name)
                {
                    (errored.store(true, std::sync::atomic::Ordering::Relaxed));
                }
                if fs_entry.mountpoint == "[SWAP]" {
                    if fs_entry.partition.part_fs == "linux-swap"
                        || fs_entry.partition.part_fs == "swap"
                    {
                    } else {
                        (errored.store(true, std::sync::atomic::Ordering::Relaxed));
                    }
                }
                if fs_entry.mountpoint.is_empty() {
                    (errored.store(true, std::sync::atomic::Ordering::Relaxed));
                    partition_method_manual_mountpoint_empty_error_label.set_visible(true);
                }
                if fs_entry.mountpoint == "[SWAP]"
                    || fs_entry.mountpoint.starts_with("/")
                        && !fs_entry.mountpoint.starts_with("/dev")
                {
                } else {
                    (errored.store(true, std::sync::atomic::Ordering::Relaxed));
                    partition_method_manual_mountpoint_invalid_error_label.set_visible(true);
                }
                if fs_entry.partition.part_name.is_empty() {
                    (errored.store(true, std::sync::atomic::Ordering::Relaxed));
                    partition_method_manual_partition_empty_error_label.set_visible(true);
                }
                if !seen_mountpoints.insert(fs_entry.clone().mountpoint) {
                    (errored.store(true, std::sync::atomic::Ordering::Relaxed));
                    partition_method_manual_mountpoint_duplicate_label.set_visible(true);
                }
                //
                (*partition_method_manual_fstab_entry_array_refcell.borrow_mut()).push(fs_entry);
                //
                let (check_delay_sender, check_delay_receiver) = async_channel::unbounded();
                let errored_clone0 = Arc::clone(&errored);
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    check_delay_sender
                        .send_blocking(errored_clone0)
                        .expect("The channel needs to be open.");
                });

                let check_delay_main_context = glib::MainContext::default();
                check_delay_main_context.spawn_local(clone!(
                    #[strong]
                    window,
                    #[strong]
                    partition_method_manual_luks_enabled_refcell,
                    #[strong]
                    partition_method_manual_crypttab_entry_array_refcell,
                    #[strong]
                    manual_partitioning_page,
                    #[strong]
                    fs_entry_clone0,
                    #[strong]
                    seen_crypts,
                    #[strong]
                    partition_method_manual_valid_label,
                    async move {
                        while let Ok(state) = check_delay_receiver.recv().await {
                            if !state.load(std::sync::atomic::Ordering::Relaxed) {
                                partition_method_manual_valid_label.set_visible(true);
                                set_crypttab_entries(
                                    &fs_entry_clone0,
                                    &seen_crypts,
                                    window.clone(),
                                    &partition_method_manual_crypttab_entry_array_refcell,
                                    &partition_method_manual_luks_enabled_refcell,
                                );
                                manual_partitioning_page.set_next_sensitive(true);
                            }
                        }
                    }
                ));
            }

            (*partition_method_manual_fstab_entry_array_refcell.borrow_mut())
                .sort_by_key(|p| p.mountpoint.clone())
        }
    ));

    content_box.append(&drive_mounts_viewport);
    content_box.append(&utility_buttons_box);
    content_box.append(&partition_method_manual_mountpoint_empty_error_label);
    content_box.append(&partition_method_manual_mountpoint_invalid_error_label);
    content_box.append(&partition_method_manual_partition_empty_error_label);
    content_box.append(&partition_method_manual_mountpoint_duplicate_label);
    content_box.append(&partition_method_manual_valid_label);

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
            main_carousel,
            #[strong]
            partition_method_type_refcell,
            #[strong]
            page_done_action,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage| {
                *partition_method_type_refcell.borrow_mut() = String::from("manual");
                page_done_action.activate(Some(&glib::variant::Variant::from_data_with_type(
                    "partitioning_done",
                    glib::VariantTy::STRING,
                )));
                main_carousel.scroll_to(&main_carousel.nth_page(6), true)
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
        #[weak]
        partition_method_manual_mountpoint_empty_error_label,
        #[weak]
        partition_method_manual_mountpoint_invalid_error_label,
        #[weak]
        partition_method_manual_partition_empty_error_label,
        #[weak]
        partition_method_manual_mountpoint_duplicate_label,
        #[weak]
        partition_method_manual_valid_label,
        #[weak]
        open_disk_utility_button,
        #[weak]
        filesystem_table_refresh_button,
        #[weak]
        filesystem_table_validate_button,
        move |_, _| {
            manual_partitioning_page.set_page_title(t!("manual_partitioning_page_title"));
            manual_partitioning_page.set_page_subtitle(t!("manual_partitioning_page_subtitle"));
            manual_partitioning_page.set_back_tooltip_label(t!("back"));
            manual_partitioning_page.set_next_tooltip_label(t!("next"));
            //
            partition_method_manual_mountpoint_empty_error_label.set_label(&t!(
                "partition_method_manual_mountpoint_empty_error_label_label"
            ));
            partition_method_manual_mountpoint_invalid_error_label.set_label(&t!(
                "partition_method_manual_mountpoint_invalid_error_label_label"
            ));
            partition_method_manual_partition_empty_error_label.set_label(&t!(
                "partition_method_manual_partition_empty_error_label_label"
            ));
            partition_method_manual_mountpoint_duplicate_label.set_label(&t!(
                "partition_method_manual_mountpoint_duplicate_label_label"
            ));
            partition_method_manual_valid_label
                .set_label(&t!("partition_method_manual_valid_label_label"));
            //
            open_disk_utility_button.set_label(&t!("open_disk_utility_button_label"));
            filesystem_table_refresh_button.set_label(&t!("filesystem_table_refresh_button_label"));
            filesystem_table_validate_button
                .set_label(&t!("filesystem_table_validate_button_label"))
        }
    ));
    //
}

fn create_hardcoded_rows(
    drive_mounts_adw_listbox: &gtk::ListBox,
    window: adw::ApplicationWindow,
    drive_rows_size_group: &gtk::SizeGroup,
    partition_array_refcell: &Rc<RefCell<Vec<Partition>>>,
    partition_changed_action: &gio::SimpleAction,
    language_changed_action: &gio::SimpleAction,
    used_partition_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    subvol_partition_array_refcell: &Rc<RefCell<Vec<SubvolDeclaration>>>,
    extra_mount_id_refcell: &Rc<RefCell<i32>>,
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
        drive_mounts_adw_listbox,
        window.clone(),
        drive_rows_size_group,
        &partition_array_refcell.borrow(),
        partition_changed_action,
        language_changed_action,
        used_partition_array_refcell,
        subvol_partition_array_refcell,
    );
    func::create_boot_row(
        drive_mounts_adw_listbox,
        window.clone(),
        drive_rows_size_group,
        &partition_array_refcell.borrow(),
        partition_changed_action,
        language_changed_action,
        used_partition_array_refcell,
        subvol_partition_array_refcell,
    );
    func::create_root_row(
        drive_mounts_adw_listbox,
        window.clone(),
        drive_rows_size_group,
        &partition_array_refcell.borrow(),
        partition_changed_action,
        language_changed_action,
        used_partition_array_refcell,
        subvol_partition_array_refcell,
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
        #[strong]
        extra_mount_id_refcell,
        move |_| {
            func::create_mount_row(
                &drive_mounts_adw_listbox,
                window.clone(),
                &drive_rows_size_group,
                &partition_array_refcell.borrow(),
                &partition_changed_action,
                &language_changed_action,
                &used_partition_array_refcell,
                &subvol_partition_array_refcell,
                &extra_mount_id_refcell,
            );
        }
    ));
}

fn generate_filesystem_table_array(drive_mounts_adw_listbox: &gtk::ListBox) -> Vec<FstabEntry> {
    let mut fstab_array: Vec<FstabEntry> = Vec::new();
    let mut widget_counter = drive_mounts_adw_listbox.first_child();
    while let Some(ref child) = widget_counter {
        if let Ok(t) = child.clone().downcast::<DriveMountRow>() {
            fstab_array.push(DriveMountRow::get_fstab_entry(&t));
        }
        widget_counter = child.next_sibling();
    }
    fstab_array
}

fn set_crypttab_entries(
    fs_entry: &FstabEntry,
    seen_crypts: &Rc<RefCell<HashSet<String>>>,
    window: adw::ApplicationWindow,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
) {
    if fs_entry.partition.has_encryption
        && (*seen_crypts.borrow_mut()).insert(fs_entry.clone().partition.part_name)
    {
        let fs_entry = fs_entry.clone();
        let (luks_manual_password_sender, luks_manual_password_receiver) =
            async_channel::unbounded::<bool>();
        let crypttab_password_listbox = gtk::ListBox::builder()
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();
        let crypttab_password_status_label = gtk::Label::builder().build();
        crypttab_password_listbox.add_css_class("boxed-list");
        let crypttab_password_entry_row = adw::PasswordEntryRow::builder()
            .title(
                strfmt::strfmt(
                    &t!("crypttab_password_entry_row_title"),
                    &std::collections::HashMap::from([(
                        "LUKS_NAME".to_string(),
                        fs_entry.clone().partition.part_name,
                    )]),
                )
                .unwrap(),
            )
            .build();
        crypttab_password_entry_row.set_show_apply_button(true);
        crypttab_password_listbox.append(&crypttab_password_entry_row);
        let crypttab_password_child_box = gtk::Box::new(Orientation::Vertical, 0);
        crypttab_password_child_box.append(&crypttab_password_listbox);
        crypttab_password_child_box.append(&crypttab_password_status_label);
        let crypttab_dialog = adw::AlertDialog::builder()
            //.transient_for(&window)
            //.hide_on_close(true)
            .extra_child(&crypttab_password_child_box)
            .width_request(400)
            .height_request(200)
            .heading(
                strfmt::strfmt(
                    &t!("crypttab_password_entry_row_title"),
                    &std::collections::HashMap::from([(
                        "LUKS_NAME".to_string(),
                        fs_entry.clone().partition.part_name,
                    )]),
                )
                .unwrap(),
            )
            .build();
        crypttab_dialog.add_response(
            "crypttab_dialog_boot",
            &t!("crypttab_dialog_response_crypttab_dialog_boot"),
        );
        crypttab_dialog.add_response(
            "crypttab_dialog_auto",
            &t!("crypttab_dialog_response_crypttab_dialog_auto"),
        );
        crypttab_dialog.set_response_enabled("crypttab_dialog_auto", false);
        crypttab_password_entry_row.connect_apply(clone!(
            #[weak]
            crypttab_password_entry_row,
            #[strong]
            fs_entry,
            #[weak]
            crypttab_password_status_label,
            move |_| {
                let luks_manual_password_sender = luks_manual_password_sender.clone();
                let luks_password = crypttab_password_entry_row.text().to_string();

                let fs_entry_clone1 = fs_entry.clone();

                crypttab_password_status_label
                    .set_label(&t!("crypttab_password_status_label_label_checking"));

                std::thread::spawn(move || {
                    luks_manual_password_sender
                        .send_blocking(test_luks_passwd(
                            &fs_entry_clone1.partition.part_name,
                            &luks_password,
                        ))
                        .expect("The channel needs to be open.");
                });
            }
        ));
        let luks_manual_password_main_context = glib::MainContext::default();
        // The main loop executes the asynchronous block
        luks_manual_password_main_context.spawn_local(clone!(
            #[weak]
            crypttab_dialog,
            #[weak]
            crypttab_password_status_label,
            async move {
                while let Ok(state) = luks_manual_password_receiver.recv().await {
                    crypttab_dialog.set_response_enabled("crypttab_dialog_auto", state);
                    if !state {
                        crypttab_password_status_label
                            .set_label(&t!("crypttab_password_status_label_label_wrong_password"))
                    } else {
                        crypttab_password_status_label.set_label("")
                    }
                }
            }
        ));

        let partition_method_manual_crypttab_entry_array_refcell_clone0 =
            partition_method_manual_crypttab_entry_array_refcell.clone();
        let partition_method_manual_luks_enabled_refcell_clone0 =
            partition_method_manual_luks_enabled_refcell.clone();

        crypttab_dialog.choose(&window, None::<&gio::Cancellable>, move |choice| {
            let part_name = fs_entry.partition.part_name;
            if choice == "crypttab_dialog_auto" {
                (*partition_method_manual_crypttab_entry_array_refcell_clone0.borrow_mut()).push(
                    CrypttabEntry {
                        partition: part_name.clone(),
                        map: part_name.replace("mapper/", ""),
                        uuid: get_luks_uuid(&part_name),
                        password: Some(crypttab_password_entry_row.text().to_string()),
                    },
                );
            } else {
                (*partition_method_manual_crypttab_entry_array_refcell_clone0.borrow_mut()).push(
                    CrypttabEntry {
                        partition: part_name.clone(),
                        map: part_name.replace("mapper/", ""),
                        uuid: get_luks_uuid(&part_name),
                        password: None,
                    },
                );
            }
            (*partition_method_manual_luks_enabled_refcell_clone0.borrow_mut()) = true;
        });
    }
}
