use crate::{
    build_ui::{BlockDevice, CrypttabEntry, FstabEntry, PikaKeymap, PikaLocale},
    config::{MINIMUM_BOOT_BYTE_SIZE, MINIMUM_EFI_BYTE_SIZE},
    installer_stack_page,
};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, fs, ops::Deref, path::Path, process::Command, rc::Rc};

mod auto_basic;
mod auto_btrfs;

pub const standard_installation_prog: &str = r###"#! /bin/bash
set -e

SOCKET_PATH="/tmp/pikainstall-status.sock"

PIKA_INSTALL_CHROOT_PATH='{CHROOT_PATH}'
PIKA_INSTALL_LOCALE='{LOCALE}.UTF-8'
PIKA_INSTALL_KEYMAP_BASE='{KEYMAP_BASE}'
PIKA_INSTALL_KEYMAP_VARIANT='{KEYMAP_VARIANT}'
PIKA_INSTALL_TIMEZONE='{TIMEZONE}'
"###;

pub fn create_installation_script(
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
    let mut final_script = String::new();
    
    let standard_installation_format = strfmt::strfmt(
        standard_installation_prog,
        &std::collections::HashMap::from([
            ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
            (
                "LOCALE".to_string(),
                language_selection_text_refcell.borrow().name.as_str(),
            ),
            (
                "KEYMAP_BASE".to_string(),
                keymap_selection_text_refcell.borrow().name.as_str(),
            ),
            (
                "KEYMAP_VARIANT".to_string(),
                match &keymap_selection_text_refcell.borrow().variant {
                    Some(t) => t.as_str(),
                    None => "",
                },
            ),
            (
                "TIMEZONE".to_string(),
                timezone_selection_text_refcell.borrow().as_str(),
            ),
        ]),
    )
    .unwrap();

    final_script.push_str(&standard_installation_format);

    match &*partition_method_type_refcell.borrow().as_str() {
        "automatic" => {
            let is_encrypted = *partition_method_automatic_luks_enabled_refcell.borrow();
            //
            let automatic_standard_installation_format = strfmt::strfmt(
                auto_basic::automatic_standard_installation_prog,
                &std::collections::HashMap::from([
                    (
                        "AUTO_INSTALL_TARGET_DISK".to_string(),
                        partition_method_automatic_target_refcell.borrow().block_name.as_str(),
                    ),
                ]),
            )
            .unwrap();

            final_script.push_str(&automatic_standard_installation_format);

            //
            match &*partition_method_automatic_target_fs_refcell.borrow().as_str().to_lowercase() {
                "btrfs" => {
                    match &*partition_method_automatic_seperation_refcell.borrow().as_str() {
                        "subvol" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::automatic_home_subvol_btrfs_locked_installation_prog,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::automatic_home_subvol_btrfs_open_installation_prog,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        "partition" => {}
                        "none" => {}
                        _ => panic!()
                    }
                }
                "ext4" => {

                }
                "xfs" => {

                }
                _ => panic!()
            }
            //
            if is_encrypted {
                final_script.push_str(auto_basic::automatic_locked_part_pikainstall_prog);
            } else {
                final_script.push_str(auto_basic::automatic_open_part_pikainstall_prog);
            }
        }
        "manual" => {}
        _ => panic!()
    }

    println!("{}", final_script)
}
