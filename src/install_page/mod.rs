use crate::config::DISTRO_ICON;
use std::cell::RefCell;
// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;
use vte::prelude::*;
use vte::*;

use gettextrs::gettext;

use crate::done_page::done_page;

use std::process::Command;

use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::manual_partitioning::DriveMount;
use duct::*;
use serde::*;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize)]
struct CrypttabEntry {
    partition: String,
    password: String,
}

pub fn install_page(
    done_main_box: &gtk::Box,
    install_main_box: &gtk::Box,
    content_stack: &gtk::Stack,
    window: &adw::ApplicationWindow,
    manual_drive_mount_array: &Rc<RefCell<Vec<DriveMount>>>,
) {
    let mut _iter_count = 0;
    _iter_count = 0;
    let mut unlocked_array: Vec<String> = Default::default();
    manual_drive_mount_array
        .borrow_mut()
        .sort_by_key(|p| p.clone().mountpoint);
    for partitions in manual_drive_mount_array.borrow_mut().iter() {
        let new_crypt = if partitions.mountpoint != "/"
            && !unlocked_array.contains(&partitions.partition)
            && Command::new("sudo")
                .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
                .arg("has_encryption")
                .arg(&partitions.partition)
                .output()
                .expect("failed to execute process")
                .status
                .success()
        {
            let crypttab_password_listbox = gtk::ListBox::builder()
                .margin_top(10)
                .margin_bottom(10)
                .margin_start(10)
                .margin_end(10)
                .build();
            crypttab_password_listbox.add_css_class("boxed-list");
            let crypttab_password = adw::PasswordEntryRow::builder()
                .title(gettext("luks_password_for") + &partitions.partition)
                .build();
            crypttab_password.set_show_apply_button(true);
            crypttab_password_listbox.append(&crypttab_password);
            let crypttab_dialog = adw::MessageDialog::builder()
                .transient_for(window)
                .hide_on_close(true)
                .extra_child(&crypttab_password_listbox)
                .width_request(400)
                .height_request(200)
                .heading(
                    gettext("luks_how_should")
                        + &partitions.partition
                        + &gettext("be_added_crypttab"),
                )
                .build();
            crypttab_dialog.add_response("crypttab_dialog_boot", &gettext("unlock_boot_manually"));
            crypttab_dialog.add_response("crypttab_dialog_auto", &gettext("unlock_boot_manual"));
            crypttab_dialog.set_response_enabled("crypttab_dialog_auto", false);
            crypttab_password.connect_apply(clone!(@weak crypttab_password, @strong partitions, @weak crypttab_dialog => move |_| {
            let (luks_manual_password_sender, luks_manual_password_receiver) = async_channel::unbounded();
            let luks_manual_password_sender = luks_manual_password_sender.clone();
            let luks_password = crypttab_password.text().to_string();

            gio::spawn_blocking(clone!(@strong crypttab_password, @strong partitions  => move || {
                    let result = cmd!("sudo", "/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh", "test_luks_passwd", &partitions.partition, luks_password).run();
                    if result.is_ok() {
                        luks_manual_password_sender
                        .send_blocking(false)
                        .expect("The channel needs to be open.");
                    } else {
                        luks_manual_password_sender
                        .send_blocking(true)
                        .expect("The channel needs to be open.");
                    }
            }));
                let luks_manual_password_main_context = MainContext::default();
            // The main loop executes the asynchronous block
            luks_manual_password_main_context.spawn_local(clone!(@weak crypttab_dialog => async move {
                while let Ok(state) = luks_manual_password_receiver.recv().await {
                    crypttab_dialog.set_response_enabled("crypttab_dialog_auto", !state);
                }
            }));
            }));

            let partition_final = partitions.partition.clone();
            let partition_final2 = partitions.partition.clone();
            crypttab_dialog.choose(None::<&gio::Cancellable>, move |choice| {
                if choice == "crypttab_dialog_auto" {
                    let crypttab_entry = CrypttabEntry {
                        partition: partition_final2,
                        password: (&crypttab_password.text()).to_string(),
                    };
                    fs::write(
                        "/tmp/pika-installer-gtk4-target-manual-luks-p".to_owned()
                            + &_iter_count.to_string()
                            + ".json",
                        serde_json::to_string(&crypttab_entry).unwrap(),
                    )
                    .expect("Unable to write file");
                } else {
                    let crypttab_entry = CrypttabEntry {
                        partition: partition_final2,
                        password: (&"").to_string(),
                    };
                    fs::write(
                        "/tmp/pika-installer-gtk4-target-manual-luks-p".to_owned()
                            + &_iter_count.to_string()
                            + ".json",
                        serde_json::to_string(&crypttab_entry).unwrap(),
                    )
                    .expect("Unable to write file");
                }
            });
            partition_final
        } else {
            String::from("")
        };
        fs::write(
            "/tmp/pika-installer-gtk4-target-manual-p".to_owned()
                + &_iter_count.to_string()
                + ".json",
            serde_json::to_string(partitions).unwrap(),
        )
        .expect("Unable to write file");
        if !new_crypt.is_empty() {
            unlocked_array.push(new_crypt);
        }
        dbg!(&unlocked_array);
        _iter_count += 1;
    }

    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();

    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label(gettext("back"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);

    let install_nested_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::SlideLeftRight)
        .build();

    let install_confirm_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // the header box for the install page
    let install_confirm_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the install page
    let install_confirm_header_text = gtk::Label::builder()
        .label(gettext("sit_back_relax"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    install_confirm_header_text.add_css_class("header_sized_text");

    // the header icon for the install icon
    let install_confirm_header_icon = gtk::Spinner::builder()
        .halign(gtk::Align::Start)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();
    install_confirm_header_icon.start();

    // make install selection box for choosing installation or live media
    let install_confirm_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let install_confirm_details_boxed_list = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(256)
        .margin_end(256)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .build();
    install_confirm_details_boxed_list.add_css_class("boxed-list");

    let install_confirm_detail_language = adw::ActionRow::builder()
        .title(gettext("language_detail"))
        .subtitle(
            fs::read_to_string("/tmp/pika-installer-gtk4-lang.txt").expect("Unable to read file"),
        )
        .build();
    install_confirm_detail_language.add_css_class("property");

    let install_confirm_detail_timezone = adw::ActionRow::builder()
        .title(gettext("timezone_detail"))
        .subtitle(
            fs::read_to_string("/tmp/pika-installer-gtk4-timezone.txt")
                .expect("Unable to read file"),
        )
        .build();
    install_confirm_detail_timezone.add_css_class("property");

    let install_confirm_detail_keyboard = adw::ActionRow::builder()
        .title(gettext("keyboard_detail"))
        .subtitle(
            fs::read_to_string("/tmp/pika-installer-gtk4-keyboard.txt")
                .expect("Unable to read file"),
        )
        .build();
    install_confirm_detail_keyboard.add_css_class("property");

    if Path::new("/tmp/pika-installer-gtk4-target-manual.txt").exists() {
        //install_confirm_detail_target.set_subtitle(&fs::read_to_string("/tmp/pika-installer-gtk4-target-manual.txt").expect("Unable to read file"));
        install_confirm_details_boxed_list.append(&install_confirm_detail_language);
        install_confirm_details_boxed_list.append(&install_confirm_detail_timezone);
        install_confirm_details_boxed_list.append(&install_confirm_detail_keyboard);
        for partitions in manual_drive_mount_array.borrow_mut().iter() {
            let confirm_row = adw::ActionRow::builder()
                .title(
                    "/dev/".to_owned()
                        + &partitions.partition
                        + &gettext("mounted_on_detail")
                        + &partitions.mountpoint,
                )
                .build();
            install_confirm_details_boxed_list.append(&confirm_row);
        }
    } else {
        let install_confirm_detail_target = adw::ActionRow::builder()
            .title(gettext("install_target_detail"))
            .build();
        install_confirm_detail_target.set_subtitle(
            &fs::read_to_string("/tmp/pika-installer-gtk4-target-auto.txt")
                .expect("Unable to read file"),
        );
        install_confirm_detail_target.add_css_class("property");
        let target_block_device = &fs::read_to_string("/tmp/pika-installer-gtk4-target-auto.txt")
            .expect("Unable to read file");
        let target_size_cli = Command::new("sudo")
            .arg("/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh")
            .arg("get_block_size")
            .arg(target_block_device)
            .output()
            .expect("failed to execute process");
        let target_size = String::from_utf8(target_size_cli.stdout)
            .expect("Failed to create float")
            .trim()
            .parse::<f64>()
            .unwrap();
        let mut _target_p3_size = 0.0;
        if (target_size * 40.0) / 100.0 >= 150000000000.0 {
            _target_p3_size = 150000000000.0;
        } else if (target_size * 40.0) / 100.0 <= 36507222016.0 {
            _target_p3_size = 36507222016.0
        } else {
            _target_p3_size = (target_size * 40.0) / 100.0;
        }
        let target_p4_size = target_size - (_target_p3_size + 1536.0);
        if Path::new("/tmp/pika-installer-p3-size.txt").exists() {
            fs::remove_file("/tmp/pika-installer-p3-size.txt")
                .expect("Bad permissions on /tmp/pika-installer-p3-size.txt");
        }
        let target_p3_sector = _target_p3_size + 1537.0;
        fs::write(
            "/tmp/pika-installer-p3-size.txt",
            target_p3_sector.to_string(),
        )
        .expect("Unable to write file");
        let mut _p1_row_text = String::new();
        let mut _p2_row_text = String::new();
        let mut _p3_row_text = String::new();
        let mut _p4_row_text = String::new();
        if target_block_device.contains("nvme") {
            _p1_row_text =
                "512 MB ".to_owned() + target_block_device + "p1" + " as fat32" + " on /boot/efi";
            _p2_row_text =
                "1 GB ".to_owned() + target_block_device + "p2" + " as ext4" + " on /boot";
            _p3_row_text = pretty_bytes::converter::convert(_target_p3_size)
                + " "
                + target_block_device
                + "p3"
                + " as btrfs"
                + " on /";
            _p4_row_text = pretty_bytes::converter::convert(target_p4_size)
                + " "
                + target_block_device
                + "p4"
                + " as btrfs"
                + " on /home";
        } else {
            _p1_row_text =
                "512 MB ".to_owned() + target_block_device + "1" + " as fat32" + " on /boot/efi";
            _p2_row_text =
                "1 GB ".to_owned() + target_block_device + "2" + " as ext4" + " on /boot";
            _p3_row_text = pretty_bytes::converter::convert(_target_p3_size)
                + " "
                + target_block_device
                + "3"
                + " as btrfs"
                + " on /";
            _p4_row_text = pretty_bytes::converter::convert(target_p4_size)
                + " "
                + target_block_device
                + "4"
                + " as btrfs"
                + " on /home";
        }
        let install_confirm_p1 = adw::ActionRow::builder()
            .title(_p1_row_text.clone())
            .build();
        let install_confirm_p2 = adw::ActionRow::builder()
            .title(_p2_row_text.clone())
            .build();
        let install_confirm_p3 = adw::ActionRow::builder()
            .title(_p3_row_text.clone())
            .build();
        let install_confirm_p4 = adw::ActionRow::builder()
            .title(_p4_row_text.clone())
            .build();
        // / install_confirm_selection_box appends
        //// add live and install media button to install page selections
        install_confirm_details_boxed_list.append(&install_confirm_detail_language);
        install_confirm_details_boxed_list.append(&install_confirm_detail_timezone);
        install_confirm_details_boxed_list.append(&install_confirm_detail_keyboard);
        install_confirm_details_boxed_list.append(&install_confirm_detail_target);
        install_confirm_details_boxed_list.append(&install_confirm_p1);
        install_confirm_details_boxed_list.append(&install_confirm_p2);
        install_confirm_details_boxed_list.append(&install_confirm_p3);
        install_confirm_details_boxed_list.append(&install_confirm_p4);
    }

    let install_confirm_button = gtk::Button::builder()
        .label(gettext("confirm_install_pika"))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    install_confirm_button.add_css_class("destructive-action");

    // / install_confirm_header_box appends
    //// Add the install page header text and icon
    install_confirm_header_box.append(&install_confirm_header_text);
    install_confirm_header_box.append(&install_confirm_header_icon);

    // / install_confirm_box appends
    //// Add the install header to install main box
    install_confirm_box.append(&install_confirm_header_box);
    //// Add the install selection/page content box to install main box
    install_confirm_box.append(&install_confirm_selection_box);

    // Start Appending widgets to boxes

    //
    install_confirm_selection_box.append(&install_confirm_details_boxed_list);
    install_confirm_selection_box.append(&install_confirm_button);

    // / install_confirm_header_box appends
    //// Add the install page header text and icon
    install_confirm_header_box.append(&install_confirm_header_text);
    install_confirm_header_box.append(&install_confirm_header_icon);

    // / install_confirm_box appends
    //// Add the install header to install main box
    install_confirm_box.append(&install_confirm_header_box);
    //// Add the install selection/page content box to install main box
    install_confirm_box.append(&install_confirm_selection_box);

    install_main_box.append(&install_nested_stack);

    install_confirm_box.append(&bottom_box);

    let install_progress_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let install_progress_log_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::SlideUpDown)
        .build();

    let install_progress_log_terminal = vte::Terminal::builder()
        .vexpand(true)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .input_enabled(false)
        .build();

    let placeholder_icon = gtk::Image::builder()
        .icon_name(DISTRO_ICON)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .pixel_size(512)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let progress_bar_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_start(15)
        .margin_end(15)
        .build();

    let install_progress_bar = gtk::ProgressBar::builder()
        .hexpand(true)
        .margin_start(15)
        .margin_end(15)
        .margin_top(15)
        .margin_bottom(15)
        .show_text(true)
        .build();

    let progress_log_button_content = adw::ButtonContent::builder()
        .label(gettext("view_logs"))
        .icon_name("terminal")
        .build();

    let progress_log_button = gtk::Button::builder()
        .child(&progress_log_button_content)
        .margin_start(15)
        .margin_end(15)
        .margin_top(15)
        .margin_bottom(15)
        .build();

    progress_bar_box.append(&install_progress_bar);
    progress_bar_box.append(&progress_log_button);

    install_progress_log_stack.add_titled(
        &placeholder_icon,
        Some("slideshow_page"),
        "slideshow_page",
    );
    install_progress_log_stack.add_titled(
        &install_progress_log_terminal,
        Some("terminal_log_page"),
        "terminal_log_page",
    );

    install_progress_box.append(&install_progress_log_stack);
    install_progress_box.append(&progress_bar_box);

    install_nested_stack.add_titled(&install_confirm_box, Some("confirm_page"), "confirm_page");
    install_nested_stack.add_titled(
        &install_progress_box,
        Some("progress_page"),
        "progress_page",
    );

    //

    //

    install_confirm_button.connect_clicked(clone!(@weak install_nested_stack, @weak install_progress_log_terminal, @weak install_progress_bar, @weak done_main_box, @weak content_stack, @weak window => move |_| {
        install_nested_stack.set_visible_child_name("progress_page");
        begin_install(&install_progress_log_terminal, &install_progress_bar, &done_main_box, &content_stack, &window);
    }));

    progress_log_button.connect_clicked(clone!(@weak install_progress_log_stack => move |_| {
        if install_progress_log_stack.visible_child_name() == Some(GString::from_string_unchecked("slideshow_page".into())) {
            install_progress_log_stack.set_visible_child_name("terminal_log_page");
        } else {
            install_progress_log_stack.set_visible_child_name("slideshow_page");
        }
    }));

    bottom_back_button.connect_clicked(
        clone!(@weak content_stack, @weak install_main_box, @weak install_nested_stack => move |_| {
            content_stack.set_visible_child_name("partitioning_page");
            install_main_box.remove(&install_nested_stack)
        }),
    );
}

fn begin_install(
    install_progress_log_terminal: &vte::Terminal,
    install_progress_bar: &gtk::ProgressBar,
    done_main_box: &gtk::Box,
    content_stack: &gtk::Stack,
    window: &adw::ApplicationWindow,
) {
    // SPAWN TERMINAL WITH PIKAINSTALL PROCESS
    install_progress_log_terminal.spawn_async(
        PtyFlags::DEFAULT,
        Some(""),
        &["/usr/lib/pika/pika-installer-gtk4/scripts/begin-install.sh"],
        &[],
        SpawnFlags::DEFAULT,
        || {},
        -1,
        None::<&gio::Cancellable>,
        move |result| match result {
            Ok(_) => {
                eprintln!("could not spawn terminal")
            }
            Err(err) => {
                eprintln!("could not spawn terminal: {}", err);
            }
        },
    );
    // wait till /tmp/pika-installer-gtk4-status-parting.txt to change progressbar
    let (parting_status_sender, parting_status_receiver) = async_channel::unbounded();
    let parting_status_sender = parting_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let parting_status = true;
        while parting_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-parting.txt").exists() == true {
                parting_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let parting_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    parting_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(parting_status_state) = parting_status_receiver.recv().await {

            if parting_status_state == true {
                println!("Installation status: Parting");
                install_progress_bar.set_fraction(0.20);
                install_progress_bar.set_text(Some(&gettext("parting_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-image.txt to change progressbar
    let (image_status_sender, image_status_receiver) = async_channel::unbounded();
    let image_status_sender = image_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let image_status = true;
        while image_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-image.txt").exists() == true {
                image_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let image_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    image_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(image_status_state) = image_status_receiver.recv().await {

            if image_status_state == true {
                println!("Installation status: Imaging");
                install_progress_bar.set_fraction(0.60);
                install_progress_bar.set_text(Some(&gettext("image_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-flag1.txt to change progressbar
    let (flag1_status_sender, flag1_status_receiver) = async_channel::unbounded();
    let flag1_status_sender = flag1_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let flag1_status = true;
        while flag1_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-flag1.txt").exists() == true {
                flag1_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let flag1_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    flag1_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(flag1_status_state) = flag1_status_receiver.recv().await {

            if flag1_status_state == true {
                println!("Installation status: Flag1");
                install_progress_bar.set_fraction(0.65);
                install_progress_bar.set_text(Some(&gettext("flag1_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-flag2.txt to change progressbar
    let (flag2_status_sender, flag2_status_receiver) = async_channel::unbounded();
    let flag2_status_sender = flag2_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let flag2_status = true;
        while flag2_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-flag2.txt").exists() == true {
                flag2_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let flag2_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    flag2_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(flag2_status_state) = flag2_status_receiver.recv().await {

            if flag2_status_state == true {
                println!("Installation status: Flag2");
                install_progress_bar.set_fraction(0.70);
                install_progress_bar.set_text(Some(&gettext("flag2_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-crypt.txt to change progressbar
    let (crypt_status_sender, crypt_status_receiver) = async_channel::unbounded();
    let crypt_status_sender = crypt_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let crypt_status = true;
        while crypt_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-crypt.txt").exists() == true {
                crypt_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let crypt_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    crypt_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(crypt_status_state) = crypt_status_receiver.recv().await {

            if crypt_status_state == true {
                println!("Installation status: Crypttab");
                install_progress_bar.set_fraction(0.75);
                install_progress_bar.set_text(Some(&gettext("crypt_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-lang.txt to change progressbar
    let (lang_status_sender, lang_status_receiver) = async_channel::unbounded();
    let lang_status_sender = lang_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let lang_status = true;
        while lang_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-lang.txt").exists() == true {
                lang_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let lang_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    lang_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(lang_status_state) = lang_status_receiver.recv().await {

            if lang_status_state == true {
                println!("Installation status: Language");
                install_progress_bar.set_fraction(0.80);
                install_progress_bar.set_text(Some(&gettext("lang_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-boot.txt to change progressbar
    let (boot_status_sender, boot_status_receiver) = async_channel::unbounded();
    let boot_status_sender = boot_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let boot_status = true;
        while boot_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-boot.txt").exists() == true {
                boot_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let boot_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    boot_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(boot_status_state) = boot_status_receiver.recv().await {

            if boot_status_state == true {
                println!("Installation status: Bootloader");
                install_progress_bar.set_fraction(0.85);
                install_progress_bar.set_text(Some(&gettext("boot_status_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-status-post.txt to change progressbar
    let (post_status_sender, post_status_receiver) = async_channel::unbounded();
    let post_status_sender = post_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let post_status = true;
        while post_status == true {
            if Path::new("/tmp/pika-installer-gtk4-status-post.txt").exists() == true {
                post_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let post_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    post_status_main_context.spawn_local(clone!(@weak install_progress_bar => async move {
        while let Ok(post_status_state) = post_status_receiver.recv().await {

            if post_status_state == true {
                println!("Installation status: Post Install");
                install_progress_bar.set_fraction(0.90);
                install_progress_bar.set_text(Some(&gettext("post_status_text")));
            }
        }
    }));
    // wait till /tmp/pika-installer-gtk4-successful.txt to change progressbar
    let (done_status_sender, done_status_receiver) = async_channel::unbounded();
    let done_status_sender = done_status_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        let done_status = true;
        while done_status == true {
            if Path::new("/tmp/pika-installer-gtk4-successful.txt").exists() == true
                || Path::new("/tmp/pika-installer-gtk4-fail.txt").exists() == true
            {
                done_status_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
                break;
            }
        }
    });
    let done_status_main_context = MainContext::default();
    // The main loop executes the asynchronous block
    done_status_main_context.spawn_local(
        clone!(@weak done_main_box, @weak content_stack, @weak window => async move {
            while let Ok(done_status_state) = done_status_receiver.recv().await {
                if done_status_state == true {
                    println!("Installation status: Done");
                    done_page(&done_main_box, &window);
                    content_stack.set_visible_child_name("done_page");
                }
            }
        }),
    );
}
