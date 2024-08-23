use crate::{
    build_ui::{BlockDevice, CrypttabEntry, FstabEntry, PikaKeymap, PikaLocale},
    config::{MINIMUM_BOOT_BYTE_SIZE, MINIMUM_EFI_BYTE_SIZE},
    installer_stack_page,
};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, fs, ops::Deref, path::Path, process::Command, rc::Rc};

pub const standard_installation_prog: &str = r###"#! /bin/bash
set -e

SOCKET_PATH="/tmp/pikainstall-status.sock"

PIKA_INSTALL_CHROOT_PATH={CHROOT_PATH}
PIKA_INSTALL_LOCALE="{LOCALE}.UTF-8"
PIKA_INSTALL_KEYMAP_BASE={KEYMAP_BASE}
PIKA_INSTALL_KEYMAP_VARIANT={KEYMAP_VARIANT}
PIKA_INSTALL_TIMEZONE={TIMEZONE}
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
    let script = strfmt::strfmt(
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
        ]),
    )
    .unwrap();

    let script2 = strfmt::strfmt(
        automatic_home_part_ext4_locked_installation_prog,
        &std::collections::HashMap::from([
            ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
            (
                "AUTO_LUKS_PASSWORD".to_string(),
                partition_method_automatic_luks_refcell.borrow().as_str(),
            ),
            (
                "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().as_str(),
            ),
        ]),
    )
    .unwrap();

    println!("{}", script2)
}
