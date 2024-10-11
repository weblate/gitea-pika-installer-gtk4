use crate::{
    build_ui::{BlockDevice, CrypttabEntry, FstabEntry, PikaKeymap, PikaLocale},
    config::{LOG_FILE_PATH, MINIMUM_BOOT_BYTE_SIZE, MINIMUM_EFI_BYTE_SIZE},
    installer_stack_page,
};
use adw::prelude::*;
use duct::cmd;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{
    cell::RefCell,
    error::Error,
    fs,
    io::{prelude::*, BufReader, Write},
    path::Path,
    rc::Rc,
    thread,
};

mod script_gen;

fn run_install_process(
    sender: async_channel::Sender<String>,
    preq: &str,
    log_file_path: &str,
) -> Result<(), std::boxed::Box<dyn Error + Send + Sync>> {
    if !Path::new(&log_file_path).exists() {
        match fs::File::create(log_file_path) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Warning: {} file couldn't be created", log_file_path);
            }
        };
    }
    let (pipe_reader, pipe_writer) = os_pipe::pipe()?;
    let child = cmd!("sudo", "bash", "-c", preq)
        .stderr_to_stdout()
        .stdout_file(pipe_writer)
        .start()?;
    for line in BufReader::new(pipe_reader).lines() {
        let line = line?;
        let line_clone0 = line.clone();
        sender
            .send_blocking(line)
            .expect("Channel needs to be opened.");
        let mut log_file = fs::OpenOptions::new()
            .append(true)
            .open(log_file_path)
            .unwrap();

        if let Err(e) = writeln!(
            log_file,
            "[{}] {}",
            chrono::offset::Local::now().format("%Y/%m/%d_%H:%M"),
            line_clone0
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    child.wait()?;

    Ok(())
}

pub fn installation_summary_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction,
    page_done_action: &gio::SimpleAction,
    installation_log_loop_sender: async_channel::Sender<String>,
    installation_log_status_loop_sender: async_channel::Sender<bool>,
    language_selection_text_refcell: &Rc<RefCell<PikaLocale>>,
    keymap_selection_text_refcell: &Rc<RefCell<PikaKeymap>>,
    timezone_selection_text_refcell: &Rc<RefCell<String>>,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_refcell: &Rc<RefCell<BlockDevice>>,
    partition_method_automatic_target_fs_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_automatic_luks_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_ratio_refcell: &Rc<RefCell<f64>>,
    partition_method_automatic_seperation_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
) {
    let installation_summary_page = installer_stack_page::InstallerStackPage::new();
    installation_summary_page.set_page_icon("dialog-warning-symbolic");
    installation_summary_page.set_back_sensitive(true);
    installation_summary_page.set_next_sensitive(false);
    installation_summary_page.set_back_visible(true);
    installation_summary_page.set_next_visible(true);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let installation_summary_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    installation_summary_row_viewport_listbox.add_css_class("boxed-list");
    installation_summary_row_viewport_listbox.add_css_class("round-all-scroll");

    let installation_summary_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&installation_summary_row_viewport_listbox)
        .build();

    installation_summary_row_viewport.add_css_class("round-all-scroll");

    let install_confirm_button = gtk::Button::builder()
        .margin_top(15)
        .margin_bottom(15)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    install_confirm_button.add_css_class("destructive-action");
    install_confirm_button.add_css_class("rounded-all-25-with-padding");

    // / content_box appends
    //// add text and and entry to installation_summary page selections
    content_box.append(&installation_summary_row_viewport);
    content_box.append(&install_confirm_button);

    installation_summary_page.set_child_widget(&content_box);

    //

    install_confirm_button.connect_clicked(clone!(
        #[weak]
        main_carousel,
        #[strong]
        installation_log_loop_sender,
        #[strong]
        installation_log_status_loop_sender,
        #[strong]
        language_selection_text_refcell,
        #[strong]
        keymap_selection_text_refcell,
        #[strong]
        timezone_selection_text_refcell,
        #[strong]
        partition_method_type_refcell,
        #[strong]
        partition_method_automatic_luks_enabled_refcell,
        #[strong]
        partition_method_automatic_luks_refcell,
        #[strong]
        partition_method_automatic_target_fs_refcell,
        #[strong]
        partition_method_automatic_target_refcell,
        #[strong]
        partition_method_automatic_seperation_refcell,
        #[strong]
        partition_method_automatic_ratio_refcell,
        #[strong]
        partition_method_manual_fstab_entry_array_refcell,
        #[strong]
        partition_method_manual_luks_enabled_refcell,
        #[strong]
        partition_method_manual_crypttab_entry_array_refcell,
        move |_| {
            let cmd = script_gen::create_installation_script(
                &language_selection_text_refcell,
                &keymap_selection_text_refcell,
                &timezone_selection_text_refcell,
                &partition_method_type_refcell,
                &partition_method_automatic_target_refcell,
                &partition_method_automatic_target_fs_refcell,
                &partition_method_automatic_luks_enabled_refcell,
                &partition_method_automatic_luks_refcell,
                &partition_method_automatic_ratio_refcell,
                &partition_method_automatic_seperation_refcell,
                &partition_method_manual_fstab_entry_array_refcell,
                &partition_method_manual_luks_enabled_refcell,
                &partition_method_manual_crypttab_entry_array_refcell,
            );
            let installation_log_loop_sender_clone0 = installation_log_loop_sender.clone();
            let installation_log_status_loop_sender_clone0 =
                installation_log_status_loop_sender.clone();
            thread::spawn(move || {
                if Path::new(LOG_FILE_PATH).exists() {
                    fs::remove_file(LOG_FILE_PATH).expect("bad perms on log file");
                }
                match run_install_process(installation_log_loop_sender_clone0, &cmd, LOG_FILE_PATH)
                {
                    Ok(_) => installation_log_status_loop_sender_clone0
                        .send_blocking(true)
                        .expect("channel needs to be open"),
                    Err(_) => installation_log_status_loop_sender_clone0
                        .send_blocking(false)
                        .expect("channel needs to be open"),
                }
            });
            main_carousel.scroll_to(&main_carousel.nth_page(7), true);
        }
    ));

    //

    page_done_action.connect_activate(clone!(
        #[strong]
        installation_summary_row_viewport_listbox,
        #[strong]
        language_selection_text_refcell,
        #[strong]
        keymap_selection_text_refcell,
        #[strong]
        timezone_selection_text_refcell,
        #[strong]
        partition_method_type_refcell,
        #[strong]
        partition_method_automatic_luks_enabled_refcell,
        #[strong]
        partition_method_automatic_target_fs_refcell,
        #[strong]
        partition_method_automatic_target_refcell,
        #[strong]
        partition_method_automatic_seperation_refcell,
        #[strong]
        partition_method_automatic_ratio_refcell,
        #[strong]
        partition_method_manual_fstab_entry_array_refcell,
        #[strong]
        partition_method_manual_luks_enabled_refcell,
        #[strong]
        partition_method_manual_crypttab_entry_array_refcell,
        move|_, action_arg|
            {
                let action_arg = String::from_utf8_lossy(action_arg.unwrap().data());
                if action_arg == "partitioning_done" {
                    while let Some(row) = installation_summary_row_viewport_listbox.last_child() {
                        installation_summary_row_viewport_listbox.remove(&row);
                    }
                    //
                    let partition_method_automatic_luks_enabled = partition_method_automatic_luks_enabled_refcell.borrow();
                    let partition_method_manual_luks_enabled = partition_method_manual_luks_enabled_refcell.borrow();
                    //
                    let install_confirm_detail_language = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_language_title"))
                        .subtitle(&language_selection_text_refcell.borrow().pretty_name)
                        .build();
                    install_confirm_detail_language.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_language);
                    //
                    let install_confirm_detail_keymap = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_keymap_title"))
                        .subtitle(&keymap_selection_text_refcell.borrow().pretty_name)
                        .build();
                    install_confirm_detail_keymap.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_keymap);
                    //
                    let install_confirm_detail_timezone = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_timezone_title"))
                        .subtitle(timezone_selection_text_refcell.borrow().to_string())
                        .build();
                    install_confirm_detail_timezone.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_timezone);
                    //
                    let install_confirm_detail_partition_method_type_subtitle = match partition_method_type_refcell.borrow().as_str() {
                        "automatic" => {
                            if *partition_method_automatic_luks_enabled {
                                t!("install_confirm_detail_partition_method_type_subtitle_automatic_luks").to_string()
                            } else {
                                t!("install_confirm_detail_partition_method_type_subtitle_automatic").to_string()
                            }
                        }
                        "manual" => {
                            if *partition_method_manual_luks_enabled {
                                t!("install_confirm_detail_partition_method_type_subtitle_manual_luks").to_string()
                            } else {
                                t!("install_confirm_detail_partition_method_type_subtitle_manual").to_string()
                            }
                        }
                        _ => panic!()
                    };
                    let install_confirm_detail_partition_method_type = adw::ActionRow::builder()
                        .title(t!("install_confirm_detail_partition_method_type_title"))
                        .subtitle(&install_confirm_detail_partition_method_type_subtitle)
                        .build();
                    install_confirm_detail_partition_method_type.add_css_class("property");
                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_type);
                    //
                    match partition_method_type_refcell.borrow().as_str() {
                        "automatic" => {
                            let install_confirm_detail_partition_method_automatic_target = adw::ActionRow::builder()
                                .title(t!("install_confirm_detail_partition_method_automatic_target_title"))
                                .subtitle(strfmt::strfmt(&t!("install_confirm_detail_partition_method_automatic_target_subtitle"), &std::collections::HashMap::from([("DISK_SIZE".to_string(), partition_method_automatic_target_refcell.borrow().block_size_pretty.as_str()), ("DISK_NAME".to_string(), partition_method_automatic_target_refcell.borrow().block_name.as_str())])).unwrap())
                                .build();
                            install_confirm_detail_partition_method_automatic_target.add_css_class("property");
                            installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_automatic_target);
                            //
                            let install_confirm_detail_partition_method_automatic_target_fs = adw::ActionRow::builder()
                                .title(t!("install_confirm_detail_partition_method_automatic_target_fs_title"))
                                .subtitle(partition_method_automatic_target_fs_refcell.borrow().to_uppercase())
                                .build();
                            install_confirm_detail_partition_method_automatic_target_fs.add_css_class("property");
                            installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_automatic_target_fs);
                            //
                            match partition_method_automatic_seperation_refcell.borrow().as_str() {
                                "subvol" => {
                                    let install_confirm_detail_partition_method_automatic_seperation = adw::ActionRow::builder()
                                        .title(t!("install_confirm_detail_partition_method_automatic_seperation_title"))
                                        .subtitle(t!("advanced_home_seperation_selection_checkbutton_subvol_label"))
                                        .build();
                                    install_confirm_detail_partition_method_automatic_seperation.add_css_class("property");
                                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_automatic_seperation);
                                }
                                "partition" => {
                                    let install_confirm_detail_partition_method_automatic_seperation = adw::ActionRow::builder()
                                        .title(t!("install_confirm_detail_partition_method_automatic_seperation_title"))
                                        .subtitle(t!("advanced_home_seperation_selection_checkbutton_partition_label"))
                                        .build();
                                    install_confirm_detail_partition_method_automatic_seperation.add_css_class("property");
                                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_automatic_seperation);
                                    //
                                    let usable_disk_space = partition_method_automatic_target_refcell.borrow().block_size - (MINIMUM_EFI_BYTE_SIZE + MINIMUM_BOOT_BYTE_SIZE);
                                    let root_part_size = *partition_method_automatic_ratio_refcell.borrow();
                                    let home_part_size = usable_disk_space - root_part_size;
                                    let root_part_percent = (root_part_size/usable_disk_space) * 100.0;
                                    let home_part_percent = (home_part_size/usable_disk_space) * 100.0;
                                    let install_confirm_detail_partition_method_automatic_ratio = adw::ActionRow::builder()
                                        .title(t!("install_confirm_detail_partition_method_automatic_ratio_title"))
                                        .subtitle(strfmt::strfmt(
                                                &t!("install_confirm_detail_partition_method_automatic_ratio_subtitle"),
                                                &std::collections::HashMap::from([
                                                    ("ROOT_PER".to_string(), (root_part_percent.round() as i64).to_string().as_str()),
                                                    ("ROOT_SIZE".to_string(), &pretty_bytes::converter::convert(root_part_size)),
                                                    ("HOME_PER".to_string(), (home_part_percent.round() as i64).to_string().as_str()),
                                                    ("HOME_SIZE".to_string(), &pretty_bytes::converter::convert(home_part_size))
                                                ])
                                        ).unwrap())
                                        .build();
                                    install_confirm_detail_partition_method_automatic_ratio.add_css_class("property");
                                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_automatic_ratio);
                                }
                                "none" => {
                                    let install_confirm_detail_partition_method_automatic_seperation = adw::ActionRow::builder()
                                        .title(t!("install_confirm_detail_partition_method_automatic_seperation_title"))
                                        .subtitle(t!("advanced_home_seperation_selection_checkbutton_none_label"))
                                        .build();
                                    install_confirm_detail_partition_method_automatic_seperation.add_css_class("property");
                                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_automatic_seperation);
                                }
                                _ => panic!()
                            }
                        }
                        "manual" => {
                            if *partition_method_manual_luks_enabled {
                                for crypttab_entry in partition_method_manual_crypttab_entry_array_refcell.borrow().iter() {
                                    let crypttab_entry_partition = &crypttab_entry.partition;
                                    let install_confirm_detail_partition_method_manual_crypttab_entry_subtitle = if crypttab_entry.password.is_some() {
                                        t!("install_confirm_detail_partition_method_manual_crypttab_entry_subtitle_auto")
                                    } else {
                                        t!("install_confirm_detail_partition_method_manual_crypttab_entry_subtitle_manual")
                                    };
                                    let install_confirm_detail_partition_method_manual_crypttab_entry = adw::ActionRow::builder()
                                        .title(t!("install_confirm_detail_partition_method_manual_crypttab_entry_title"))
                                        .subtitle(strfmt::strfmt(
                                            &install_confirm_detail_partition_method_manual_crypttab_entry_subtitle,
                                            &std::collections::HashMap::from([
                                                ("LUKS_NAME".to_string(), (crypttab_entry_partition).to_string().as_str()),
                                            ])
                                        ).unwrap())
                                        .build();
                                    install_confirm_detail_partition_method_manual_crypttab_entry.add_css_class("property");
                                    installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_manual_crypttab_entry);
                                }
                            }
                            for fstab_entry in partition_method_manual_fstab_entry_array_refcell.borrow().iter() {
                                let install_confirm_detail_partition_method_manual_fstab_entry = adw::ActionRow::builder()
                                .title(t!("install_confirm_detail_partition_method_manual_fstab_entry_title"))
                                .subtitle(strfmt::strfmt(
                                    &t!("install_confirm_detail_partition_method_manual_fstab_entry_subtitle"),
                                    &std::collections::HashMap::from([
                                        ("PART_NAME".to_string(), fstab_entry.partition.part_name.as_str()),
                                        ("PART_SIZE".to_string(), fstab_entry.partition.part_size_pretty.as_str()),
                                        ("PART_FS".to_string(), fstab_entry.partition.part_fs.as_str()),
                                        ("MOUNTPOINT".to_string(), fstab_entry.mountpoint.as_str()),
                                    ])
                                ).unwrap())
                                .build();
                            install_confirm_detail_partition_method_manual_fstab_entry.add_css_class("property");
                            installation_summary_row_viewport_listbox.append(&install_confirm_detail_partition_method_manual_fstab_entry);
                            }
                        }
                        _ => panic!()
                    }
                }
            }
        )
    );

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        installation_summary_page,
        #[weak]
        install_confirm_button,
        move |_, _| {
            installation_summary_page.set_page_title(t!("installation_summary_page_title"));
            installation_summary_page.set_page_subtitle(t!("installation_summary_page_subtitle"));
            installation_summary_page.set_back_tooltip_label(t!("back"));
            installation_summary_page.set_next_tooltip_label(t!("next"));
            //
            install_confirm_button.set_label(&t!("install_confirm_button_label"));
        }
    ));
    //

    installation_summary_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_installation_summary_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(5), true)
            }
        ),
    );

    main_carousel.append(&installation_summary_page);
}
